// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Data structures and methods related to the formatted GPS format
(ANSI/VITA-49.2-2017 section 9.4.5).
*/

use deku::prelude::*;

/// Base formatted GPS data structure.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattedGps {
    w1: u32,
    ts1: u32,
    ts2: u32,
    ts3: u32,
    latitude: i32,
    longitude: i32,
    altitude: i32,
    speed_over_ground: i32,
    heading_angle: i32,
    track_angle: i32,
    magnetic_variation: i32,
}

impl FormattedGps {
    /// Gets the size of the formatted GPS structure in 32-bit words.
    pub fn size_words(&self) -> u16 {
        (std::mem::size_of_val(self) / std::mem::size_of::<u32>()) as u16
    }
}
