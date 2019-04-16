use bitflags::bitflags;
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{RegisterField, remove_bits, BitIndexU16, BitIndexU8, extract_bit_from_u8, extract_bit_from_u16};

pub const ADDRESS_REGISTER: u16 = 0x03CE;
pub const DATA_REGISTER: u16 = 0x03CF;


#[derive(Debug)]
pub struct RegisterIndex;

impl RegisterIndex {
    pub const SET_SLASH_RESET: u8 = 0x00;
    pub const ENABLE_SET_SLASH_RESET: u8 = 0x01;
    pub const COLOR_COMPARE: u8 = 0x02;
    pub const DATA_ROTATE: u8 = 0x03;
    pub const READ_MAP_SELECT: u8 = 0x04;
    pub const GRAPHICS_MODE: u8 = 0x05;
    pub const MISCELLANEOUS: u8 = 0x06;
    pub const COLOR_DO_NOT_CARE: u8 = 0x07;
    pub const BIT_MASK: u8 = 0x08;
}

declare_register_type!(SetSlashResetRegister, MapFlags);

bitflags! {
    pub struct MapFlags: u8 {
        const MAP_3 = 0b0000_1000;
        const MAP_2 = 0b0000_0100;
        const MAP_1 = 0b0000_0010;
        const MAP_0 = 0b0000_0001;
    }
}

declare_register_type!(EnableSetSlashResetRegister, MapFlags);

declare_register_type!(ColorCompareRegister, MapFlags);

declare_register_type!(DataRotateRegister);

impl DataRotateRegister {
    pub fn function_select(&self) -> DataFunction {
        DataFunction::from_register_value(self.0).unwrap()
    }

    pub fn set_function_select(&mut self, value: DataFunction) {
        value.update_register_value(&mut self.0)
    }

    register_value!(
        /// A 3-bit value.
        rotate_count,
        set_rotate_count,
        0b0000_0111,
    );
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum DataFunction {
    Unmodified = 0b0000_0000,
    And = 0b0000_1000,
    Or = 0b0001_0000,
    Xor = 0b0001_1000,
}

impl_from_enum_for_u8!(DataFunction);

impl RegisterField for DataFunction {
    const ALL_BITS_ON_MASK: u8 = 0b0001_1000;
}

declare_register_type!(ReadMapSelect);

impl ReadMapSelect {
    register_enum_with_unwrap!(
        map_select,
        set_map_select,
        MapSelect,
    );
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum MapSelect {
    Map0 = 0b0000_0000,
    Map1 = 0b0000_0001,
    Map2 = 0b0000_0010,
    Map3 = 0b0000_0011,
}

impl_from_enum_for_u8!(MapSelect);

impl RegisterField for MapSelect {
    const ALL_BITS_ON_MASK: u8 = 0b0000_0011;
}

declare_register_type!(GraphicsModeRegister, GraphicsModeRegisterFlags);

bitflags! {
    pub struct GraphicsModeRegisterFlags: u8 {
        const COLOR_MODE_256 = 0b0100_0000;
        const SHIFT_REGISTER_MODE = 0b0010_0000;
        const ODD_SLASH_EVEN = 0b0001_0000;
        const READ_MODE = 0b0000_1000;
    }
}

impl GraphicsModeRegister {
    register_enum_with_unwrap!(
        write_mode,
        set_write_mode,
        WriteMode,
    );
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum WriteMode {
    Mode0 = 0b0000_0000,
    Mode1 = 0b0000_0001,
    Mode2 = 0b0000_0010,
    Mode3 = 0b0000_0011,
}

impl_from_enum_for_u8!(WriteMode);

impl RegisterField for WriteMode {
    const ALL_BITS_ON_MASK: u8 = 0b0000_0011;
}
