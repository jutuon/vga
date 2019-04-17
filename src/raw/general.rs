
use bitflags::bitflags;
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{RegisterField};

pub const READ_MISCELLANEOUS_OUTPUT: u16 = 0x03CC;
pub const READ_INPUT_STATUS_0: u16 = 0x03C2;
pub const READ_INPUT_STATUS_1_IO_SELECT_OFF: u16 = 0x03BA;
pub const READ_INPUT_STATUS_1_IO_SELECT_ON: u16 = 0x03DA;
pub const READ_FEATURE_CONTROL_REGISTER: u16 = 0x03CA;
pub const READ_VIDEO_SUBSYSTEM_ENABLE_REGISTER: u16 = 0x03C3;

pub const WRITE_MISCELLANEOUS_OUTPUT: u16 = 0x03C2;
pub const WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_OFF: u16 = 0x03BA;
pub const WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_ON: u16 = 0x03DA;
pub const WRITE_VIDEO_SUBSYSTEM_ENABLE_REGISTER: u16 = 0x03C3;


declare_register_type!(MiscellaneousOutputRegister, MiscellaneousOutputRegisterFlags);

impl MiscellaneousOutputRegister {
    register_enum!(
        vertical_size,
        set_vertical_size,
        VerticalSize
    );

    register_enum!(
        clock_select,
        set_clock_select,
        ClockSelect
    );
}

declare_register_enum!(
    pub enum VerticalSize {
        Lines400 = 0b0100_0000,
        Lines350 = 0b1000_0000,
        Lines480 = 0b1100_0000,
    }
);
declare_register_enum!(
    pub enum ClockSelect {
        /// 25.175 MHz
        Preset1 = 0b0000_0000,
        /// 28.322 MHz
        Preset2 = 0b0000_0100,
        ExternalClock = 0b0000_1000,
    }
);

bitflags! {
    pub struct MiscellaneousOutputRegisterFlags: u8 {
        const ENABLE_RAM = 0b0000_0010;
        const IO_ADDRESS_SELECT = 0b0000_0001;
    }
}

declare_register_type!(InputStatusRegister0);

impl InputStatusRegister0 {
    pub fn flags(&self) -> InputStatusRegister0Flags {
        InputStatusRegister0Flags::from_bits_truncate(self.0)
    }
}

bitflags! {
    pub struct InputStatusRegister0Flags: u8 {
        const CRT_INTERRUPT = 0b1000_0000;
        const SWITCH_SENSE_BIT = 0b0001_0000;
    }
}

declare_register_type!(InputStatusRegister1);

impl InputStatusRegister1 {
    pub fn flags(&self) -> InputStatusRegister1Flags {
        InputStatusRegister1Flags::from_bits_truncate(self.0)
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
