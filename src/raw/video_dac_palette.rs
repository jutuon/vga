
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{UnknownValue, RegisterField, Register};


/// Read/Write
pub const PALETTE_ADDRESS_WRITE_MODE: u16 = 0x03C8;

/// Write
pub const PALETTE_ADDRESS_READ_MODE: u16 = 0x03C7;

/// Read
pub const DAC_STATE: u16 = 0x03C7;

/// Read/Write
pub const PALETTE_DATA: u16 = 0x03C9;

/// IBM reference says that this register is read only.
/// However, the reference also says the register is initialized to
/// to 0xFF when changing video mode so writing to the register
/// probably also works.
pub const PEL_MASK: u16 = 0x03C6;


declare_register_type!(PaletteAddressWriteModeRegister);

impl PaletteAddressWriteModeRegister {
    pub fn new(index: u8) -> Self {
        PaletteAddressWriteModeRegister(index)
    }

    register_value!(
        palette_address_write_mode,
        set_palette_address_write_mode,
        u8,
    );
}

declare_register_type!(PaletteAddressReadModeRegister);

impl PaletteAddressReadModeRegister {
    pub fn new(index: u8) -> Self {
        PaletteAddressReadModeRegister(index)
    }

    register_value!(
        palette_address_read_mode,
        set_palette_address_read_mode,
        u8,
    );
}

declare_register_type!(PaletteDataRegister);

impl PaletteDataRegister {
    register_value!(
        /// A 6-bit value.
        color_value,
        set_color_value,
        0b0011_1111,
    );
}

#[derive(Debug)]
pub struct PaletteColor{
    pub(crate) r: PaletteDataRegister,
    pub(crate) g: PaletteDataRegister,
    pub(crate) b: PaletteDataRegister,
}

impl Default for PaletteColor {
    fn default() -> Self {
        Self {
            r: PaletteDataRegister::from_register_value(0),
            g: PaletteDataRegister::from_register_value(0),
            b: PaletteDataRegister::from_register_value(0),
        }
    }
}

impl PaletteColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let mut value = Self::default();
        value.set_r(r);
        value.set_g(g);
        value.set_b(b);
        value
    }

    /// A 6-bit value.
    pub fn r(&self) -> u8 {
        self.r.color_value()
    }

    /// A 6-bit value.
    pub fn g(&self) -> u8 {
        self.g.color_value()
    }

    /// A 6-bit value.
    pub fn b(&self) -> u8 {
        self.b.color_value()
    }

    /// A 6-bit value.
    pub fn set_r(&mut self, value: u8) {
        self.r.set_color_value(value);
    }

    /// A 6-bit value.
    pub fn set_g(&mut self, value: u8) {
        self.g.set_color_value(value);
    }

    /// A 6-bit value.
    pub fn set_b(&mut self, value: u8) {
        self.b.set_color_value(value);
    }
}

declare_register_type!(DacStateRegister);

impl DacStateRegister {
    pub fn previous_dac_operation(&self) -> Result<PreviousDacOperation, UnknownValue> {
        PreviousDacOperation::from_register_value(self.0)
    }
}

declare_register_enum!(
    pub enum PreviousDacOperation {
        Read = 0b0000_0000,
        Write = 0b0000_0011,
    }
);

declare_register_type!(PelMaskRegister);

impl PelMaskRegister {
    register_value!(
        pel_mask,
        set_pel_mask,
        u8,
    );
}
