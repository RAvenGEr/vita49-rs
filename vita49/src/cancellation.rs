// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::prelude::*;
use deku::prelude::*;
use std::fmt;

/// Cancellation packet data structure. This is similar to a control packet, but does not include
/// data fields for the set CIF fields. In other words, it only contains indicator fields for the
/// fields you'd like to cancel.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cancellation {
    /// CIF0 indicator fields.
    cif0: Cif0,
    /// CIF1 indicator fields.
    #[deku(cond = "cif0.cif1_enabled()")]
    cif1: Option<Cif1>,
    /// CIF2 indicator fields.
    #[deku(cond = "cif0.cif2_enabled()")]
    cif2: Option<Cif2>,
    /// CIF3 indicator fields.
    #[deku(cond = "cif0.cif3_enabled()")]
    cif3: Option<Cif3>,
}

impl Cancellation {
    /// Get the cancellation size (in 32-bit words).
    pub fn size_words(&self) -> u16 {
        // Start with 1 32-bit word for the CIF0 field
        let mut ret = 1;
        if self.cif1.is_some() {
            ret += 1;
        }
        if self.cif2.is_some() {
            ret += 1;
        }
        if self.cif3.is_some() {
            ret += 1;
        }

        ret
    }

    /// Get a reference to the CIF0 indicator fields.
    pub fn cif0(&self) -> &Cif0 {
        &self.cif0
    }
    /// Get a mutable reference to the CIF0 indicator fields.
    pub fn cif0_mut(&mut self) -> &mut Cif0 {
        &mut self.cif0
    }
    /// Get a reference to the CIF1 indicator fields.
    pub fn cif1(&self) -> Option<&Cif1> {
        self.cif1.as_ref()
    }
    /// Get a mutable reference to the CIF1 indicator fields.
    pub fn cif1_mut(&mut self) -> &mut Option<Cif1> {
        &mut self.cif1
    }
    /// Get a reference to the CIF2 indicator fields.
    pub fn cif2(&self) -> Option<&Cif2> {
        self.cif2.as_ref()
    }
    /// Get a mutable reference to the CIF2 indicator fields.
    pub fn cif2_mut(&mut self) -> &mut Option<Cif2> {
        &mut self.cif2
    }
    /// Get a reference to the CIF3 indicator fields.
    pub fn cif3(&self) -> Option<&Cif3> {
        self.cif3.as_ref()
    }
    /// Get a mutable reference to the CIF3 indicator fields.
    pub fn cif3_mut(&mut self) -> &mut Option<Cif3> {
        &mut self.cif3
    }
}

impl fmt::Display for Cancellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cancellation:")?;
        writeln!(f, "{}", self.cif0)?;
        if let Some(c) = self.cif1 {
            writeln!(f, "{}", c)?;
        }
        // TODO: implement Display for CIF2 and CIF3, then add here.

        Ok(())
    }
}
