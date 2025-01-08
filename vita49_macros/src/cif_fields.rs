// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemStruct};

static PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "char", "bool",
];

pub fn cif_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cif_name = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident;
    let mut expanded_fields = Vec::new();
    let mut expanded_size_lines = Vec::new();
    let mut empty_check_lines = Vec::new();

    for field in input.fields {
        let cif_field = field.clone().ident.unwrap();
        let cif_type = field.clone().ty;

        let field_doc = format!("{} data field", cif_field);
        let attr_doc = format!(
            "{} data attributes field (only used if CIF7 is enabled)",
            cif_field
        );

        let attr_field = format_ident!("{}_attributes", cif_field);
        // CIF fields other than cif0 are optional, so we have to add an unwrap()
        let (main_cond, attr_cond) = if cif_name == "cif0" {
            (
                format!("{}.{}() && cif7_opts.current_val", cif_name, cif_field),
                format!(
                    "{}.{}() && cif7_opts.num_extra_attrs > 0",
                    cif_name, cif_field
                ),
            )
        } else {
            (
                format!(
                    "{}.unwrap().{}() && cif7_opts.current_val",
                    cif_name, cif_field
                ),
                format!(
                    "{}.unwrap().{}() && cif7_opts.num_extra_attrs > 0",
                    cif_name, cif_field
                ),
            )
        };

        let expanded = quote! {
            #[doc = #field_doc]
            #[deku(cond = #main_cond)]
            pub #cif_field: Option<#cif_type>,

            #[doc = #attr_doc]
            #[cfg(feature = "cif7")]
            #[deku(cond = #attr_cond, count = "cif7_opts.num_extra_attrs")]
            pub #attr_field: Vec<#cif_type>,
        };
        expanded_fields.push(expanded);

        let cif_type_string = cif_type.to_token_stream().to_string();

        let expanded = if PRIMITIVES.contains(&cif_type_string.as_str()) {
            quote! {
                if let Some(v) = &self.#cif_field {
                    acc += (std::mem::size_of_val(v) / std::mem::size_of::<u32>()) as u16;
                }
                #[cfg(feature = "cif7")]
                if let Some(v) = self.#attr_field.first() {
                    acc += ((std::mem::size_of_val(v) * self.#attr_field.len()) / std::mem::size_of::<u32>()) as u16;
                }
            }
        } else {
            quote! {
                if let Some(v) = &self.#cif_field {
                    acc += v.size_words();
                }
                #[cfg(feature = "cif7")]
                if let Some(v) = self.#attr_field.first() {
                    acc += v.size_words() * (self.#attr_field.len() as u16);
                }
            }
        };

        expanded_size_lines.push(expanded);

        let expanded = quote! {
            #[cfg(feature = "cif7")]
            if self.#cif_field.is_some() || ! self.#attr_field.is_empty() {
                return false;
            }
            #[cfg(not(feature = "cif7"))]
            if self.#cif_field.is_some() {
                return false;
            }
        };
        empty_check_lines.push(expanded);
    }

    let cif_name_str = cif_name.to_string();
    let mut cif_name_chars = cif_name_str.chars();
    let mut cif_type_name = match cif_name_chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + cif_name_chars.as_str(),
    };
    cif_type_name = format!("&{}", cif_type_name);
    if cif_name != "cif0" {
        cif_type_name = format!("Option<{}>", cif_type_name);
    }
    let deku_ctx = format!(
        "endian: deku::ctx::Endian, {}: {}, cif7_opts: Cif7Opts",
        cif_name, cif_type_name
    );
    let struct_doc = format!(
        "Structure for all {} data fields (not indicators)",
        cif_name
    );
    let size_doc = format!(
        "Gets the size of all {} data fields in 32-bit words",
        cif_name
    );
    let empty_doc = format!(
        "Returns true if all {} data fields are empty, false if not",
        cif_name
    );

    let expanded = quote! {
        #[doc = #struct_doc]
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite)]
        #[deku(
            endian = "endian",
            ctx = #deku_ctx,
        )]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #struct_name {
            #(#expanded_fields)*
        }

        impl #struct_name {
            #[doc = #size_doc]
            pub fn size_words(&self) -> u16 {
                let mut acc = 0;
                #(#expanded_size_lines)*
                acc
            }

            #[doc = #empty_doc]
            pub fn empty(&self) -> bool {
                #(#empty_check_lines)*
                true
            }
        }
    };

    TokenStream::from(expanded)
}
