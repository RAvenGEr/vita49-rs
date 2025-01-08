// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Data structures and methods related to the ECEF ephemeris format
(ANSI/VITA-49.2-2017 section 9.4.3).
*/

use deku::prelude::*;

/// Base ECEF ephemeris data structure.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EcefEphemeris {
    w1: u32,
    ts1: u32,
    ts2: u32,
    ts3: u32,
    position_x: i32,
    position_y: i32,
    position_z: i32,
    attitude_alpha: i32,
    attitude_beta: i32,
    attitude_phi: i32,
    velocity_dx: i32,
    velocity_dy: i32,
    velocity_dz: i32,
}

impl EcefEphemeris {
    /// Gets the size of the ECEF ephemeris field in 32-bit words.
    pub fn size_words(&self) -> u16 {
        (std::mem::size_of_val(self) / std::mem::size_of::<u32>()) as u16
    }
}
