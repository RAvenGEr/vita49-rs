// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Defines fields and methods related to CIF3 (ANSI/VITA-49.2-2017 9.1).
Fields here are compatible with VITA 49.2 and later.
*/

use crate::{cif0::Cif0, cif7::Cif7Opts};
use deku::prelude::*;
use fixed::{types::extra::U6, FixedI16};
use vita49_macros::{cif_basic, cif_field, cif_fields, cif_radix_masked, todo_cif_field};

/// Base data structure for the CIF3 single-bit indicators
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cif3(u32);

impl Cif3 {
    cif_field!(timestamp_details, 31);
    cif_field!(timestamp_skew, 30);
    // Bits 28-29 are reserved
    cif_field!(rise_time, 27);
    cif_field!(fall_time, 26);
    cif_field!(offset_time, 25);
    cif_field!(pulse_width, 24);
    cif_field!(period, 23);
    cif_field!(duration, 22);
    cif_field!(dwell, 21);
    cif_field!(jitter, 20);
    // Bits 18-19 are reserved
    todo_cif_field!(age, 17, 3);
    todo_cif_field!(shelf_life, 16, 3);
    // Bits 8-15 are reserved
    cif_field!(air_temp, 7);
    cif_field!(ground_temp, 6);
    cif_field!(humidity, 5);
    cif_field!(barometric_pressure, 4);
    cif_field!(sea_and_swell_state, 3);
    cif_field!(tropospheric_state, 2);
    cif_field!(network_id, 1);
    // Bit 0 is reserved

    fn empty(&self) -> bool {
        self.0 == 0
    }
}

#[cif_fields(cif3)]
pub struct Cif3Fields {
    // TODO: add full support
    timestamp_details: u64,
    timestamp_skew: i64,
    rise_time: i64,
    fall_time: i64,
    offset_time: i64,
    pulse_width: i64,
    period: i64,
    duration: i64,
    dwell: i64,
    jitter: i64,
    // TODO: add basic support
    age: u32,
    // TODO: add basic support
    shelf_life: u32,
    air_temp: i32,
    ground_temp: i32,
    humidity: u32,
    barometric_pressure: u32,
    sea_and_swell_state: u32,
    tropospheric_state: u32,
    network_id: u32,
}

/// Trait for common CIF3 manipulation methods. Used by Context and
/// Command packets.
#[rustfmt::skip]
pub trait Cif3Manipulators {
    /// Get a reference to the packet's CIF0 (indicators)
    fn cif0(&self) -> &Cif0;
    /// Get a mutable reference to the packet's CIF0 (indicators)
    fn cif0_mut(&mut self) -> &mut Cif0;
    /// Get a reference to the packet's CIF3 (indicators)
    fn cif3(&self) -> Option<&Cif3>;
    /// Get a mutable reference to the packet's CIF3 (indicators)
    fn cif3_mut(&mut self) -> &mut Option<Cif3>;
    /// Get a reference to the packet's CIF3 data fields
    fn cif3_fields(&self) -> Option<&Cif3Fields>;
    /// Get a mutable reference to the packet's CIF3 data fields
    fn cif3_fields_mut(&mut self) -> &mut Option<Cif3Fields>;

    // TODO: add full support
    cif_basic!(cif3, timestamp_details, timestamp_details, u64);
    cif_basic!(cif3, timestamp_skew, timestamp_skew, i64);
    cif_basic!(cif3, rise_time, rise_time, i64);
    cif_basic!(cif3, fall_time, fall_time, i64);
    cif_basic!(cif3, offset_time, offset_time, i64);
    cif_basic!(cif3, pulse_width, pulse_width, i64);
    cif_basic!(cif3, period, period, i64);
    cif_basic!(cif3, duration, duration, i64);
    cif_basic!(cif3, dwell, dwell, i64);
    cif_basic!(cif3, jitter, jitter, i64);
    // TODO: add basic support
    cif_basic!(cif3, age, age, u32);
    // TODO: add basic support
    cif_basic!(cif3, shelf_life, shelf_life, u32);
    cif_radix_masked!(cif3, air_temp, air_temp_c, f32, FixedI16::<U6>, i32, i16);
    cif_radix_masked!(cif3, ground_temp, ground_temp_c, f32, FixedI16::<U6>, i32, i16);
    // TODO: add full support
    cif_basic!(cif3, humidity, humidity, u32);
    // TODO: add full support
    cif_basic!(cif3, barometric_pressure, barometric_pressure, u32);
    // TODO: add full support
    cif_basic!(cif3, sea_and_swell_state, sea_and_swell_state, u32);
    // TODO: add full support
    cif_basic!(cif3, tropospheric_state, tropospheric_state, u32);
    cif_basic!(cif3, network_id, network_id, u32);
}
