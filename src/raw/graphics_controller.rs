use bitflags::bitflags;
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{RegisterField};

pub const ADDRESS_REGISTER: u16 = 0x03CE;
pub const DATA_REGISTER: u16 = 0x03CF;

declare_register_type!(GraphicsAddressRegister);

impl GraphicsAddressRegister {
    register_value!(
        graphics_address,
        set_graphics_address,
        0b0000_1111,
    );
}

declare_register_type!(SetSlashResetRegister, MapFlags, 0x00);

bitflags! {
    pub struct MapFlags: u8 {
        const MAP_3 = 0b0000_1000;
        const MAP_2 = 0b0000_0100;
        const MAP_1 = 0b0000_0010;
        const MAP_0 = 0b0000_0001;
    }
}

declare_register_type!(EnableSetSlashResetRegister, MapFlags, 0x01);

declare_register_type!(ColorCompareRegister, MapFlags, 0x02);

declare_register_type!(DataRotateRegister, 0x03);

impl DataRotateRegister {
    register_enum_with_unwrap!(
        function_select,
        set_function_select,
        DataFunction,
    );

    register_value!(
        /// A 3-bit value.
        rotate_count,
        set_rotate_count,
        0b0000_0111,
    );
}

declare_register_enum!(
    pub enum DataFunction {
        Unmodified = 0b0000_0000,
        And = 0b0000_1000,
        Or = 0b0001_0000,
        Xor = 0b0001_1000,
    }
);

declare_register_type!(ReadMapSelectRegister, 0x04);

impl ReadMapSelectRegister {
    register_enum_with_unwrap!(
        map_select,
        set_map_select,
        MapSelect,
    );
}

declare_register_enum!(
    pub enum MapSelect {
        Map0 = 0b0000_0000,
        Map1 = 0b0000_0001,
        Map2 = 0b0000_0010,
        Map3 = 0b0000_0011,
    }
);

declare_register_type!(GraphicsModeRegister, GraphicsModeRegisterFlags, 0x05);

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

declare_register_enum!(
    pub enum WriteMode {
        Mode0 = 0b0000_0000,
        Mode1 = 0b0000_0001,
        Mode2 = 0b0000_0010,
        Mode3 = 0b0000_0011,
    }
);

declare_register_type!(MiscellaneousRegister, MiscellaneousRegisterFlags, 0x06);

impl MiscellaneousRegister {
    register_enum_with_unwrap!(
        addressing_assignment,
        set_addressing_assignment,
        AddressingAssignment
    );
}

bitflags! {
    pub struct MiscellaneousRegisterFlags: u8 {
        const ODD_SLASH_EVEN = 0b0000_0010;
        const GRAPHICS_MODE = 0b0000_0001;
    }
}

declare_register_enum!(
    pub enum AddressingAssignment {
        /// 0xA0000 for 128 KiB
        Mode0 = 0b0000_0000,
        /// 0xA0000 for 64 KiB
        Mode1 = 0b0000_0100,
        /// 0xB0000 for 32 KiB
        Mode2 = 0b0000_1000,
        /// 0xB8000 for 32 KiB
        Mode3 = 0b0000_1100,
    }
);


declare_register_type!(ColorDoNotCareRegister, MapFlags, 0x07);

declare_register_type!(BitMaskRegister, 0x08);

impl BitMaskRegister {
    register_value!(
        bit_mask,
        set_bit_mask,
        u8
    );
}
