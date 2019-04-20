use bitflags::bitflags;

pub trait AttributeRegisterMarker {}

macro_rules! impl_marker_trait {
    ($name:ident) => {
        impl AttributeRegisterMarker for $name {}
    };
}

pub const WRITE_ADDRESS: u16 = 0x03C0;
pub const READ_ADDRESS: u16 = 0x03C1;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum InternalPaletteIndex {
    I0 = 0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    I11,
    I12,
    I13,
    I14,
    I15,
}

const fn values() -> [InternalPaletteIndex; 16] {
    use InternalPaletteIndex::*;
    [
        I0,
        I1,
        I2,
        I3,
        I4,
        I5,
        I6,
        I7,
        I8,
        I9,
        I10,
        I11,
        I12,
        I13,
        I14,
        I15,
    ]
}

impl InternalPaletteIndex {
    pub const VALUES: [InternalPaletteIndex; 16] = values();
}

impl_from_enum_for_u8!(InternalPaletteIndex);


declare_register_type!(AttributeAddressRegister);

impl AttributeAddressRegister {
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

declare_register_type!(AttributeModeControlRegister, AttributeModeControlRegisterFlags, 0x10);

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

declare_register_type!(OverscanColorRegister, 0x11);

impl OverscanColorRegister {
    register_value!(
        overscan_palette_address,
        set_overscan_palette_address,
        u8
    );
}

declare_register_type!(ColorPlaneEnableRegister, ColorPlaneEnableRegisterFlags, 0x12);

bitflags! {
    pub struct ColorPlaneEnableRegisterFlags: u8 {
        const PLANE_3 = 0b0000_1000;
        const PLANE_2 = 0b0000_0100;
        const PLANE_1 = 0b0000_0010;
        const PLANE_0 = 0b0000_0001;
    }
}

declare_register_type!(HorizontalPelPanningRegister, 0x13);

impl HorizontalPelPanningRegister {
    register_value!(
        /// A 4-bit value.
        horizontal_pel_panning,
        set_horizontal_pel_panning,
        0b0000_1111,
    );
}

declare_register_type!(ColorSelectRegister, ColorSelectRegisterFlags, 0x14);

bitflags! {
    pub struct ColorSelectRegisterFlags: u8 {
        const S_COLOR_7 = 0b0000_1000;
        const S_COLOR_6 = 0b0000_0100;
        const S_COLOR_5 = 0b0000_0010;
        const S_COLOR_4 = 0b0000_0001;
    }
}
