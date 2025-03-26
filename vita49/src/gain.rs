// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Data structures and methods related to the gain format
(ANSI/VITA-49.2-2017 section 9.5.3).

"In RF equipment such as tuners and receivers, the total gain
of the equipment is typically distributed to allow tradeoffs
between noise power and linearity. For such equipment, Stage 1
Gain conveys the front-end or RF gain, and Stage 2 Gain conveys
the back-end or IF gain. For equipment that does not require gain
distribution, Stage 1 Gain provides the gain of the device, and
Stage 2 Gain is set to zero."
*/

use deku::prelude::*;
use fixed::{types::extra::U7, FixedI16};
use std::fmt;

/// Base gain data structure.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite,
)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gain(i32);

impl Gain {
    /// Create a new `Gain` object given stage 1 and 2 gain in dB.
    pub fn new(stage_1_gain_db: f32, stage_2_gain_db: f32) -> Gain {
        let s1 = FixedI16::<U7>::from_num(stage_1_gain_db).to_bits() as i32;
        let s2 = FixedI16::<U7>::from_num(stage_2_gain_db).to_bits() as i32;
        Gain((s2 << 16) | s1)
    }

    /// Gets the size of the gain structure in 32-bit words.
    pub fn size_words(&self) -> u16 {
        (std::mem::size_of_val(&self.0) / std::mem::size_of::<u32>()) as u16
    }

    /// Gets stage 1 gain (dB)
    pub fn stage_1_gain_db(&self) -> f32 {
        let s1 = (self.0 & 0xFFFF) as i16;
        FixedI16::<U7>::from_bits(s1).to_num()
    }

    /// Sets stage 1 gain (dB)
    pub fn set_stage_1_gain_db(&mut self, stage_1_gain_db: f32) {
        let s1 = FixedI16::<U7>::from_num(stage_1_gain_db).to_bits() as i32;
        self.0 = (self.0 & (0xFFFF_0000u32 as i32)) & s1
    }

    /// Gets stage 2 gain (dB)
    pub fn stage_2_gain_db(&self) -> f32 {
        let s2 = ((self.0 >> 16) & 0xFFFF) as i16;
        FixedI16::<U7>::from_bits(s2).to_num()
    }

    /// Sets stage 2 gain (dB)
    pub fn set_stage_2_gain_db(&mut self, stage_2_gain_db: f32) {
        let s2 = FixedI16::<U7>::from_num(stage_2_gain_db).to_bits() as i32;
        self.0 = (self.0 & 0x0000_FFFF) & (s2 << 16)
    }
}

impl fmt::Display for Gain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Stage 1: {} dB, Stage 2: {} dB",
            self.stage_1_gain_db(),
            self.stage_2_gain_db()
        )
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::Gain;

    #[test]
    fn manipulate_gain() {
        let _ = env_logger::builder().is_test(true).try_init();
        use crate::prelude::*;
        let mut packet = Vrt::new_context_packet();
        let context = packet.payload_mut().context_mut().unwrap();
        let s1: f32 = 25.2;
        let s2: f32 = 0.23;
        context.set_gain(Some(Gain::new(s1, s2)));
        assert_relative_eq!(
            context.gain().unwrap().stage_1_gain_db(),
            s1,
            max_relative = 0.1
        );
        assert_relative_eq!(
            context.gain().unwrap().stage_2_gain_db(),
            s2,
            max_relative = 0.1
        );
    }
}
