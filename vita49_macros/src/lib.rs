// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;

mod cif_basic;
mod cif_field;
mod cif_fields;
mod cif_radix;
mod cif_radix_masked;
mod todo_cif_field;

/// Generates a getter, setter, and unsetter function for the
/// given CIF field.
///
/// In format:
///  - `${cif_field}()`: Returns bit state (1 = true, 0 = false)
///  - `set_${cif_field}()`: Sets the bit
///  - `unset_${cif_field}()`: Unsets the bit
#[proc_macro]
pub fn cif_field(input: TokenStream) -> TokenStream {
    cif_field::cif_field(input.into()).into()
}

/// Attribute macro that generates struct members with the proper
/// deku conditionals attached. Also automatically handles the
/// CIF7 attributes members associated with each CIF field (e.g.
/// bandwidth_attributes) and generates a size_words() method
/// based on each field's type.
#[proc_macro_attribute]
pub fn cif_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    cif_fields::cif_fields(attr, item)
}

/// Generates a TODO implementation for the given CIF field.
/// If the field is encountered at runtime, the program will
/// panic.
#[proc_macro]
pub fn todo_cif_field(input: TokenStream) -> TokenStream {
    todo_cif_field::todo_cif_field(input.into()).into()
}

/// Generates getter/setter implementations for a given CIF
/// field that is represented internally as a simple type.
/// The same type is used for the internal representation and
/// the user-facing value.
#[proc_macro]
pub fn cif_basic(input: TokenStream) -> TokenStream {
    cif_basic::cif_basic(input.into()).into()
}

/// Generates getter/setter implementations for a given CIF
/// field that is represented internally as a fixed point
/// number. A "friendly" primitive type is exposed.
#[proc_macro]
pub fn cif_radix(input: TokenStream) -> TokenStream {
    cif_radix::cif_radix(input.into()).into()
}

/// Generates getter/setter implementations for a given CIF
/// field that is represented internally as an integer, but
/// some portion of the field is reserve/irrelevant. For example,
/// the field may be a 32-bit int, but only the lower 16 bits are
/// used to represent a fixed point number.
#[proc_macro]
pub fn cif_radix_masked(input: TokenStream) -> TokenStream {
    cif_radix_masked::cif_radix_masked(input.into()).into()
}
