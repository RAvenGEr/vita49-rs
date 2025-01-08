// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Data structures and methods related to the device identifier field
(ANSI/VITA-49.2-2017 section 9.10.1).
*/

use core::fmt;
use deku::prelude::*;

/// Base device ID data structure.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceId(u64);

impl DeviceId {
    /// Gets the size of the device ID structure in 32-bit words.
    pub fn size_words(&self) -> u16 {
        (std::mem::size_of_val(&self.0) / std::mem::size_of::<u32>()) as u16
    }

    /// Gets the manufacturer Organizational Unique Identifier (OUI).
    pub fn manufacturer_oui(&self) -> u32 {
        ((self.0 >> 32) & 0xFF_FFFF) as u32
    }

    /// Sets the manufacturer Organizational Unique Identifier (OUI).
    ///
    /// Note: while this API takes a 32-bit integer, only the least
    /// significant 24 bits are used.
    pub fn set_manufacturer_oui(&mut self, oui: u32) {
        let val = (oui as u64 & 0xFF_FFFF) << 32;
        self.0 = self.0 & !(0xFF_FFFF << 32) | val;
    }

    /// Gets the device code.
    pub fn device_code(&self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }

    /// Sets the device code.
    pub fn set_device_code(&mut self, code: u16) {
        self.0 = self.0 & !(0xFFFF) | code as u64;
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Device ID:")?;
        writeln!(f, "  Manufacturer OUI: {:#x}", self.manufacturer_oui())?;
        writeln!(f, "  Device code: {}", self.device_code())?;
        Ok(())
    }
}
