// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Data structures and methods related to command payloads
(ANSI/VITA-49.2-2017 section 8).
*/

use core::fmt;

use deku::prelude::*;

use crate::{
    cif0::{Cif0, Cif0Fields, Cif0Manipulators},
    cif1::{Cif1, Cif1Fields, Cif1Manipulators},
    cif2::{Cif2, Cif2Fields, Cif2Manipulators},
    cif3::{Cif3, Cif3Fields, Cif3Manipulators},
    cif7::{Cif7, Cif7Opts},
    control_ack_mode::{ControlAckMode, IdFormat},
    payload::Payload,
    VitaError,
};

/// Main command payload structure.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Command {
    /// Control acknowledgement mode.
    cam: ControlAckMode,
    /// Message ID.
    message_id: u32,
    #[deku(cond = "cam.controllee_enabled() && cam.controllee_id_format() == IdFormat::Id32bit")]
    /// Controllee ID.
    controllee_id: Option<u32>,
    #[deku(
        cond = "cam.controllee_enabled() && cam.controllee_id_format() == IdFormat::Uuid128bit"
    )]
    /// Controllee UUID.
    controllee_uuid: Option<u128>,
    #[deku(cond = "cam.controller_enabled() && cam.controller_id_format() == IdFormat::Id32bit")]
    /// Controller ID.
    controller_id: Option<u32>,
    #[deku(
        cond = "cam.controller_enabled() && cam.controller_id_format() == IdFormat::Uuid128bit"
    )]
    /// Controller UUID.
    controller_uuid: Option<u128>,
    /// CIF0 indicator fields.
    cif0: Cif0,
    #[deku(cond = "cif0.cif1_enabled()")]
    /// CIF1 indicator fields.
    cif1: Option<Cif1>,
    #[deku(cond = "cif0.cif2_enabled()")]
    /// CIF2 indicator fields.
    cif2: Option<Cif2>,
    #[deku(cond = "cif0.cif3_enabled()")]
    /// CIF3 indicator fields.
    cif3: Option<Cif3>,
    #[deku(cond = "cif0.field_attributes_enabled()")]
    /// CIF7 indicator fields.
    pub cif7: Option<Cif7>,

    #[deku(ctx = "cif0, Cif7Opts::from(cif7.as_ref())")]
    /// CIF0 data fields.
    cif0_fields: Cif0Fields,
    #[deku(
        cond = "cif0.cif1_enabled()",
        ctx = "cif1.as_ref(), Cif7Opts::from(cif7.as_ref())"
    )]
    /// CIF1 data fields.
    cif1_fields: Option<Cif1Fields>,
    #[deku(
        cond = "cif0.cif2_enabled()",
        ctx = "cif2.as_ref(), Cif7Opts::from(cif7.as_ref())"
    )]
    /// CIF2 data fields.
    cif2_fields: Option<Cif2Fields>,
    #[deku(
        cond = "cif0.cif3_enabled()",
        ctx = "cif3.as_ref(), Cif7Opts::from(cif7.as_ref())"
    )]
    /// CIF3 data fields.
    cif3_fields: Option<Cif3Fields>,
}

impl Command {
    /// Create a new, empty command packet.
    pub fn new() -> Command {
        Command::default()
    }

    /// Get the packet message ID.
    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    /// Set the packet message ID.
    pub fn set_message_id(&mut self, message_id: u32) {
        self.message_id = message_id;
    }

    /// Get the packet's Control Ack Mode (CAM)
    pub fn cam(&self) -> ControlAckMode {
        self.cam
    }

    /// Set the packet's Control Ack Mode (CAM)
    /// # Example
    /// ```
    /// use vita49::{prelude::*, ControlAckMode, ActionMode};
    /// let mut packet = Vrt::new_command_packet();
    /// let command_mut = packet.payload_mut().command_mut().unwrap();
    /// let mut cam = ControlAckMode::default();
    /// cam.set_action_mode(ActionMode::Execute);
    /// command_mut.set_cam(cam);
    /// assert_eq!(command_mut.cam().action_mode(), ActionMode::Execute);
    /// ````
    pub fn set_cam(&mut self, mode: ControlAckMode) {
        self.cam = mode;
    }

    /// Get the controllee identifier.
    pub fn controllee_id(&self) -> Option<u32> {
        self.controllee_id
    }
    /// Sets the controllee identifier. If `None` is passed, the field
    /// will be unset.
    ///
    /// # Errors
    /// If this function is called while the `controllee_uuid` field is set,
    /// an error will be returned as these fields are mutually exclusive.
    pub fn set_controllee_id(&mut self, id: Option<u32>) -> Result<(), VitaError> {
        if self.controllee_uuid.is_some() {
            return Err(VitaError::TriedIdWhenUuidSet);
        }
        self.controllee_id = id;
        if id.is_some() {
            self.cam.enable_controllee();
            self.cam.set_controllee_id_format(IdFormat::Id32bit);
        } else {
            self.cam.disable_controllee();
            self.cam.set_controllee_id_format(IdFormat::Uuid128bit);
        }
        Ok(())
    }

    /// Get the controller identifier.
    pub fn controller_id(&self) -> Option<u32> {
        self.controller_id
    }
    /// Sets the controller identifier. If `None` is passed, the field
    /// will be unset.
    ///
    /// # Errors
    /// If this function is called while the `controller_uuid` field is set,
    /// an error will be returned as these fields are mutually exclusive.
    pub fn set_controller_id(&mut self, id: Option<u32>) -> Result<(), VitaError> {
        if self.controller_uuid.is_some() {
            return Err(VitaError::TriedIdWhenUuidSet);
        }
        self.controller_id = id;
        if id.is_some() {
            self.cam.enable_controller();
            self.cam.set_controller_id_format(IdFormat::Id32bit);
        } else {
            self.cam.disable_controller();
            self.cam.set_controller_id_format(IdFormat::Uuid128bit);
        }
        Ok(())
    }

    /// Get the controllee UUID.
    pub fn controllee_uuid(&self) -> Option<u128> {
        self.controllee_uuid
    }
    /// Sets the controllee UUID. If `None` is passed, the field
    /// will be unset.
    ///
    /// # Errors
    /// If this function is called while the `controllee_id` field is set,
    /// an error will be returned as these fields are mutually exclusive.
    pub fn set_controllee_uuid(&mut self, uuid: Option<u128>) -> Result<(), VitaError> {
        if self.controllee_uuid.is_some() {
            return Err(VitaError::TriedUuidWhenIdSet);
        }
        self.controllee_uuid = uuid;
        if uuid.is_some() {
            self.cam.enable_controllee();
            self.cam.set_controllee_id_format(IdFormat::Uuid128bit);
        } else {
            self.cam.disable_controllee();
            self.cam.set_controllee_id_format(IdFormat::Id32bit);
        }
        Ok(())
    }

    /// Get the controller UUID.
    pub fn controller_uuid(&self) -> Option<u128> {
        self.controller_uuid
    }
    /// Sets the controller UUID. If `None` is passed, the field
    /// will be unset.
    ///
    /// # Errors
    /// If this function is called while the `controller_uuid` field is set,
    /// an error will be returned as these fields are mutually exclusive.
    pub fn set_controller_uuid(&mut self, uuid: Option<u128>) -> Result<(), VitaError> {
        if self.controller_uuid.is_some() {
            return Err(VitaError::TriedUuidWhenIdSet);
        }
        self.controller_uuid = uuid;
        if uuid.is_some() {
            self.cam.enable_controller();
            self.cam.set_controller_id_format(IdFormat::Uuid128bit);
        } else {
            self.cam.disable_controller();
            self.cam.set_controller_id_format(IdFormat::Id32bit);
        }
        Ok(())
    }

    /// Get the size of the command packet (in 32-bit words).
    pub fn size_words(&self) -> u16 {
        // Start with 1 32-bit word for the CIF0 field
        let mut ret = 1 + self.cif0_fields.size_words();

        ret += self.cam.size_words();
        ret += 1; // message_id
        if self.controllee_id.is_some() {
            ret += 1;
        } else if self.controllee_uuid.is_some() {
            ret += 4;
        }
        if self.controller_id.is_some() {
            ret += 1;
        } else if self.controller_uuid.is_some() {
            ret += 4;
        }
        if let Some(f) = &self.cif1_fields {
            ret += 1 + f.size_words();
        }
        if let Some(f) = &self.cif2_fields {
            ret += 1 + f.size_words();
        }
        if let Some(f) = &self.cif3_fields {
            ret += 1 + f.size_words();
        }
        if self.cif0.field_attributes_enabled() {
            ret += 1;
        }
        ret
    }
}

impl Cif0Manipulators for Command {
    fn cif0(&self) -> &Cif0 {
        &self.cif0
    }
    fn cif0_mut(&mut self) -> &mut Cif0 {
        &mut self.cif0
    }
    fn cif0_fields(&self) -> &Cif0Fields {
        &self.cif0_fields
    }
    fn cif0_fields_mut(&mut self) -> &mut Cif0Fields {
        &mut self.cif0_fields
    }
}

impl Cif1Manipulators for Command {
    fn cif0(&self) -> &Cif0 {
        &self.cif0
    }
    fn cif0_mut(&mut self) -> &mut Cif0 {
        &mut self.cif0
    }
    fn cif1(&self) -> Option<&Cif1> {
        self.cif1.as_ref()
    }
    fn cif1_mut(&mut self) -> &mut Option<Cif1> {
        &mut self.cif1
    }
    fn cif1_fields(&self) -> Option<&Cif1Fields> {
        self.cif1_fields.as_ref()
    }
    fn cif1_fields_mut(&mut self) -> &mut Option<Cif1Fields> {
        &mut self.cif1_fields
    }
}

impl Cif2Manipulators for Command {
    fn cif0(&self) -> &Cif0 {
        &self.cif0
    }
    fn cif0_mut(&mut self) -> &mut Cif0 {
        &mut self.cif0
    }
    fn cif2(&self) -> Option<&Cif2> {
        self.cif2.as_ref()
    }
    fn cif2_mut(&mut self) -> &mut Option<Cif2> {
        &mut self.cif2
    }
    fn cif2_fields(&self) -> Option<&Cif2Fields> {
        self.cif2_fields.as_ref()
    }
    fn cif2_fields_mut(&mut self) -> &mut Option<Cif2Fields> {
        &mut self.cif2_fields
    }
}

impl Cif3Manipulators for Command {
    fn cif0(&self) -> &Cif0 {
        &self.cif0
    }
    fn cif0_mut(&mut self) -> &mut Cif0 {
        &mut self.cif0
    }
    fn cif3(&self) -> Option<&Cif3> {
        self.cif3.as_ref()
    }
    fn cif3_mut(&mut self) -> &mut Option<Cif3> {
        &mut self.cif3
    }
    fn cif3_fields(&self) -> Option<&Cif3Fields> {
        self.cif3_fields.as_ref()
    }
    fn cif3_fields_mut(&mut self) -> &mut Option<Cif3Fields> {
        &mut self.cif3_fields
    }
}

impl TryFrom<Payload> for Command {
    type Error = Payload;

    fn try_from(value: Payload) -> Result<Self, Self::Error> {
        match value {
            Payload::Command(c) => Ok(c),
            a => Err(a),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cam)?;
        writeln!(f, "Message ID: {:x}", self.message_id)?;
        if let Some(cid) = self.controllee_id {
            writeln!(f, "Controllee ID: {:x}", cid)?;
        }
        if let Some(cuuid) = self.controllee_uuid {
            writeln!(f, "Controllee UUID: {:x}", cuuid)?;
        }
        if let Some(cid) = self.controller_id {
            writeln!(f, "Controller ID: {:x}", cid)?;
        }
        if let Some(cuuid) = self.controller_uuid {
            writeln!(f, "Controller UUID: {:x}", cuuid)?;
        }
        if let Some(cif1) = self.cif1 {
            write!(f, "{}", cif1)?;
        }
        if let Some(bw) = &self.bandwidth_hz() {
            writeln!(f, "Bandwidth: {} Hz", bw)?;
        }
        if let Some(rf_freq) = &self.rf_ref_freq_hz() {
            writeln!(f, "RF reference frequency: {} Hz", rf_freq)?;
        }
        if let Some(samp_rate) = &self.sample_rate_sps() {
            writeln!(f, "Sample rate: {} sps", samp_rate)?;
        }
        if let Some(device_id) = &self.device_id() {
            write!(f, "{}", device_id)?;
        }
        if let Some(spectrum) = self.spectrum() {
            write!(f, "{}", spectrum)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{ActionMode, ControlAckMode, IdFormat, Tsf, Tsi};

    #[test]
    fn create_command_packet() {
        let mut packet = Vrt::new_command_packet();
        packet.set_stream_id(Some(0xDEADBEEF));
        packet.set_integer_timestamp(Some(0), Tsi::Utc).unwrap();
        packet
            .set_fractional_timestamp(Some(0), Tsf::SampleCount)
            .unwrap();
        let command = packet.payload_mut().command_mut().unwrap();
        command.set_message_id(123);
        let mut cam = ControlAckMode::default();
        cam.enable_controllee();
        cam.enable_controller();
        cam.set_controllee_id_format(IdFormat::Id32bit);
        cam.set_controller_id_format(IdFormat::Uuid128bit);
        cam.set_action_mode(ActionMode::Execute);
        cam.set_partial_packet_impl_permitted();
        cam.set_warnings_permitted();
        cam.set_validation();
        cam.set_warning();
        cam.set_error();
        command.set_cam(cam);
        command.controllee_id = Some(123);
        command.controller_uuid = Some(321);

        assert_eq!(command.cif0.as_u32(), 0);
        command.set_rf_ref_freq_hz(Some(100e6));
        assert_eq!(command.cif0.as_u32(), 0x800_0000);
        command.set_bandwidth_hz(Some(8e6));
        assert_eq!(command.cif0.as_u32(), 0x2800_0000);
    }
}
