
use bitflags::bitflags;
use enum_tryfrom_derive::TryFromPrimitive;

use core::convert::TryFrom;

use super::{RegisterField, remove_bits, BitIndexU16, BitIndexU8, extract_bit_from_u8, extract_bit_from_u16};

pub const ADDRESS_REGISTER_IO_SELECT_OFF: u16 = 0x03B4;
pub const ADDRESS_REGISTER_IO_SELECT_ON: u16 = 0x03D4;
pub const DATA_REGISTER_IO_SELECT_OFF: u16 = 0x03B5;
pub const DATA_REGISTER_IO_SELECT_ON: u16 = 0x03D5;


#[derive(Debug)]
pub struct RegisterIndex;

impl RegisterIndex {
    pub const HORIZONTAL_TOTAL: u8 = 0x00;
    pub const HORIZONTAL_DISPLAY_ENABLE_END: u8 = 0x01;
    pub const START_HORIZONTAL_BLANKING: u8 = 0x02;
    pub const END_HORIZONTAL_BLANKING: u8 = 0x03;
    pub const START_HORIZONTAL_RETRACE_PULSE: u8 = 0x04;
    pub const END_HORIZONTAL_RETRACE: u8 = 0x05;
    pub const VERTICAL_TOTAL: u8 = 0x06;
    pub const OVERFLOW: u8 = 0x07;
    pub const PRESET_ROW_SCAN: u8 = 0x08;
    pub const MAXIMUM_SCAN_LINE: u8 = 0x09;
    pub const CURSOR_START: u8 = 0x0A;
    pub const CURSOR_END: u8 = 0x0B;
    pub const START_ADDRESS_HIGH: u8 = 0x0C;
    pub const START_ADDRESS_LOW: u8 = 0x0D;
    pub const CURSOR_LOCATION_HIGH: u8 = 0x0E;
    pub const CURSOR_LOCATION_LOW: u8 = 0x0F;
    pub const VERTICAL_RETRACE_START: u8 = 0x10;
    pub const VERTICAL_RETRACE_END: u8 = 0x11;
    pub const VERTICAL_DISPLAY_ENABLE_END: u8 = 0x12;
    pub const OFFSET: u8 = 0x13;
    pub const UNDERLINE_LOCATION: u8 = 0x14;
    pub const START_VERTICAL_BLANKING: u8 = 0x15;
    pub const END_VERTICAL_BLANKING: u8 = 0x16;
    pub const CRT_MODE_CONTROL: u8 = 0x17;
    pub const LINE_COMPARE: u8 = 0x18;
}

#[derive(Debug)]
pub struct HorizontalTotalRegister(pub u8);

#[derive(Debug)]
pub struct HorizontalDisplayEnableEndRegister(pub u8);

#[derive(Debug)]
pub struct StartHorizontalBlankingRegister(pub u8);

declare_register_type!(EndHorizontalBlankingRegister);

impl EndHorizontalBlankingRegister {
    pub fn skew_control(&self) -> SkewControl {
        SkewControl::from_register_value(self.0).unwrap()
    }

    pub fn set_skew_control(&mut self, value: SkewControl) {
        value.update_register_value(&mut self.0)
    }

    const END_BLANKING_MASK: u8 = 0b0001_1111;

    /// Part 1/2 of a 6-bit end horizontal blanking value.
    pub fn end_blanking_bits_from_0_to_4(&self) -> u8 {
        self.0 & Self::END_BLANKING_MASK
    }

    /// Set part 1/2 of a 6-bit end horizontal blanking value.
    pub fn set_end_blanking_bits_from_0_to_4(&mut self, value: u8) {
        remove_bits(&mut self.0, Self::END_BLANKING_MASK);
        self.0 |= value & Self::END_BLANKING_MASK;
    }
}

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
#[TryFromPrimitiveType="u8"]
pub enum SkewControl {
    Zero = 0b0000_0000,
    One = 0b0010_0000,
    Two = 0b0100_0000,
    Three = 0b0110_0000,
}

impl_from_enum_for_u8!(SkewControl);

impl RegisterField for SkewControl {
    const ALL_BITS_ON_MASK: u8 = 0b0110_0000;
}

#[derive(Debug)]
pub struct StartHorizontalRetracePulseRegister(pub u8);

declare_register_type!(EndHorizontalRetraceRegister);

impl EndHorizontalRetraceRegister {
    const END_BLANKING_MASK: u8 = 0b1000_0000;

    /// Part 2/2 of a 6-bit end horizontal blanking value.
    pub fn end_blanking_bit_5(&self) -> u8 {
        (self.0 & Self::END_BLANKING_MASK) >> 2
    }

    /// Set part 2/2 of a 6-bit end horizontal blanking value.
    pub fn set_end_blanking_bit_5(&mut self, value: u8) {
        remove_bits(&mut self.0, Self::END_BLANKING_MASK);
        self.0 |= (value << 2) & Self::END_BLANKING_MASK;
    }

    pub fn horizontal_retrace_delay(&self) -> SkewControl {
        SkewControl::from_register_value(self.0).unwrap()
    }

    pub fn set_horizontal_retrace_delay(&mut self, value: SkewControl) {
        value.update_register_value(&mut self.0)
    }

    const END_HORIZONTAL_RETRACE_MASK: u8 = 0b0001_1111;

    /// A 5-bit value.
    pub fn end_horizontal_retrace(&mut self) -> u8 {
        self.0 & Self::END_HORIZONTAL_RETRACE_MASK
    }

    /// A 5-bit value.
    pub fn set_end_horizontal_retrace(&mut self, value: u8) {
        remove_bits(&mut self.0, Self::END_HORIZONTAL_RETRACE_MASK);
        self.0 |= value & Self::END_HORIZONTAL_RETRACE_MASK;
    }
}

/// Part 1/2 of a 10-bit vertical total value.
declare_register_type!(VerticalTotalRegister);
impl VerticalTotalRegister {
    pub fn vertical_total_bits_from_0_to_7(&self) -> u16 {
        self.0 as u16
    }

    pub fn set_vertical_total_bits_from_0_to_7(&mut self, value: u16) {
        self.0 = value as u8;
    }
}

declare_register_type!(OverflowRegister);

impl OverflowRegister {
    /// Part 2/2 of a 10-bit value.
    pub fn vertical_retrace_start_bits_8_and_9(&self) -> u16 {
        let bit9 = extract_bit_from_u8(self.0, BitIndexU8::I7, BitIndexU16::I9);
        let bit8 = extract_bit_from_u8(self.0, BitIndexU8::I2, BitIndexU16::I8);
        bit9 | bit8
    }

    /// Set part 2/2 of a 10-bit value.
    pub fn set_vertical_retrace_start_bits_8_and_9(&mut self, value: u16) {
        let bit9 = extract_bit_from_u16(value, BitIndexU16::I9, BitIndexU8::I7);
        let bit8 = extract_bit_from_u16(value, BitIndexU16::I8, BitIndexU8::I2);

        remove_bits(&mut self.0, 1 << 7 | 1 << 2);
        self.0 |= bit9 | bit8;
    }

    /// Part 2/2 of a 10-bit value.
    pub fn vertical_display_enable_end_bits_8_and_9(&self) -> u16 {
        let bit9 = extract_bit_from_u8(self.0, BitIndexU8::I6, BitIndexU16::I9);
        let bit8 = extract_bit_from_u8(self.0, BitIndexU8::I1, BitIndexU16::I8);
        bit9 | bit8
    }

    /// Set part 2/2 of a 10-bit value.
    pub fn set_vertical_display_enable_end_bits_8_and_9(&mut self, value: u16) {
        let bit9 = extract_bit_from_u16(value, BitIndexU16::I9, BitIndexU8::I6);
        let bit8 = extract_bit_from_u16(value, BitIndexU16::I8, BitIndexU8::I1);

        remove_bits(&mut self.0, 1 << 6 | 1 << 1);
        self.0 |= bit9 | bit8;
    }

    /// Part 2/2 of a 10-bit value.
    pub fn vertical_total_bits_8_and_9(&self) -> u16 {
        let bit9 = extract_bit_from_u8(self.0, BitIndexU8::I5, BitIndexU16::I9);
        let bit8 = extract_bit_from_u8(self.0, BitIndexU8::I0, BitIndexU16::I8);
        bit9 | bit8
    }

    /// Set part 2/2 of a 10-bit value.
    pub fn set_vertical_total_bits_8_and_9(&mut self, value: u16) {
        let bit9 = extract_bit_from_u16(value, BitIndexU16::I9, BitIndexU8::I5);
        let bit8 = extract_bit_from_u16(value, BitIndexU16::I8, BitIndexU8::I0);

        remove_bits(&mut self.0, 1 << 5 | 1);
        self.0 |= bit9 | bit8;
    }

    /// Part 2/3 of a 10-bit value.
    pub fn line_compare_one_bit_8(&self) -> u16 {
        extract_bit_from_u8(self.0, BitIndexU8::I4, BitIndexU16::I8)
    }

    /// Set part 2/3 of a 10-bit value.
    pub fn set_line_compare_bit_8(&mut self, value: u16) {
        remove_bits(&mut self.0, 1 << 4);
        self.0 |= extract_bit_from_u16(value, BitIndexU16::I8, BitIndexU8::I4);
    }

    /// Part 2/3 of a 10-bit value.
    pub fn vertical_blanking_start_bit_8(&self) -> u16 {
        extract_bit_from_u8(self.0, BitIndexU8::I3, BitIndexU16::I8)
    }

    /// Set part 2/3 of a 10-bit value.
    pub fn set_vertical_blanking_start_bit_8(&mut self, value: u16) {
        remove_bits(&mut self.0, 1 << 3);
        self.0 |= extract_bit_from_u16(value, BitIndexU16::I0, BitIndexU8::I3);
    }
}

declare_register_type!(PresetRowScanRegister, PresetRowScanRegisterFlags);

impl PresetRowScanRegister {
    const STARTING_ROW_SCAN_COUNT_MASK: u8 = 0b0001_1111;

    /// A 5-bit value.
    pub fn starting_row_scan_count(&self) -> u8 {
        self.0 & Self::STARTING_ROW_SCAN_COUNT_MASK
    }

    /// A 5-bit value.
    pub fn set_starting_row_scan_count(&mut self, value: u8) {
        remove_bits(&mut self.0, Self::STARTING_ROW_SCAN_COUNT_MASK);
        self.0 |= value & Self::STARTING_ROW_SCAN_COUNT_MASK;
    }
}

bitflags! {
    pub struct PresetRowScanRegisterFlags: u8 {
        const BYTE_PANNING_1 = 0b0100_0000;
        const BYTE_PANNING_2 = 0b0010_0000;
    }
}

declare_register_type!(MaximumScanLineRegister);

impl MaximumScanLineRegister {
    register_boolean!(
        line_conversion_200_to_400,
        set_line_conversion_200_to_400,
        0b1000_0000,
    );

    /// Part 3/3 of a 10-bit value.
    pub fn line_compare_bit_9(&self) -> u16 {
        extract_bit_from_u8(self.0, BitIndexU8::I6, BitIndexU16::I9)
    }

    /// Set part 3/3 of a 10-bit value.
    pub fn set_line_compare_bit_9(&mut self, value: u16) {
        remove_bits(&mut self.0, 1 << 6);
        self.0 |= extract_bit_from_u16(value, BitIndexU16::I9, BitIndexU8::I6);
    }

    /// Part 3/3 of a 10-bit value.
    pub fn start_vertical_blanking_bit_9(&self) -> u16 {
        extract_bit_from_u8(self.0, BitIndexU8::I5, BitIndexU16::I9)
    }

    /// Set part 3/3 of a 10-bit value.
    pub fn set_start_vertical_blanking_bit_9(&mut self, value: u16) {
        remove_bits(&mut self.0, 1 << 5);
        self.0 |= extract_bit_from_u16(value, BitIndexU16::I9, BitIndexU8::I5);
    }

    simple_register_value!(
        maximum_scan_line,
        set_maximum_scan_line,
        0b0001_1111,
        "A 5-bit value."
    );
}

declare_register_type!(CursorStartRegister);

impl CursorStartRegister {
    register_boolean!(
        cursor_off,
        set_cursor_off,
        0b0010_0000,
    );

    simple_register_value!(
        row_scan_cursor_begins,
        set_row_scan_cursor_begins,
        0b0001_1111,
        "A 5-bit value."
    );
}
