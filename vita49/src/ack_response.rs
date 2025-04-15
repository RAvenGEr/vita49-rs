// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt;

use deku::prelude::*;

/// ACK response field. Each warning or error field in an ACK packet uses one of these regardless
/// of the underlying field's data type. For example, even though `bandwidth` is a 64-bit field in
/// VITA 49, the response field is always 32-bits that represent various things that might be wrong
/// with the bandwidth.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AckResponse(u32);

impl AckResponse {
    /// Returns true if a bit in the field is set, false if not.
    fn bit_is_set(&self, bit: u32) -> bool {
        self.0 & (1 << bit) != 0
    }

    /// Sets the bit.
    fn set_bit(&mut self, bit: u32) {
        self.0 |= 1 << bit;
    }

    /// Unsets the bit.
    fn unset_bit(&mut self, bit: u32) {
        self.0 &= !(1 << bit);
    }

    /// The field was NOT executed because of a Warning or Error.
    pub fn field_not_executed(&self) -> bool {
        self.bit_is_set(31)
    }
    /// Set the field_not_executed bit.
    pub fn set_field_not_executed(&mut self) {
        self.set_bit(31)
    }
    /// Unset the field_not_executed bit.
    pub fn unset_field_not_executed(&mut self) {
        self.unset_bit(31);
    }

    /// The field was NOT executed *properly* because of a device/hardware failure.
    pub fn device_failure(&self) -> bool {
        self.bit_is_set(30)
    }
    /// Set the device_failure bit.
    pub fn set_device_failure(&mut self) {
        self.set_bit(30)
    }
    /// Unset the device_failure bit.
    pub fn unset_device_failure(&mut self) {
        self.unset_bit(30);
    }

    /// The device does NOT accept this particular Control field.
    pub fn erroneous_field(&self) -> bool {
        self.bit_is_set(29)
    }
    /// Set the erroneous_field bit.
    pub fn set_erroneous_field(&mut self) {
        self.set_bit(29)
    }
    /// Unset the erroneous_field bit.
    pub fn unset_erroneous_field(&mut self) {
        self.unset_bit(29);
    }

    /// The supplied field is beyond the capability or operational range of this device.
    pub fn param_out_of_range(&self) -> bool {
        self.bit_is_set(28)
    }
    /// Set the param_out_of_range bit.
    pub fn set_param_out_of_range(&mut self) {
        self.set_bit(28)
    }
    /// Unset the param_out_of_range bit.
    pub fn unset_param_out_of_range(&mut self) {
        self.unset_bit(28);
    }

    /// The supplied field value specifies a level of precision beyond the capability of this device
    pub fn parameter_unsupported_precision(&self) -> bool {
        self.bit_is_set(27)
    }
    /// Set the parameter_unsupported_precision bit.
    pub fn set_parameter_unsupported_precision(&mut self) {
        self.set_bit(27)
    }
    /// Unset the parameter_unsupported_precision bit.
    pub fn unset_parameter_unsupported_precision(&mut self) {
        self.unset_bit(27);
    }

    /// This field had an invalid setting beyond those specified above.
    pub fn field_value_invalid(&self) -> bool {
        self.bit_is_set(26)
    }
    /// Set the field_value_invalid bit.
    pub fn set_field_value_invalid(&mut self) {
        self.set_bit(26)
    }
    /// Unset the field_value_invalid bit.
    pub fn unset_field_value_invalid(&mut self) {
        self.unset_bit(26);
    }

    /// The Controllee was unable to meet the timestamp requirement specified by the [T2,T1,T0] bits for the specified field.
    pub fn timestamp_problem(&self) -> bool {
        self.bit_is_set(25)
    }
    /// Set the timestamp_problem bit.
    pub fn set_timestamp_problem(&mut self) {
        self.set_bit(25)
    }
    /// Unset the timestamp_problem bit.
    pub fn unset_timestamp_problem(&mut self) {
        self.unset_bit(25);
    }

    /// The supplied field will cause transmission of hazardous power levels.
    pub fn hazardous_power_levels(&self) -> bool {
        self.bit_is_set(24)
    }
    /// Set the foo bit.
    pub fn set_hazardous_power_levels(&mut self) {
        self.set_bit(24)
    }
    /// Unset the hazardous_power_levels bit.
    pub fn unset_hazardous_power_levels(&mut self) {
        self.unset_bit(24);
    }

    /// The supplied field will cause components to be over driven leading to distortion. This applies to both receive and transmit.
    pub fn distortion(&self) -> bool {
        self.bit_is_set(23)
    }
    /// Set the distrortion bit.
    pub fn set_distrortion(&mut self) {
        self.set_bit(23)
    }
    /// Unset the distrortion bit.
    pub fn unset_distrortion(&mut self) {
        self.unset_bit(23);
    }

    /// The supplied field will place the in-band power levels out of compliance.
    pub fn in_band_power_compliance(&self) -> bool {
        self.bit_is_set(22)
    }
    /// Set the in_band_power_compliance bit.
    pub fn set_in_band_power_compliance(&mut self) {
        self.set_bit(22)
    }
    /// Unset the in_band_power_compliance bit.
    pub fn unset_in_band_power_compliance(&mut self) {
        self.unset_bit(22);
    }

    /// The supplied field will place the out-of-band power levels out of compliance.
    pub fn out_of_band_power_compliance(&self) -> bool {
        self.bit_is_set(21)
    }
    /// Set the out_of_band_power_compliance bit.
    pub fn set_out_of_band_power_compliance(&mut self) {
        self.set_bit(21)
    }
    /// Unset the out_of_band_power_compliance bit.
    pub fn unset_out_of_band_power_compliance(&mut self) {
        self.unset_bit(21);
    }

    /// The supplied field will cause co-site interference between transmitter and receiver at same location.
    pub fn co_site_interference(&self) -> bool {
        self.bit_is_set(20)
    }
    /// Set the co_site_interference bit.
    pub fn set_co_site_interference(&mut self) {
        self.set_bit(20)
    }
    /// Unset the co_site_interference bit.
    pub fn unset_co_site_interference(&mut self) {
        self.unset_bit(20);
    }

    /// The supplied field will cause interference between devices in the same operational region.
    pub fn regional_interference(&self) -> bool {
        self.bit_is_set(19)
    }
    /// Set the regional_interference bit.
    pub fn set_regional_interference(&mut self) {
        self.set_bit(19)
    }
    /// Unset the regional_interference bit.
    pub fn unset_regional_interference(&mut self) {
        self.unset_bit(19);
    }

    // Bits 18-13 are reserved
    /// User-defined error/warning types (bits 12-1).
    /// Pass in the bit number (must be between 1 and 12 inclusively).
    pub fn user_defined(&self, bit: u32) -> bool {
        assert!(bit > 0 && bit < 13);
        self.bit_is_set(bit)
    }
    /// Set the user_defined bit.
    /// Pass in the bit number (must be between 1 and 12 inclusively).
    pub fn set_user_defined(&mut self, bit: u32) {
        assert!(bit > 0 && bit < 13);
        self.set_bit(bit)
    }
    /// Unset the user_defined_ bit.
    /// Pass in the bit number (must be between 1 and 12 inclusively).
    pub fn unset_user_defined(&mut self, bit: u32) {
        assert!(bit > 0 && bit < 13);
        self.unset_bit(bit);
    }
    // Bit 0 is reserved

    /// Returns the size of the ACK response field in 32-bit words.
    pub fn size_words(&self) -> u16 {
        (std::mem::size_of_val(self) / std::mem::size_of::<u32>()) as u16
    }

    /// Returns true if no fields in the response are set.
    pub fn empty(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for AckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.field_not_executed() {
            writeln!(f, "- Field not executed")?
        }
        if self.device_failure() {
            writeln!(f, "- Device failure")?
        }
        if self.erroneous_field() {
            writeln!(f, "- Erroneous field")?
        }
        if self.param_out_of_range() {
            writeln!(f, "- Parameter out of range")?
        }
        if self.parameter_unsupported_precision() {
            writeln!(f, "- Parameter with unsupported precision")?
        }
        if self.field_value_invalid() {
            writeln!(f, "- Field value invalid")?
        }
        if self.timestamp_problem() {
            writeln!(f, "- Timestamp problem")?
        }
        if self.hazardous_power_levels() {
            writeln!(f, "- Hazardous power levels")?
        }
        if self.distortion() {
            writeln!(f, "- Distortion")?
        }
        if self.in_band_power_compliance() {
            writeln!(f, "- In-band power compliance error")?
        }
        if self.out_of_band_power_compliance() {
            writeln!(f, "- Out-of-band power compliance error")?
        }
        if self.co_site_interference() {
            writeln!(f, "- Co-site interference")?
        }
        if self.regional_interference() {
            writeln!(f, "- Regional interference")?
        }
        for bit in 1..=12 {
            if self.user_defined(bit) {
                writeln!(f, "- User-defined error (bit {})", bit)?
            }
        }
        Ok(())
    }
}
