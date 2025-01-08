// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Defines fields and methods related to CIF7 (ANSI/VITA-49.2-2017 section 9.1).
Fields here are compatible with VITA 49.2 and later.

Note: CIF7 behaves differently than other CIFs - it adds attribute
fields to all other CIF fields. For example, if you set the `current` and
`average` bits in CIF7, and you're using the `bandwidth` field in CIF0,
the current bandwidth value will be sent in the CIF0 fields followed immediately
by the *average* bandwidth.

This crate does not handle the math for the descriptive statistics below and
additionally does not provide hooks out for each individual attribute field.
Instead, each CIF field has an equivalent `*_attributes` field which is a vector
of the main CIF field type.

So, a user wishing to use CIF7 would need to do some additional work to correlate
the vector of values with the statistical fields in CIF7.

See ANSI/VITA-49.2-2017 section 9.12 for additional details.
*/

use deku::prelude::*;
use vita49_macros::cif_field;

/// Base data structure for the CIF7 single-bit indicators.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cif7(u32);

impl Cif7 {
    cif_field!(current, 31);
    cif_field!(average, 30);
    cif_field!(median, 29);
    cif_field!(std_dev, 28);
    cif_field!(max, 27);
    cif_field!(min, 26);
    cif_field!(precision, 25);
    cif_field!(accuracy, 24);
    cif_field!(first_derivative, 23);
    cif_field!(second_derivative, 22);
    cif_field!(third_derivative, 21);
    cif_field!(probability, 20);
    cif_field!(belief, 19);
    // Bits 0-18 are reserved

    /// Returns the number of set bits in CIF7. This is
    /// used internally to know how many fields to parse
    /// when reading a packet with CIF7 enabled.
    pub fn num_set(&self) -> usize {
        u32::count_ones(self.0) as usize
    }
}

/// Structure representing the state of CI7.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct Cif7Opts {
    /// If the "current" value bit is set, we should include the
    /// main field which would be included by default if CIF7 was
    /// not used. But, if it's not set in CIF7, it wouldn't be
    /// included.
    pub(crate) current_val: bool,
    /// The number of attributes (other than current) being used
    /// in CIF7. This is used to count how many fields to parse.
    pub(crate) num_extra_attrs: usize,
}

impl Cif7Opts {
    /// Translate from a literal `Cif7` to a friendlier `Cif7Opts` structure.
    pub(crate) fn from(cif7: Option<&Cif7>) -> Cif7Opts {
        if let Some(c) = cif7 {
            Cif7Opts {
                current_val: c.current(),
                num_extra_attrs: if c.num_set() > 0 { c.num_set() - 1 } else { 0 },
            }
        } else {
            Cif7Opts {
                current_val: true,
                num_extra_attrs: 0,
            }
        }
    }
}
