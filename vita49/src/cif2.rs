// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Defines fields and methods related to CIF2 (ANSI/VITA-49.2-2017 9.1).
Fields here are compatible with VITA 49.2 and later.
*/

use crate::{cif0::Cif0, cif7::Cif7Opts};
use deku::prelude::*;
use vita49_macros::{cif_basic, cif_field, cif_fields};

/// Base data structure for the CIF2 single-bit indicators
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cif2(u32);

impl Cif2 {
    cif_field!(bind, 31);
    cif_field!(cited_sid, 30);
    cif_field!(sibling_sid, 29);
    cif_field!(parent_sid, 28);
    cif_field!(child_sid, 27);
    cif_field!(cited_message_id, 26);
    cif_field!(controllee_id, 25);
    cif_field!(controllee_uuid, 24);
    cif_field!(controller_id, 23);
    cif_field!(controller_uuid, 22);
    cif_field!(info_source_id, 21);
    cif_field!(track_id, 20);
    cif_field!(country_code, 19);
    cif_field!(operator, 18);
    cif_field!(platform_class, 17);
    cif_field!(platform_instance, 16);
    cif_field!(platform_display, 15);
    cif_field!(ems_device_class, 14);
    cif_field!(ems_device_type, 13);
    cif_field!(ems_device_instance, 12);
    cif_field!(modulation_class, 11);
    cif_field!(modulation_type, 10);
    cif_field!(function_id, 9);
    cif_field!(mode_id, 8);
    cif_field!(event_id, 7);
    cif_field!(function_priority_id, 6);
    cif_field!(comms_priority_id, 5);
    cif_field!(rf_footprint, 4);
    cif_field!(rf_footprint_range, 3);
    // Bits 0-2 are reserved

    fn empty(&self) -> bool {
        self.0 == 0
    }
}

#[cif_fields(cif2)]
pub struct Cif2Fields {
    bind: u32,
    cited_sid: u32,
    sibling_sid: u32,
    parent_sid: u32,
    child_sid: u32,
    cited_message_id: u32,
    controllee_id: u32,
    controllee_uuid: u128,
    controller_id: u32,
    controller_uuid: u128,
    info_source_id: u32,
    track_id: u32,
    // TODO: add full support
    country_code: u32,
    operator: u32,
    platform_class: u32,
    platform_instance: u32,
    platform_display: u32,
    ems_device_class: u32,
    ems_device_type: u32,
    ems_device_instance: u32,
    modulation_class: u32,
    modulation_type: u32,
    function_id: u32,
    mode_id: u32,
    event_id: u32,
    function_priority_id: u32,
    comms_priority_id: u32,
    rf_footprint: u32,
    rf_footprint_range: u32,
}

/// Trait for common CIF2 manipulation methods. Used by Context and
/// Command packets.
#[rustfmt::skip]
pub trait Cif2Manipulators {
    /// Get a reference to the packet's CIF0 (indicators)
    fn cif0(&self) -> &Cif0;
    /// Get a mutable reference to the packet's CIF0 (indicators)
    fn cif0_mut(&mut self) -> &mut Cif0;
    /// Get a reference to the packet's CIF2 (indicators)
    fn cif2(&self) -> Option<&Cif2>;
    /// Get a mutable reference to the packet's CIF2 (indicators)
    fn cif2_mut(&mut self) -> &mut Option<Cif2>;
    /// Get a reference to the packet's CIF2 data fields
    fn cif2_fields(&self) -> Option<&Cif2Fields>;
    /// Get a mutable reference to the packet's CIF2 data fields
    fn cif2_fields_mut(&mut self) -> &mut Option<Cif2Fields>;

    cif_basic!(cif2, bind, bind, u32);
    cif_basic!(cif2, cited_sid, cited_sid, u32);
    cif_basic!(cif2, sibling_sid, sibling_sid, u32);
    cif_basic!(cif2, parent_sid, parent_sid, u32);
    cif_basic!(cif2, child_sid, child_sid, u32);
    cif_basic!(cif2, cited_message_id, cited_message_id, u32);
    cif_basic!(cif2, controllee_id, controllee_id, u32);
    cif_basic!(cif2, controllee_uuid, controllee_uuid, u128);
    cif_basic!(cif2, controller_id, controller_id, u32);
    cif_basic!(cif2, controller_uuid, controller_uuid, u128);
    cif_basic!(cif2, info_source_id, info_source_id, u32);
    cif_basic!(cif2, track_id, track_id, u32);
    // TODO: add full support
    cif_basic!(cif2, country_code, country_code, u32);
    cif_basic!(cif2, operator, operator, u32);
    cif_basic!(cif2, platform_class, platform_class, u32);
    cif_basic!(cif2, platform_instance, platform_instance, u32);
    cif_basic!(cif2, platform_display, platform_display, u32);
    cif_basic!(cif2, ems_device_class, ems_device_class, u32);
    cif_basic!(cif2, ems_device_type, ems_device_type, u32);
    cif_basic!(cif2, ems_device_instance, ems_device_instance, u32);
    cif_basic!(cif2, modulation_class, modulation_class, u32);
    cif_basic!(cif2, modulation_type, modulation_type, u32);
    cif_basic!(cif2, function_id, function_id, u32);
    cif_basic!(cif2, mode_id, mode_id, u32);
    cif_basic!(cif2, event_id, event_id, u32);
    cif_basic!(cif2, function_priority_id, function_priority_id, u32);
    cif_basic!(cif2, comms_priority_id, comms_priority_id, u32);
    cif_basic!(cif2, rf_footprint, rf_footprint, u32);
    cif_basic!(cif2, rf_footprint_range, rf_footprint_range, u32);
}

/// Enum to describe the various EMS device relationships.
/// See ANSI/VITA-49.2-2017 section 9.8.9 for details.
pub enum EmsOrganizationRelationship {
    /// Coalition device type.
    Coalition,
    /// Known device type.
    Known,
    /// Unknown device type.
    Unknown,
    /// Reserved field.
    Reserved,
}

impl Cif2Fields {
    /// Get the EMS device organization relationship.
    pub fn ems_device_class_org(&self) -> Option<EmsOrganizationRelationship> {
        self.ems_device_class
            .map(|ems_class| match (ems_class >> 14) & 0b11 {
                0b00 => EmsOrganizationRelationship::Coalition,
                0b01 => EmsOrganizationRelationship::Known,
                0b10 => EmsOrganizationRelationship::Unknown,
                0b11 => EmsOrganizationRelationship::Reserved,
                _ => unreachable!(),
            })
    }
    /// Returns true if the device is an exciter, false if not.
    pub fn ems_device_is_exciter(&self) -> Option<bool> {
        self.ems_device_class
            .map(|ems_class| (ems_class & (1 << 13)) > 0)
    }
    /// Returns true if the device is a receiver, false if not.
    pub fn ems_device_is_receiver(&self) -> Option<bool> {
        self.ems_device_class
            .map(|ems_class| (ems_class & (1 << 12)) > 0)
    }
}
