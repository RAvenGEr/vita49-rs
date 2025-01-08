// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
/*!
Data structures and methods related to context association lists
(ANSI/VITA-49.2-2017 section 9.13.2).
*/

use deku::prelude::*;

/// Base context association lists structure.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContextAssociationLists {
    w1: u32,
    w2: u32,
    #[deku(count = "((w1 >> 16) & 0x3FF)")]
    source_list: Vec<u32>,
    #[deku(count = "(w1 & 0x3FF)")]
    system_list: Vec<u32>,
    #[deku(count = "(w2 >> 16)")]
    vector_component_list: Vec<u32>,
    #[deku(count = "(w2 & 0x1FF)")]
    async_channel_list: Vec<u32>,
    #[deku(cond = "(w2 & (1 << 15) > 1)", count = "(w2 & 0x1FF)")]
    async_channel_tag_list: Vec<u32>,
}

impl ContextAssociationLists {
    /// Get the size of the lists in 32-bit words.
    pub fn size_words(&self) -> u16 {
        // Start with the 2 top words
        let mut ret = 2;
        ret += self.source_list.len();
        ret += self.system_list.len();
        ret += self.vector_component_list.len();
        ret += self.async_channel_list.len();
        ret += self.async_channel_tag_list.len();
        ret as u16
    }
}
