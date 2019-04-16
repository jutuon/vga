
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{UnknownValue, RegisterField};


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


declare_register_type!(PaletteAddress);

impl PaletteAddress {
    register_value!(
        palette_address,
        set_palette_address,
        u8,
    );
}

declare_register_type!(PaletteData);

impl PaletteData {
    register_value!(
        /// A 6-bit value.
        color_value,
        set_color_value,
        0b0011_1111,
    );
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
