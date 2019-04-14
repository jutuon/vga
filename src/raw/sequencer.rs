

use bitflags::bitflags;
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{RegisterField};

pub const ADDRESS_REGISTER: u16 = 0x03C4;
pub const DATA_REGISTER: u16 = 0x03C5;

#[derive(Debug)]
pub struct RegisterIndex;

impl RegisterIndex {
    pub const RESET: u8 = 0x00;
    pub const CLOCKING_MODE: u8 = 0x01;
    pub const MAP_MASK: u8 = 0x02;
    pub const CHARACTER_MAP_SELECT: u8 = 0x03;
    pub const MEMORY_MODE: u8 = 0x04;
}

declare_register_type!(ResetRegister, ResetRegisterFlags);

bitflags! {
    pub struct ResetRegisterFlags: u8 {
        const SYNCHRONOUS_RESET = 0b0000_0010;
        const ASYNCHRONOUS_RESET = 0b0000_0001;
    }
}

declare_register_type!(ClockingModeRegister, ClockingModeRegisterFlags);

bitflags! {
    pub struct ClockingModeRegisterFlags: u8 {
        const SCREEN_OFF = 0b0010_0000;
        const SHIFT_4 = 0b0001_0000;
        const DOT_CLOCK = 0b0000_1000;
        const SHIFT_LOAD = 0b0000_0100;
        const DOT_CLOCKS_8_SLASH_9 = 0b0000_0001;
    }
}

declare_register_type!(MapMaskRegister, MapMaskRegisterFlags);

bitflags! {
    pub struct MapMaskRegisterFlags: u8 {
        const MAP_3 = 0b0000_1000;
        const MAP_2 = 0b0000_0100;
        const MAP_1 = 0b0000_0010;
        const MAP_0 = 0b0000_0001;
    }
}

declare_register_type!(CharacterMapSelectRegister);

impl CharacterMapSelectRegister {
    pub fn character_map_a_select(&self) -> CharacterMapASelect {
        CharacterMapASelect::from_register_value(self.0).unwrap()
    }

    pub fn character_map_b_select(&self) -> CharacterMapBSelect {
        CharacterMapBSelect::from_register_value(self.0).unwrap()
    }

    pub fn set_character_map_a_select(&mut self, value: CharacterMapASelect) {
        value.update_register_value(&mut self.0)
    }

    pub fn set_character_map_b_select(&mut self, value: CharacterMapASelect) {
        value.update_register_value(&mut self.0)
    }
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum CharacterMapASelect {
    Map0 = 0b0000_0000,
    Map1 = 0b0000_0100,
    Map2 = 0b0000_1000,
    Map3 = 0b0000_1100,
    Map4 = 0b0010_0000,
    Map5 = 0b0010_0100,
    Map6 = 0b0010_1000,
    Map7 = 0b0010_1100
}

impl_from_enum_for_u8!(CharacterMapASelect);

impl RegisterField for CharacterMapASelect {
    const ALL_BITS_ON_MASK: u8 = 0b0010_1100;
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum CharacterMapBSelect {
    Map0 = 0b0000_0000,
    Map1 = 0b0000_0001,
    Map2 = 0b0000_0010,
    Map3 = 0b0000_0011,
    Map4 = 0b0001_0000,
    Map5 = 0b0001_0001,
    Map6 = 0b0001_0010,
    Map7 = 0b0001_0011
}

impl_from_enum_for_u8!(CharacterMapBSelect);

impl RegisterField for CharacterMapBSelect {
    const ALL_BITS_ON_MASK: u8 = 0b0001_0011;
}

declare_register_type!(MemoryModeRegister, MemoryModeRegisterFlags);

bitflags! {
    pub struct MemoryModeRegisterFlags: u8 {
        const CHAIN_4 = 0b0000_1000;
        const ODD_SLASH_EVEN = 0b0000_0100;
        const EXTENDED_MEMORY = 0b0000_0010;
    }
}
