
use bitflags::bitflags;
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{UnknownValue, RegisterField};

#[derive(Debug)]
pub struct RegisterReadAddress;

impl RegisterReadAddress {
    pub const MISCELLANEOUS_OUTPUT: u16 = 0x03CC;
    pub const INPUT_STATUS_0: u16 = 0x03C2;
    pub const INPUT_STATUS_1_IO_SELECT_OFF: u16 = 0x03BA;
    pub const INPUT_STATUS_1_IO_SELECT_ON: u16 = 0x03DA;
    pub const FEATURE_CONTROL_REGISTER: u16 = 0x03CA;
    pub const VIDEO_SUBSYSTEM_ENABLE_REGISTER: u16 = 0x03C3;
}

#[derive(Debug)]
pub struct RegisterWriteAddress;

impl RegisterWriteAddress {
    pub const MISCELLANEOUS_OUTPUT: u16 = 0x03C2;
    pub const FEATURE_CONTROL_REGISTER_IO_SELECT_OFF: u16 = 0x03BA;
    pub const FEATURE_CONTROL_REGISTER_IO_SELECT_ON: u16 = 0x03DA;
    pub const VIDEO_SUBSYSTEM_ENABLE_REGISTER: u16 = 0x03C3;
}

declare_register_type!(MiscellaneousOutputRegister, MiscellaneousOutputRegisterFlags);

impl MiscellaneousOutputRegister {
    pub fn vertical_size(&self) -> Result<VerticalSize, UnknownValue> {
        VerticalSize::from_register_value(self.0)
    }

    pub fn set_vertical_size(&mut self, value: VerticalSize) {
        value.update_register_value(&mut self.0)
    }

    pub fn clock_select(&self) -> Result<ClockSelect, UnknownValue> {
        ClockSelect::from_register_value(self.0)
    }

    pub fn set_clock_select(&mut self, value: ClockSelect) {
        value.update_register_value(&mut self.0);
    }
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum VerticalSize {
    Lines400 = 0b0100_0000,
    Lines350 = 0b1000_0000,
    Lines480 = 0b1100_0000,
}

impl_from_enum_for_u8!(VerticalSize);

impl RegisterField for VerticalSize {
    const ALL_BITS_ON_MASK: u8 = 0b1100_0000;
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum ClockSelect {
    /// 25.175 MHz
    Preset1 = 0b0000_0000,
    /// 28.322 MHz
    Preset2 = 0b0000_0100,
    ExternalClock = 0b0000_1000,
}

impl_from_enum_for_u8!(ClockSelect);

impl RegisterField for ClockSelect {
    const ALL_BITS_ON_MASK: u8 = 0b0000_1100;
}


bitflags! {
    pub struct MiscellaneousOutputRegisterFlags: u8 {
        const ENABLE_RAM = 0b0000_0010;
        const IO_ADDRESS_SELECT = 0b0000_0001;
    }
}

bitflags! {
    pub struct InputStatusRegister0Flags: u8 {
        const CRT_INTERRUPT = 0b1000_0000;
        const SWITCH_SENSE_BIT = 0b0001_0000;
    }
}

bitflags! {
    pub struct InputStatusRegister1Flags: u8 {
        const VERTICAL_RETRACE = 0b0000_1000;
        const DISPLAY_ENABLE = 0b0000_0001;
    }
}

declare_register_type!(VideoSubsystemEnableRegister);

impl VideoSubsystemEnableRegister {
    register_boolean!(
        video_subsystem_enable,
        set_video_subsystem_enable,
        0b0000_0001,
    );
}
