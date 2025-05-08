// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Ident, LitInt, Token};

struct CifFieldArgs {
    cif_field: Ident,
    _comma: Token![,],
    bit: LitInt,
}

impl Parse for CifFieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let cif_field = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let bit = input.parse()?;
        Ok(CifFieldArgs {
            cif_field,
            _comma,
            bit,
        })
    }
}

pub fn cif_field(input: TokenStream) -> TokenStream {
    let CifFieldArgs { cif_field, bit, .. } = parse2(input).expect("failed to parse macro input");

    let set = format_ident!("set_{}", cif_field);
    let unset = format_ident!("unset_{}", cif_field);

    let get_doc = format!("Returns true if the {cif_field} CIF field bit is set, false otherwise");
    let set_doc = format!("Sets the {cif_field} CIF field bit");
    let unset_doc = format!("Unsets the {cif_field} CIF field bit");

    quote! {
        #[doc = #get_doc]
        pub fn #cif_field(&self) -> bool {
            self.0 & (1 << #bit) != 0
        }
        #[doc = #set_doc]
        pub fn #set(&mut self) {
            self.0 |= 1 << #bit;
        }
        #[doc = #unset_doc]
        pub fn #unset(&mut self) {
            self.0 &= !(1 << #bit);
        }
    }
}
