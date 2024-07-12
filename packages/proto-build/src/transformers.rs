use std::collections::HashMap;
use std::path::Path;

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use proc_macro2::{Group, TokenStream as TokenStream2, TokenTree};
use prost_types::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorSet, ServiceDescriptorProto,
};
use quote::{format_ident, quote};
use regex::Regex;
use syn::ItemEnum;
use syn::ItemMod;
use syn::{parse_quote, Attribute, Fields, Ident, Item, ItemStruct, Type};

/// Regex substitutions to apply to the prost-generated output
pub const REPLACEMENTS: &[(&str, &str)] = &[
    // Feature-gate gRPC client modules
    (
        "/// Generated client implementations.",
        "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    ),
    // Feature-gate gRPC client impls which use `tonic::transport`
    (
        "impl (.+)Client<tonic::transport::Channel>",
        "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl ${1}Client<tonic::transport::Channel>",
    ),
];

pub fn add_derive_eq(mut attr: Attribute) -> Attribute {
    // find derive attribute
    if attr.path.is_ident("derive") {
        attr.tokens = attr
            .tokens
            .into_iter()
            .map(|token_tree| {
                match token_tree {
                    // with group token stream, which is `#[derive( ... )]`
                    TokenTree::Group(group) => {
                        let has_ident = |ident_str: &str| {
                            group.stream().into_iter().any(|token| match token {
                                TokenTree::Ident(ident) => ident == format_ident!("{}", ident_str),
                                _ => false,
                            })
                        };

                        // if does not have both PartialEq and Eq
                        let stream = if !(has_ident("PartialEq") && has_ident("Eq")) {
                            // construct new token stream
                            group
                                .stream()
                                .into_iter()
                                .flat_map(|token| {
                                    match token {
                                        // if there exist `PartialEq` in derive attr
                                        TokenTree::Ident(ident) => {
                                            if ident == format_ident!("PartialEq") {
                                                // expand token stream in group with `#[derive( ..., PartialEq, ... )]` to ``#[derive( ..., PartialEq, Eq, ... )]``
                                                let expanded_token_stream: TokenStream2 =
                                                    syn::parse_quote!(PartialEq, Eq);
                                                expanded_token_stream.into_iter().collect()
                                            } else {
                                                vec![TokenTree::Ident(ident)]
                                            }
                                        }
                                        tt => vec![tt],
                                    }
                                })
                                .collect()
                        } else {
                            group.stream()
                        };

                        TokenTree::Group(Group::new(group.delimiter(), stream))
                    }
                    _ => token_tree,
                }
            })
            .collect();
        attr
    } else {
        attr
    }
}

pub fn add_derive_eq_struct(s: &ItemStruct) -> ItemStruct {
    let mut item_struct = s.clone();
    item_struct.attrs = item_struct.attrs.into_iter().map(add_derive_eq).collect();

    item_struct
}

pub fn add_derive_eq_enum(s: &ItemEnum) -> ItemEnum {
    let mut item_enum = s.clone();
    item_enum.attrs = item_enum.attrs.into_iter().map(add_derive_eq).collect();

    item_enum
}

pub fn append_attrs_struct(
    src: &Path,
    s: &ItemStruct,
    descriptor: &FileDescriptorSet,
) -> ItemStruct {
    let mut s = s.clone();
    let query_services = extract_query_services(descriptor);
    let type_url = get_type_url(src, &s.ident, descriptor);

    let deprecated = get_deprecation(src, &s.ident, descriptor);

    s.attrs.append(&mut vec![
        syn::parse_quote! { #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)] },
        syn::parse_quote! { #[proto_message(type_url = #type_url)] },
    ]);

    if let Some(attr) = get_query_attr(src, &s.ident, &query_services) {
        s.attrs.append(&mut vec![attr])
    }

    if deprecated {
        s.attrs
            .append(&mut vec![syn::parse_quote! { #[deprecated] }]);
    }

    s
}

pub fn append_attrs_enum(src: &Path, e: &ItemEnum, descriptor: &FileDescriptorSet) -> ItemEnum {
    let mut e = e.clone();
    let deprecated = get_deprecation(src, &e.ident, descriptor);

    e.attrs.append(&mut vec![
        syn::parse_quote! { #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)] },
    ]);

    if deprecated {
        e.attrs
            .append(&mut vec![syn::parse_quote! { #[deprecated] }]);
    }

    e
}

pub fn allow_serde_int_as_str(s: ItemStruct) -> ItemStruct {
    let int_types = vec![
        parse_quote!(i8),
        parse_quote!(i16),
        // parse_quote!(i32), -- this is not included since it could be either str or enum type
        parse_quote!(i64),
        parse_quote!(i128),
        parse_quote!(isize),
        parse_quote!(u8),
        parse_quote!(u16),
        parse_quote!(u32),
        parse_quote!(u64),
        parse_quote!(u128),
        parse_quote!(usize),
    ];

    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            if int_types.contains(&field.ty) {
                let from_str: syn::Attribute = parse_quote! {
                    #[serde(
                        serialize_with = "crate::serde::as_str::serialize",
                        deserialize_with = "crate::serde::as_str::deserialize"
                    )]
                };
                field.attrs.append(&mut vec![from_str]);
                field
            } else {
                field
            }
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

pub fn allow_serde_vec_int_as_vec_str(s: ItemStruct) -> ItemStruct {
    let vec_int_types = vec![
        parse_quote!(::prost::alloc::vec::Vec<i8>),
        parse_quote!(::prost::alloc::vec::Vec<i16>),
        parse_quote!(::prost::alloc::vec::Vec<i32>),
        parse_quote!(::prost::alloc::vec::Vec<i64>),
        parse_quote!(::prost::alloc::vec::Vec<i128>),
        parse_quote!(::prost::alloc::vec::Vec<isize>),
        // parse_quote!(::prost::alloc::vec::Vec<u8>), -- this is not included because it is used for bytes and has it's own rule
        parse_quote!(::prost::alloc::vec::Vec<u16>),
        parse_quote!(::prost::alloc::vec::Vec<u32>),
        parse_quote!(::prost::alloc::vec::Vec<u64>),
        parse_quote!(::prost::alloc::vec::Vec<u128>),
        parse_quote!(::prost::alloc::vec::Vec<usize>),
    ];

    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            if vec_int_types.contains(&field.ty) {
                let from_str: syn::Attribute = parse_quote! {
                    #[serde(
                        serialize_with = "crate::serde::as_str_vec::serialize",
                        deserialize_with = "crate::serde::as_str_vec::deserialize"
                    )]
                };
                field.attrs.append(&mut vec![from_str]);
                field
            } else {
                field
            }
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

/// Adds `crate::serde::as_str_bytes` serde attribute to specific Metadata fields which are a Metadata Address
/// or adds `crate::serde::as_base64_encoded_string` serde attribute to `::prost::alloc::vec::Vec<u8>` field types
pub fn allow_serde_vec_u8_as_base64_encoded_string_or_string_bytes(s: ItemStruct) -> ItemStruct {
    // These fields are string of bytes (MetadataAddress type), not base64 strings
    let str_as_byte_fields = [
        "context",
        "contract_spec_id",
        "contract_spec_id_prefix",
        "contract_spec_id_contract_spec_uuid",
        "contract_specification_id",
        "record_id",
        "record_id_hashed_name",
        "record_id_prefix",
        "record_id_scope_uuid",
        "record_spec_id",
        "record_spec_id_contract_spec_uuid",
        "record_spec_id_hashed_name",
        "record_spec_id_prefix",
        "scope_spec_id",
        "scope_spec_id_prefix",
        "scope_spec_id_scope_spec_uuid",
        "scope_specification_id",
        "scope_id",
        "scope_id_prefix",
        "scope_id_scope_uuid",
        "session_id",
        "session_id_prefix",
        "session_id_scope_uuid",
        "specification_id",
    ];

    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            if field.ty == parse_quote!(::prost::alloc::vec::Vec<u8>) {
                let from_str: syn::Attribute = if field
                    .clone()
                    .ident
                    .is_some_and(|x| str_as_byte_fields.contains(&&***&&x.to_string()))
                {
                    parse_quote! {
                        #[serde(
                            serialize_with = "crate::serde::as_str_bytes::serialize",
                            deserialize_with = "crate::serde::as_str_bytes::deserialize"
                        )]
                    }
                } else {
                    parse_quote! {
                        #[serde(
                            serialize_with = "crate::serde::as_base64_encoded_string::serialize",
                            deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
                        )]
                    }
                };

                field.attrs.append(&mut vec![from_str]);
                field
            } else {
                field
            }
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

/// Adds `crate::serde::as_str_bytes_vec` serde attribute to specific Metadata fields which
/// are a vector of Metadata Addresses
pub fn allow_serde_vec_vec_u8_as_vec_string_bytes(s: ItemStruct) -> ItemStruct {
    // These fields are string of bytes (MetadataAddress type), not base64 strings
    let str_as_byte_fields = ["contract_spec_ids", "scope_ids"];

    let fields_vec = s
        .fields
        .clone()
        .into_iter()
        .map(|mut field| {
            if field.ty == parse_quote!(::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>) {
                if field
                    .clone()
                    .ident
                    .is_some_and(|x| str_as_byte_fields.contains(&&***&&x.to_string()))
                {
                    field.attrs.append(&mut vec![parse_quote! {
                        #[serde(
                            serialize_with = "crate::serde::as_str_bytes_vec::serialize",
                            deserialize_with = "crate::serde::as_str_bytes_vec::deserialize"
                        )]
                    }]);
                };

                field
            } else {
                field
            }
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

pub fn make_next_key_optional(mut s: ItemStruct) -> ItemStruct {
    if s.ident == "PageResponse" {
        if let Fields::Named(ref mut fields_named) = s.fields {
            for field in fields_named.named.iter_mut() {
                if let Some(ident) = &field.ident {
                    if ident == "next_key" {
                        field.ty =
                            parse_quote!(::core::option::Option<::prost::alloc::vec::Vec<u8>>);
                        for attr in field.attrs.iter_mut() {
                            if attr.path.is_ident("prost") {
                                *attr = parse_quote! {
                                    #[prost(bytes = "vec", optional, tag = "1")]
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    s
}

pub fn allow_serde_option_vec_u8_as_base64_encoded_string(s: syn::ItemStruct) -> syn::ItemStruct {
    let fields_vec = s.fields
        .clone()
        .into_iter()
        .map(|mut field| {
            if let syn::Type::Path(type_path) = &field.ty {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Option" {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(arg) = args.args.first() {
                                if let syn::GenericArgument::Type(inner_ty) = arg {
                                    if let syn::Type::Path(inner_path) = inner_ty {
                                        if let Some(inner_segment) = inner_path.path.segments.last() {
                                            if inner_segment.ident == "Vec" {
                                                let from_str: syn::Attribute = parse_quote! {
                                                    #[serde(
                                                        serialize_with = "crate::serde::as_option_base64_encoded_string::serialize",
                                                        deserialize_with = "crate::serde::as_option_base64_encoded_string::deserialize"
                                                    )]
                                                };
                                                field.attrs.push(from_str);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            field
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

/// Adds `crate::serde::enum_as_i32` serde attribute to specific Metadata fields which
/// are an enumerated type as an i32, or adds `crate::serde::as_str` serde attribute like `allow_serde_int_as_str`
pub fn allow_serde_int_as_str_or_enum_as_i32(s: syn::ItemStruct) -> syn::ItemStruct {
    let fields_vec = s
        .fields
        .into_iter()
        .map(|mut field| {
            let mut add_serde_attrs = false;
            for attr in &field.attrs {
                if attr.path.is_ident("prost") {
                    if let Ok(meta) = attr.parse_meta() {
                        if let syn::Meta::List(meta_list) = meta {
                            for nested_meta in meta_list.nested.iter() {
                                if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) =
                                    nested_meta
                                {
                                    if name_value.path.is_ident("enumeration") {
                                        add_serde_attrs = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if add_serde_attrs {
                field.attrs.push(parse_quote! {
                    #[serde(
                        serialize_with = "crate::serde::enum_as_i32::serialize",
                        deserialize_with = "crate::serde::enum_as_i32::deserialize"
                    )]
                });
            } else {
                field.attrs.push(parse_quote! {
                    #[serde(
                        serialize_with = "crate::serde::as_str::serialize",
                        deserialize_with = "crate::serde::as_str::deserialize"
                    )]
                });
            }

            field
        })
        .collect::<Vec<syn::Field>>();

    let fields_named: syn::FieldsNamed = parse_quote! {
        { #(#fields_vec,)* }
    };
    let fields = syn::Fields::Named(fields_named);

    syn::ItemStruct { fields, ..s }
}

// ====== helpers ======

fn get_query_attr(
    src: &Path,
    ident: &Ident,
    query_services: &HashMap<String, ServiceDescriptorProto>,
) -> Option<Attribute> {
    let package = src.file_stem().unwrap().to_str().unwrap();
    let service = query_services.get(package);

    let method = service?.method.iter().find(|m| {
        let input_type = m.input_type.clone().unwrap();
        let input_type = input_type.split('.').last().unwrap();
        *ident == input_type.to_upper_camel_case()
    });

    let method_name = method?.name.clone().unwrap();
    let response_type = method?.output_type.clone().unwrap();
    let response_type = response_type.split('.').last().unwrap();
    let response_type = format_ident!("{}", response_type.to_upper_camel_case());

    let path = format!("/{}.Query/{}", package, method_name);
    Some(syn::parse_quote! { #[proto_query(path = #path, response_type = #response_type)] })
}

fn get_type_url(src: &Path, ident: &Ident, descriptor: &FileDescriptorSet) -> String {
    let type_path = src.file_stem().unwrap().to_str().unwrap();
    let init_path = "";

    let name: Option<String> = descriptor
        .file
        .clone()
        .into_iter()
        .filter(|f| f.package.to_owned().unwrap() == type_path)
        .flat_map(|f| {
            let target = ident.to_string();
            vec![
                extract_type_path_from_enum(&target, &f.enum_type, init_path),
                extract_type_path_from_descriptor(&target, &f.message_type, init_path),
            ]
        })
        .filter(|r| r.is_some())
        .take(1)
        .collect();

    format!("/{}.{}", type_path, name.unwrap())
}

fn get_deprecation(src: &Path, ident: &Ident, descriptor: &FileDescriptorSet) -> bool {
    let type_path = src.file_stem().unwrap().to_str().unwrap();

    let deprecation: Option<bool> = descriptor
        .file
        .clone()
        .into_iter()
        .filter(|f| f.package.to_owned().unwrap() == type_path)
        .flat_map(|f| {
            let target = ident.to_string();
            vec![
                extract_deprecation_from_enum(&target, &f.enum_type),
                extract_deprecation_from_descriptor(&target, &f.message_type),
            ]
        })
        .find(|r| r.is_some())
        .flatten();

    deprecation.unwrap_or(false)
}

fn extract_deprecation_from_descriptor(
    target: &str,
    message_type: &[DescriptorProto],
) -> Option<bool> {
    message_type.iter().find_map(|descriptor| {
        let message_name = descriptor.name.to_owned().unwrap();

        if message_name.to_upper_camel_case() == target {
            descriptor.clone().options?.deprecated
        } else if let Some(deprecated) =
            extract_deprecation_from_descriptor(target, &descriptor.nested_type)
        {
            Some(deprecated)
        } else {
            extract_deprecation_from_enum(target, &descriptor.enum_type)
        }
    })
}

fn extract_deprecation_from_enum(target: &str, enum_type: &[EnumDescriptorProto]) -> Option<bool> {
    enum_type
        .iter()
        .find(|e| e.name.to_owned().unwrap().to_upper_camel_case() == target)
        .and_then(|e| e.clone().options?.deprecated)
}

fn extract_type_path_from_descriptor(
    target: &str,
    message_type: &[DescriptorProto],
    path: &str,
) -> Option<String> {
    message_type.iter().find_map(|descriptor| {
        let message_name = descriptor.name.to_owned().unwrap();

        if message_name.to_upper_camel_case() == target {
            Some(append_type_path(path, &message_name))
        } else if let Some(message_name) = extract_type_path_from_descriptor(
            target,
            &descriptor.nested_type,
            &append_type_path(path, &message_name),
        ) {
            Some(message_name)
        } else {
            extract_type_path_from_enum(
                target,
                &descriptor.enum_type,
                &append_type_path(path, &message_name),
            )
        }
    })
}

fn extract_type_path_from_enum(
    target: &str,
    enum_type: &[EnumDescriptorProto],
    path: &str,
) -> Option<String> {
    enum_type
        .iter()
        .find(|e| e.name.to_owned().unwrap().to_upper_camel_case() == target)
        .map(|e| append_type_path(path, &e.name.to_owned().unwrap()))
}

pub fn extract_query_services(
    descriptor: &FileDescriptorSet,
) -> HashMap<String, ServiceDescriptorProto> {
    descriptor
        .clone()
        .file
        .into_iter()
        .filter_map(|f| {
            let service = f
                .service
                .into_iter()
                .find(|s| s.name == Some("Query".to_string()));

            if let Some(service) = service {
                Some((
                    f.package.expect("Missing package name in file descriptor"),
                    service,
                ))
            } else {
                None
            }
        })
        .collect()
}

fn append_type_path(path: &str, name: &str) -> String {
    if path.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", path, name)
    }
}

pub fn append_querier(
    items: Vec<Item>,
    src: &Path,
    nested_mod: bool,
    descriptor: &FileDescriptorSet,
) -> Vec<Item> {
    let package = src.file_stem().unwrap().to_str().unwrap();
    let re = Regex::new(r"([^.]*)(\.v\d+(beta\d+)?)?$").unwrap();

    let package_stem = re.captures(package).unwrap().get(1).unwrap().as_str();

    let querier_wrapper_ident = format_ident!("{}Querier", &package_stem.to_upper_camel_case());

    let query_services = extract_query_services(descriptor);
    let query_fns = query_services.get(package).map(|service| service.method.iter().map(|method_desc| {
        if nested_mod {
            return quote! {};
        }

        let deprecated = method_desc.clone().options.map(|opt| opt.deprecated.unwrap_or(false) ).unwrap_or(false);
        let deprecated_macro = if deprecated {
            quote!(#[deprecated])
        } else {
            quote!()
        };

        let method_desc = method_desc.clone();

        let name = format_ident!("{}", method_desc.name.unwrap().as_str().to_snake_case());
        let req_type = format_ident!("{}", method_desc.input_type.unwrap().split('.').last().unwrap().to_string().to_upper_camel_case());
        let res_type = format_ident!("{}", method_desc.output_type.unwrap().split('.').last().unwrap().to_string().to_upper_camel_case());

        let req_args = items.clone().into_iter()
            .find_map(|item| match item {
                Item::Struct(s) => {
                    if s.ident == req_type {
                        match s.fields {
                            Fields::Named(fields_named) => {
                                Some(fields_named.named)
                            }
                            _ => None
                        }
                    } else {
                        None
                    }
                }
                _ => None
            });

        if req_args.is_none() {
            return quote! {};
        }

        let arg_idents = req_args.clone().unwrap().into_iter().map(|arg| arg.ident.unwrap()).collect::<Vec<Ident>>();
        let arg_ty = req_args.unwrap().into_iter().map(|arg| arg.ty).collect::<Vec<Type>>();

        quote! {
          #deprecated_macro
          pub fn #name( &self, #(#arg_idents : #arg_ty),* ) -> Result<#res_type, cosmwasm_std::StdError> {
            #req_type { #(#arg_idents),* }.query(self.querier)
          }
        }
    }).collect::<Vec<TokenStream2>>());

    let querier = if let Some(query_fns) = query_fns {
        if !nested_mod {
            vec![
                parse_quote! {
                  pub struct #querier_wrapper_ident<'a, Q: cosmwasm_std::CustomQuery> {
                      querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
                  }
                },
                parse_quote! {
                  impl<'a, Q: cosmwasm_std::CustomQuery> #querier_wrapper_ident<'a, Q> {
                      pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
                    Self { querier }
                    }
                    #(#query_fns)*
                  }
                },
            ]
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    vec![items, querier].concat()
}

/// This is a hack to fix a clashing name in the stake_authorization module
pub fn fix_clashing_stake_authorization_validators(input: ItemMod) -> ItemMod {
    // do this only if the module is named "stake_authorization"
    if input.ident != "stake_authorization" {
        return input;
    }
    let new_name = Ident::new("Validators_", input.ident.span());
    let mut validators = None;
    let items = input.content.clone().unwrap().1;

    // Iterate over the items in the module and look for the Validators struct then rename it
    let items = items.into_iter().map(|mut item| {
        if let Item::Struct(ref mut s) = item {
            if s.ident == "Validators" {
                s.ident = new_name.clone();
                validators = Some(s.clone());
            }
        }
        item
    });

    // Update any references to the struct
    let items = items.into_iter().map(|mut item| {
        if let Item::Enum(ref mut e) = item {
            if e.ident == "Validators" {
                for v in e.variants.iter_mut() {
                    if let Fields::Unnamed(ref mut f) = v.fields {
                        if let Type::Path(ref mut p) = f.unnamed.first_mut().unwrap().ty {
                            if p.path.segments.first().unwrap().ident == "Validators" {
                                p.path.segments.first_mut().unwrap().ident = new_name.clone();
                            }
                        }
                    }
                }
            }
        }
        item
    });

    ItemMod {
        content: Some((input.content.unwrap().0, items.collect())),
        ..input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::ItemStruct;

    macro_rules! assert_ast_eq {
        ($left:ident, $right:ident) => {
            let left_fmt =
                prettyplease::unparse(&syn::parse_file(&quote! { #$left }.to_string()).unwrap());
            let right_fmt =
                prettyplease::unparse(&syn::parse_file(&quote! { #$right}.to_string()).unwrap());

            assert!(
                $left == $right,
                "Left is: \n\n{} \n\n but right is: \n\n{} \n\n",
                left_fmt,
                right_fmt
            );
        };
    }

    #[test]
    fn test_add_derive_eq_if_there_is_partial_eq() {
        let item_struct: ItemStruct = syn::parse_quote! {
            #[derive(PartialEq, Debug)]
            struct Hello {
                name: String
            }
        };

        let result = add_derive_eq_struct(&item_struct);
        let expected: ItemStruct = syn::parse_quote! {
            #[derive(PartialEq, Eq, Debug)]
            struct Hello {
                name: String
            }
        };

        assert_ast_eq!(result, expected);
    }

    #[test]
    fn test_add_derive_eq_does_not_add_if_there_is_no_partial_eq() {
        let item_struct: ItemStruct = syn::parse_quote! {
            #[derive(Debug)]
            struct Hello {
                name: String
            }
        };

        let result = add_derive_eq_struct(&item_struct);

        assert_ast_eq!(item_struct, result);
    }

    #[test]
    fn test_add_derive_eq_does_not_add_if_there_is_partial_eq_and_eq() {
        let item_struct: ItemStruct = syn::parse_quote! {
            #[derive(PartialEq, Eq, Debug)]
            struct Hello {
                name: String
            }
        };

        let result = add_derive_eq_struct(&item_struct);

        let expected: ItemStruct = syn::parse_quote! {
            #[derive(PartialEq, Eq, Debug)]
            struct Hello {
                name: String
            }
        };

        assert_ast_eq!(result, expected);
    }

    #[test]
    fn test_make_next_key_optional() {
        let input: ItemStruct = parse_quote! {
            pub struct PageResponse {
                #[prost(bytes = "vec", tag = "1")]
                pub next_key: ::prost::alloc::vec::Vec<u8>,
            }
        };

        let result = make_next_key_optional(input);

        let expected: ItemStruct = parse_quote! {
            pub struct PageResponse {
                #[prost(bytes = "vec", optional, tag = "1")]
                pub next_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            }
        };

        assert_ast_eq!(result, expected);
    }

    #[test]
    fn test_allow_serde_option_vec_u8_as_base64_encoded_string() {
        let input: ItemStruct = parse_quote! {
            pub struct PageResponse {
                #[prost(bytes = "vec", optional, tag = "1")]
                pub next_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            }
        };

        let result = allow_serde_option_vec_u8_as_base64_encoded_string(input);

        let expected: ItemStruct = parse_quote! {
            pub struct PageResponse {
                #[prost(bytes = "vec", optional, tag = "1")]
                #[serde(
                    serialize_with = "crate::serde::as_option_base64_encoded_string::serialize",
                    deserialize_with = "crate::serde::as_option_base64_encoded_string::deserialize"
                )]
                pub next_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            }
        };

        assert_ast_eq!(result, expected);
    }

    #[test]
    fn test_allow_serde_vec_u8_as_base64_encoded_string_or_string_bytes() {
        let input: ItemStruct = parse_quote! {
            pub struct PageResponse {
                #[prost(bytes = "vec", optional, tag = "1")]
                pub next_key: ::prost::alloc::vec::Vec<u8>,
                #[prost(bytes = "vec", tag = "2")]
                pub specification_id: ::prost::alloc::vec::Vec<u8>,
            }
        };

        let result = allow_serde_vec_u8_as_base64_encoded_string_or_string_bytes(input);

        let expected: ItemStruct = parse_quote! {
            pub struct PageResponse {
                #[prost(bytes = "vec", optional, tag = "1")]
                #[serde(
                    serialize_with = "crate::serde::as_base64_encoded_string::serialize",
                    deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
                )]
                pub next_key: ::prost::alloc::vec::Vec<u8>,
                #[prost(bytes = "vec", tag = "2")]
                #[serde(
                    serialize_with = "crate::serde::as_str_bytes::serialize",
                    deserialize_with = "crate::serde::as_str_bytes::deserialize"
                )]
                pub specification_id: ::prost::alloc::vec::Vec<u8>,
            }
        };

        assert_ast_eq!(result, expected);
    }

    #[test]
    fn test_allow_serde_enum_as_i32() {
        let input: ItemStruct = parse_quote! {
            pub struct Party {
                #[prost(string, tag = "1")]
                pub address: ::prost::alloc::string::String,
                #[prost(enumeration = "PartyType", tag = "2")]
                pub role: i32,
                #[prost(bool, tag = "3")]
                pub optional: bool,
            }
        };

        let result = allow_serde_int_as_str_or_enum_as_i32(input);

        let expected: ItemStruct = parse_quote! {
            pub struct Party {
                #[prost(string, tag = "1")]
                pub address: ::prost::alloc::string::String,
                #[prost(enumeration = "PartyType", tag = "2")]
                #[serde(
                    serialize_with = "crate::serde::enum_as_i32::serialize",
                    deserialize_with = "crate::serde::enum_as_i32::deserialize"
                )]
                pub role: i32,
                #[prost(bool, tag = "3")]
                pub optional: bool,
            }
        };

        assert_ast_eq!(result, expected);
    }
}
