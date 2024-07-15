use std::ffi::OsStr;
use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{self, File, Item};

pub fn generate_mod_file(for_dir: &Path) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let types_dir = root.join(for_dir);

    let paths = fs::read_dir(&types_dir)
        .expect("[error] Unable to read dir")
        .filter_map(|d| {
            let f = d.expect("[error] Unable to get dir entry");
            if f.path().extension() == Some(OsStr::new("rs")) {
                f.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            } else {
                None
            }
        })
        .map(|s| s.split('.').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    paths
        .iter()
        .for_each(|p| create_dir_all(for_dir.join(p[..(p.len() - 1)].join("/"))).unwrap());

    recur_gen_mod(&types_dir, &types_dir, paths, "");
}

fn recur_gen_mod(for_dir: &Path, start_dir: &Path, paths: Vec<Vec<String>>, include_file: &str) {
    let uniq_keys = paths
        .iter()
        .filter_map(|p| (*p).get(0))
        .map(|s| s.to_owned())
        .unique()
        .sorted()
        .collect::<Vec<String>>();

    // base case
    if uniq_keys.is_empty() {
        let from = start_dir.join(format!("{}.rs", include_file.replace('/', ".")));
        let to = for_dir
            .parent()
            .unwrap()
            .join(format!("{}.rs", include_file.split('.').last().unwrap()));
        fs::rename(from, to).unwrap();
    } else {
        let ts = uniq_keys.iter().map(|k| {
            let module = format_ident!("{}", k);
            quote! { pub mod #module; }
        });

        let additional_mod_content = if paths.iter().any(|p| p.is_empty()) && paths.len() > 1 {
            let src_file = start_dir.join(format!("{}.rs", include_file));
            let tk = fs::read_to_string(src_file.clone())
                .unwrap()
                .parse::<TokenStream2>()
                .unwrap();

            fs::remove_file(src_file).unwrap();

            tk
        } else {
            quote!()
        };

        create_mod_rs(
            quote! {
                #(#ts)*

                #additional_mod_content
            },
            for_dir,
        );

        for k in uniq_keys {
            let paths: Vec<Vec<String>> = paths
                .iter()
                // only if head = k
                .filter(|p| (**p).get(0) == Some(&k))
                // get tail
                .map(|p| p.split_at(1).1.to_vec())
                .collect();
            let include_file = if include_file.is_empty() {
                k.clone()
            } else {
                format!("{include_file}.{k}")
            };

            recur_gen_mod(
                &for_dir.join(k.clone()),
                start_dir,
                paths.clone(),
                &include_file,
            );
        }
    }
}

fn create_mod_rs(ts: TokenStream2, path: &Path) {
    let file = syn::parse_file(ts.to_string().as_str())
        .expect("[error] Unable to parse generated content as file while genrating mod.rs");

    let write = fs::write(path.join("mod.rs"), prettyplease::unparse(&file));

    if let Err(e) = write {
        panic!("[error] Error while generating mod.rs: {}", e);
    }
}

pub fn add_root_features(root: &PathBuf) {
    // Read mod.rs
    let mod_rs_path = root.join("mod.rs");
    let contents = fs::read_to_string(&mod_rs_path).expect("Unable to read mod.rs file");

    // Parse mod.rs into a syntax tree
    let mut syntax_tree: File = syn::parse_file(&contents).expect("Unable to parse mod.rs file");

    // Update items
    for item in syntax_tree.items.iter_mut() {
        if let Item::Mod(module) = item {
            // Create the feature flag attribute
            let feature_name = format!("feature = \"{}\"", module.ident);
            let feature_meta: syn::Meta =
                syn::parse_str(&feature_name).expect("Failed to parse feature attribute");
            let feature_attr = syn::Attribute {
                pound_token: syn::token::Pound::default(),
                style: syn::AttrStyle::Outer,
                bracket_token: syn::token::Bracket::default(),
                path: syn::Path::from(syn::Ident::new("cfg", module.ident.span())),
                tokens: quote::quote!( ( #feature_meta ) ).into(),
            };

            // Add the feature attribute to the module's attributes
            module.attrs.push(feature_attr);
        }
    }

    // Generate the updated file contents
    let updated_file = syntax_tree.to_token_stream().to_string();

    // Write the updated contents back to mod.rs
    fs::write(mod_rs_path, updated_file).expect("Unable to write to mod.rs file");
}
