use bitflags::bitflags;

pub const WRITE_ADDRESS: u16 = 0x03C0;
pub const READ_ADDRESS: u16 = 0x03C1;

#[derive(Debug)]
pub struct RegisterIndex;

impl RegisterIndex {
    pub const INTERNAL_PALETTE: [u8; 16] = [
        0x0,
        0x1,
        0x2,
        0x3,
        0x4,
        0x5,
        0x6,
        0x7,
        0x8,
        0x9,
        0xA,
        0xB,
        0xC,
        0xD,
        0xE,
        0xF,
    ];
    pub const ATTRIBUTE_MODE_CONTROL: u8 = 0x10;
    pub const OVERSCAN_COLOR: u8 = 0x11;
    pub const COLOR_PLANE_ENABLE: u8 = 0x12;
    pub const HORIZONTAL_PEL_PANNING: u8 = 0x13;
    pub const COLOR_SELECT: u8 = 0x14;
}


declare_register_type!(AddressRegister);

impl AddressRegister {
    register_boolean!(
        internal_palette_address_source,
        set_internal_palette_address_source,
        0b0010_0000,
    );

    register_value!(
        /// A 5-bit value.
        register_index,
        set_register_index,
        0b0001_1111,
    );
}

declare_register_type!(InternalPaletteRegister);

impl InternalPaletteRegister {
    register_value!(
        /// A 6-bit value.
        palette_address,
        set_palette_address,
        0b0011_1111,
    );
}

declare_register_type!(AttributeModeControlRegister, AttributeModeControlRegisterFlags);

bitflags! {
    pub struct AttributeModeControlRegisterFlags: u8 {
        const P5_P4_SELECT = 0b1000_0000;
        const PEL_WIDTH = 0b0100_0000;
        const PEL_PANNING_COMPATIBILITY = 0b0010_0000;
        const ENABLE_BLINK = 0b0000_1000;
        const ENABLE_LINE_GRAPHICS_CHARACTER_CODE = 0b0000_0100;
        const MONO_EMULATION = 0b0000_0010;
        const GRAPHICS_MODE = 0b0000_0001;
    }
}

declare_register_type!(OverscanColorRegister);

impl OverscanColorRegister {
    register_value!(
        overscan_palette_address,
        set_overscan_palette_address,
        u8
    );
}

declare_register_type!(ColorPlaneEnableRegister, ColorPlaneEnableRegisterFlags);

bitflags! {
    pub struct ColorPlaneEnableRegisterFlags: u8 {
        const PLANE_3 = 0b0000_1000;
        const PLANE_2 = 0b0000_0100;
        const PLANE_1 = 0b0000_0010;
        const PLANE_0 = 0b0000_0001;
    }
}

declare_register_type!(HorizontalPelPanningRegister);

impl HorizontalPelPanningRegister {
    register_value!(
        /// A 4-bit value.
        horizontal_pel_panning,
        set_horizontal_pel_panning,
        0b0000_1111,
    );
}

declare_register_type!(ColorSelectRegister, ColorSelectRegisterFlags);

bitflags! {
    pub struct ColorSelectRegisterFlags: u8 {
        const S_COLOR_7 = 0b0000_1000;
        const S_COLOR_6 = 0b0000_0100;
        const S_COLOR_5 = 0b0000_0010;
        const S_COLOR_4 = 0b0000_0001;
    }
}
