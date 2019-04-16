
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

declare_register_type!(HorizontalTotalRegister);

impl HorizontalTotalRegister {
    register_value!(
        horizontal_total,
        set_horizontal_total,
        u8
    );
}

declare_register_type!(HorizontalDisplayEnableEndRegister);

impl HorizontalDisplayEnableEndRegister {
    register_value!(
        horizontal_display_enable_end,
        set_horizontal_display_enable_end,
        u8
    );
}

declare_register_type!(StartHorizontalBlankingRegister);

impl StartHorizontalBlankingRegister {
    register_value!(
        start_horizontal_blanking,
        set_start_horizontal_blanking,
        u8
    );
}

declare_register_type!(EndHorizontalBlankingRegister);

impl EndHorizontalBlankingRegister {
    register_enum_with_unwrap!(
        skew_control,
        set_skew_control,
        SkewControl,
    );

    register_value!(
        /// Part 1/2 of a 6-bit end horizontal blanking value.
        end_blanking_bits_from_0_to_4,
        set_end_blanking_bits_from_0_to_4,
        0b0001_1111,
    );
}

declare_register_enum!(
    pub enum SkewControl {
        Zero = 0b0000_0000,
        One = 0b0010_0000,
        Two = 0b0100_0000,
        Three = 0b0110_0000,
    }
);

declare_register_type!(StartHorizontalRetracePulseRegister);

impl StartHorizontalRetracePulseRegister {
    register_value!(
        start_horizontal_retrace_pulse,
        set_start_horizontal_retrace_pulse,
        u8
    );
}

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

    register_enum_with_unwrap!(
        horizontal_retrace_delay,
        set_horizontal_retrace_delay,
        SkewControl,
    );

    register_value!(
        /// A 5-bit value.
        end_horizontal_retrace,
        set_end_horizontal_retrace,
        0b0001_1111,
    );
}

/// Part 1/2 of a 10-bit vertical total value.
declare_register_type!(VerticalTotalRegister);
impl VerticalTotalRegister {
    register_value!(
        vertical_total_bits_from_0_to_7,
        set_vertical_total_bits_from_0_to_7,
        u16
    );
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
    register_value!(
        /// A 5-bit value.
        starting_row_scan_count,
        set_starting_row_scan_count,
        0b0001_1111,
    );
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

    register_value!(
        /// A 5-bit value.
        maximum_scan_line,
        set_maximum_scan_line,
        0b0001_1111,
    );
}

declare_register_type!(CursorStartRegister);

impl CursorStartRegister {
    register_boolean!(
        cursor_off,
        set_cursor_off,
        0b0010_0000,
    );

    register_value!(
        /// A 5-bit value.
        row_scan_cursor_begins,
        set_row_scan_cursor_begins,
        0b0001_1111,
    );
}

declare_register_type!(CursorEndRegister);

impl CursorEndRegister {
    register_enum_with_unwrap!(
        cursor_skew_control,
        set_cursor_skew_control,
        SkewControl
    );

    register_value!(
        row_scan_cursor_ends,
        set_row_scan_cursor_ends,
        0b0001_1111,
    );
}

declare_register_type!(StartAddressHighRegister);

impl StartAddressHighRegister {
    pub fn start_address_bits_from_8_to_15(&self) -> u16 {
        (self.0 as u16) << 8
    }

    pub fn set_start_address_bits_from_8_to_15(&mut self, value: u16) {
        self.0 = (value >> 8) as u8;
    }
}

declare_register_type!(StartAddressLowRegister);

impl StartAddressLowRegister {
    register_value!(
        start_address_bits_from_0_to_7,
        set_start_address_bits_from_0_to_7,
        u16
    );
}

declare_register_type!(CursorLocationHighRegister);

impl CursorLocationHighRegister {
    pub fn cursor_location_bits_from_8_to_15(&self) -> u16 {
        (self.0 as u16) << 8
    }

    pub fn set_cursor_location_bits_from_8_to_15(&mut self, value: u16) {
        self.0 = (value >> 8) as u8;
    }
}

declare_register_type!(CursorLocationLowRegister);

impl CursorLocationLowRegister {
    register_value!(
        cursor_location_bits_from_0_to_7,
        set_cursor_location_bits_from_0_to_7,
        u16
    );
}

declare_register_type!(VerticalRetraceStartRegister);

impl VerticalRetraceStartRegister {
    register_value!(
        /// A 9-bit value.
        vertical_retrace_start_bits_from_0_to_7,
        set_vertical_retrace_start_bits_from_0_to_7,
        u16
    );
}

declare_register_type!(VerticalRetraceEndRegister, VerticalRetraceEndRegisterFlags);

impl VerticalRetraceEndRegister {
    register_value!(
        /// A 4-bit value.
        vertical_retrace_end,
        set_vertical_retrace_end,
        0b0000_1111,
    );
}

bitflags! {
    pub struct VerticalRetraceEndRegisterFlags: u8 {
        const PROTECT_REGISTERS_FROM_0_TO_7 = 0b1000_0000;
        const SELECT_5_REFRESH_CYCLES = 0b0100_0000;
        const ENABLE_VERTICAL_INTERRUPT = 0b0010_0000;
        const CLEAR_VERTICAL_INTERRUPT = 0b0001_0000;
    }
}

declare_register_type!(VerticalDisplayEnableEndRegister);

impl VerticalDisplayEnableEndRegister {
    register_value!(
        /// Part 1/2 of a 10-bit value.
        vertical_display_enable_end_bits_from_0_to_7,
        set_vertical_display_enable_end_bits_from_0_to_7,
        u16
    );
}

declare_register_type!(OffsetRegister);

impl OffsetRegister {
    register_value!(
        logical_line_width_of_the_screen,
        set_logical_line_width_of_the_screen,
        u8
    );
}

declare_register_type!(UnderlineLocationRegister, UnderlineLocationRegisterFlags);

bitflags! {
    pub struct UnderlineLocationRegisterFlags: u8 {
        const DOUBLE_WORD_MODE = 0b0100_0000;
        const COUNT_BY_4 = 0b0010_0000;
    }
}

declare_register_type!(StartVerticalBlankingRegister);

impl StartVerticalBlankingRegister {
    register_value!(
        /// Part 1/3 of a 10-bit value.
        start_vertical_blanking_bits_from_0_to_7,
        set_start_vertical_blanking_bits_from_0_to_7,
        u16
    );
}

declare_register_type!(EndVerticalBlanking);

impl EndVerticalBlanking {
    register_value!(
        end_vertical_blanking,
        set_end_vertical_blanking,
        u8
    );
}

declare_register_type!(CrtModeControlRegister, CrtModeControlRegisterFlags);

bitflags! {
    pub struct CrtModeControlRegisterFlags: u8 {
        const HARDWARE_RESET = 0b1000_0000;
        const WORD_SLASH_BYTE_MODE = 0b0100_0000;
        const ADDRESS_WRAP = 0b0010_0000;
        const COUNT_BY_TWO = 0b0000_1000;
        const HORIZONTAL_RETRACE_SELECT = 0b0000_0100;
        const SELECT_ROW_SCAN_COUNTER = 0b0000_0010;
        const CMS_0 = 0b0000_0001;
    }
}

declare_register_type!(LineCompareRegister);

impl LineCompareRegister {
    register_value!(
        /// Part 1/3 of a 10-bit value.
        line_compare_target_bits_from_0_to_7,
        set_line_compare_target_bits_from_0_to_7,
        u16
    );
}
