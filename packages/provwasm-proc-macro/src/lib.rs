use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

macro_rules! match_kv_attr {
    ($key:expr, $value_type:tt) => {
        |tt| {
            if let [TokenTree::Ident(key), TokenTree::Punct(eq), TokenTree::$value_type(value)] =
                &tt[..]
            {
                if (key == $key) && (eq.as_char() == '=') {
                    Some(quote!(#value))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

#[proc_macro_derive(CosmwasmExt, attributes(proto_message, proto_query))]
pub fn derive_cosmwasm_ext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let type_url = get_type_url(&input.attrs);

    // `EncodeError` always indicates that a message failed to encode because the
    // provided buffer had insufficient capacity. Message encoding is otherwise
    // infallible.

    let (query_request_conversion, cosmwasm_query) = if get_attr("proto_query", &input.attrs)
        .is_some()
    {
        let path = get_query_attrs(&input.attrs, match_kv_attr!("path", Literal));
        let res = get_query_attrs(&input.attrs, match_kv_attr!("response_type", Ident));

        let query_request_conversion = quote! {
            impl <Q: cosmwasm_std::CustomQuery> From<#ident> for cosmwasm_std::QueryRequest<Q> {
                fn from(msg: #ident) -> Self {
                    cosmwasm_std::QueryRequest::<Q>::Grpc(cosmwasm_std::GrpcQuery{
                        path: #path.to_string(),
                        data: msg.into(),
                    })
                }
            }
        };

        let cosmwasm_query = quote! {
            pub fn query(self, querier: &cosmwasm_std::QuerierWrapper<impl cosmwasm_std::CustomQuery>) -> cosmwasm_std::StdResult<#res> {
                let binary_result = querier.query_grpc(#path.to_string(), self.into())?;
                let response_query = crate::types::tendermint::abci::ResponseQuery::try_from(binary_result)?;
                #res::try_from(response_query.value)
            }

            pub fn mock_response<T: provwasm_common::MockableQuerier>(querier: &mut T, response: #res) {
                querier.register_custom_query(#path.to_string(), Box::new(move |data| {
                    cosmwasm_std::SystemResult::Ok(cosmwasm_std::ContractResult::Ok(
                        cosmwasm_std::Binary::new(crate::types::tendermint::abci::ResponseQuery{
                            code: 0,
                            log: "".to_string(),
                            info: "".to_string(),
                            index: 0,
                            key: vec![],
                            value: response.to_proto_bytes(),
                            proof_ops: None,
                            height: 0,
                            codespace: "".to_string(),
                        }.to_proto_bytes())))
                }))
            }

            pub fn mock_failed_response<T: provwasm_common::MockableQuerier>(querier: &mut T, error: String) {
                querier.register_custom_query(#path.to_string(), Box::new(move |data| {
                    cosmwasm_std::SystemResult::Err(cosmwasm_std::SystemError::InvalidResponse {
                        error: error.clone(),
                        response: cosmwasm_std::Binary::default(),
                    })
                }))
            }
        };

        (query_request_conversion, cosmwasm_query)
    } else {
        (quote!(), quote!())
    };

    (quote! {
        impl #ident {
            pub const TYPE_URL: &'static str = #type_url;
            #cosmwasm_query

            pub fn to_proto_bytes(&self) -> Vec<u8> {
                let mut bytes = Vec::new();
                prost::Message::encode(self, &mut bytes)
                    .expect("Message encoding must be infallible");
                bytes
            }
            pub fn to_any(&self) -> crate::shim::Any {
                crate::shim::Any {
                    type_url: Self::TYPE_URL.to_string(),
                    value: self.to_proto_bytes(),
                }
            }
        }

        #query_request_conversion

        impl From<#ident> for cosmwasm_std::Binary {
            fn from(msg: #ident) -> Self {
                cosmwasm_std::Binary::new(msg.to_proto_bytes())
            }
        }

        impl<T> From<#ident> for cosmwasm_std::CosmosMsg<T> {
            fn from(msg: #ident) -> Self {
                cosmwasm_std::CosmosMsg::<T>::Any(cosmwasm_std::AnyMsg {
                    type_url: #type_url.to_string(),
                    value: msg.into(),
                })
            }
        }

        impl TryFrom<cosmwasm_std::Binary> for #ident {
            type Error = cosmwasm_std::StdError;

            fn try_from(binary: cosmwasm_std::Binary) -> ::std::result::Result<Self, Self::Error> {
                use ::prost::Message;
                Self::decode(&binary[..]).map_err(|e| {
                    cosmwasm_std::StdError::parse_err(
                        stringify!(#ident),
                        format!(
                            "Unable to decode binary: \n  - base64: {}\n  - bytes array: {:?}\n\n{:?}",
                            binary,
                            binary.to_vec(),
                            e
                        )
                    )
                })
            }
        }

        impl TryFrom<Vec<u8>> for #ident {
            type Error = cosmwasm_std::StdError;

            fn try_from(binary: Vec<u8>) -> ::std::result::Result<Self, Self::Error> {
                use ::prost::Message;
                Self::decode(&binary[..]).map_err(|e| {
                    cosmwasm_std::StdError::parse_err(
                        stringify!(#ident),
                        format!(
                            "Unable to decode binary:\n  - bytes array: {:?}\n\n{:?}",
                            binary,
                            e
                        )
                    )
                })
            }
        }

        impl TryFrom<cosmwasm_std::SubMsgResult> for #ident {
            type Error = cosmwasm_std::StdError;

            fn try_from(result: cosmwasm_std::SubMsgResult) -> ::std::result::Result<Self, Self::Error> {
                result
                    .into_result()
                    .map_err(|e| cosmwasm_std::StdError::generic_err(e))?
                    .data
                    .ok_or_else(|| cosmwasm_std::StdError::not_found("cosmwasm_std::SubMsgResult::<T>"))?
                    .try_into()
            }
        }

        impl TryFrom<crate::shim::Any> for #ident {
            type Error = prost::DecodeError;

            fn try_from(value: crate::shim::Any) -> ::std::result::Result<Self, Self::Error> {
                prost::Message::decode(value.value.as_slice())
            }
        }

        impl TryInto<crate::shim::Any> for #ident {
            type Error = prost::EncodeError;

            fn try_into(self) -> ::std::result::Result<crate::shim::Any, Self::Error> {
                let value = prost::Message::encode_to_vec(&self);
                Ok(crate::shim::Any {
                    type_url: <#ident>::TYPE_URL.to_string(),
                    value,
                })
            }
        }
    })
        .into()
}

#[proc_macro_derive(SerdeEnumAsInt)]
pub fn derive_serde_enum_as_int(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    (quote! {
        impl #ident {
            pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let enum_value = Self::try_from(*v);
                match enum_value {
                    Ok(v) => serializer.serialize_str(v.as_str_name()),
                    Err(e) => Err(serde::ser::Error::custom(e)),
                }
            }

            pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Deserialize;
                let s = String::deserialize(deserializer)?;
                match Self::from_str_name(&s) {
                    Some(v) => Ok(v.into()),
                    None => Err(serde::de::Error::custom("unknown value")),
                }
            }

            pub fn serialize_vec<S>(v: &Vec<i32>, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeTuple;

                let mut enum_strs: Vec<&str> = Vec::new();
                for ord in v {
                    // let enum_value = Self::try_from(*ord);
                    let enum_value = Self::try_from(*ord);
                    match enum_value {
                        Ok(v) => {
                            enum_strs.push(v.as_str_name());
                        }
                        Err(e) => return Err(serde::ser::Error::custom(e)),
                    }
                }
                let mut seq = serializer.serialize_tuple(enum_strs.len())?;
                for item in enum_strs {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }

            fn deserialize_vec<'de, D>(deserializer: D) -> std::result::Result<Vec<i32>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::{Deserialize, Error};

                let strs: Vec<String> = Vec::deserialize(deserializer)?;
                let mut ords: Vec<i32> = Vec::new();
                for str_name in strs {
                    let enum_value = Self::from_str_name(&str_name)
                        .ok_or_else(|| Error::custom(format!("unknown enum string: {}", str_name)))?;
                    ords.push(enum_value as i32);
                }
                Ok(ords)
            }
        }
    })
        .into()
}

fn get_type_url(attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let proto_message = get_attr("proto_message", attrs).and_then(|a| a.parse_meta().ok());

    if let Some(syn::Meta::List(meta)) = proto_message.clone() {
        match meta.nested[0].clone() {
            syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) => {
                if meta.path.is_ident("type_url") {
                    match meta.lit {
                        syn::Lit::Str(s) => quote!(#s),
                        _ => proto_message_attr_error(meta.lit),
                    }
                } else {
                    proto_message_attr_error(meta.path)
                }
            }
            t => proto_message_attr_error(t),
        }
    } else {
        proto_message_attr_error(proto_message)
    }
}

fn get_query_attrs<F>(attrs: &[syn::Attribute], f: F) -> proc_macro2::TokenStream
where
    F: FnMut(&Vec<TokenTree>) -> Option<proc_macro2::TokenStream>,
{
    let proto_query = get_attr("proto_query", attrs);

    if let Some(attr) = proto_query {
        if attr.tokens.clone().into_iter().count() != 1 {
            return proto_query_attr_error(proto_query);
        }

        if let Some(TokenTree::Group(group)) = attr.tokens.clone().into_iter().next() {
            let kv_groups = group.stream().into_iter().chunk_by(|t| {
                if let TokenTree::Punct(punct) = t {
                    punct.as_char() != ','
                } else {
                    true
                }
            });
            let mut key_values: Vec<Vec<TokenTree>> = vec![];

            for (non_sep, g) in &kv_groups {
                if non_sep {
                    key_values.push(g.collect());
                }
            }

            return key_values
                .iter()
                .find_map(f)
                .unwrap_or_else(|| proto_query_attr_error(proto_query));
        }

        proto_query_attr_error(proto_query)
    } else {
        proto_query_attr_error(proto_query)
    }
}

fn get_attr<'a>(attr_ident: &str, attrs: &'a [syn::Attribute]) -> Option<&'a syn::Attribute> {
    attrs
        .iter()
        .find(|&attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == attr_ident)
}

fn proto_message_attr_error<T: quote::ToTokens>(tokens: T) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(tokens, "expected `proto_message(type_url = \"...\")`")
        .to_compile_error()
}

fn proto_query_attr_error<T: quote::ToTokens>(tokens: T) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(
        tokens,
        "expected `proto_query(path = \"...\", response_type = ...)`",
    )
    .to_compile_error()
}
