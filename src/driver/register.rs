
use crate::io::PortIo;

use crate::raw::{
    RegisterWithIndex,
    Register,
    general::*,
    sequencer::*,
    crt_controller::*,
    graphics_controller::*,
    attribute_controller::*,
    video_dac_palette::*,
};

#[derive(Debug)]
pub struct RegisterHandler<T: PortIo>(T);

impl_port_io_available!(<T: PortIo> RegisterHandler<T>);

macro_rules! read_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $register_type:ident, $port:expr $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> $register_type {
            let raw = self.0.read($port);
            $register_type::from_register_value(raw)
        }
    };
}

macro_rules! write_register {
    ($( #[doc=$text:literal] )* $write_method_name:ident, $register_type:ident, $port:expr $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $write_method_name(&mut self, value: $register_type) {
            self.0.write($port, value.value());
        }
    };
}

macro_rules! read_write_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $write_method_name:ident, $register_type:ident, $port:expr $(,)?) => {
        read_register!( $( #[doc=$text] )* $read_method_name, $register_type, $port );
        write_register!( $( #[doc=$text] )* $write_method_name, $register_type, $port );
    };
}

macro_rules! sequencer_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $write_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> $register_type {
            let mut address = self.read_sequencer_address();
            address.set_sequencer_address($register_type::INDEX);
            self.write_sequencer_address(address);

            let raw = self.0.read(port!(T::SequencerPorts::DATA_REGISTER));
            $register_type::from_register_value(raw)
        }

        $(
            #[doc=$text]
        )*
        pub fn $write_method_name(&mut self, value: $register_type) {
            let mut address = self.read_sequencer_address();
            address.set_sequencer_address($register_type::INDEX);
            self.write_sequencer_address(address);

            self.0.write(port!(T::SequencerPorts::DATA_REGISTER), value.value());
        }
    };
}

macro_rules! crt_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $write_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> $register_type {
            let mut address = self.read_crt_address();
            address.set_crt_address($register_type::INDEX);
            self.write_crt_address(address);

            let port = if self.io_select_address_enabled() {
                port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_ON)
            } else {
                port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_OFF)
            };

            let raw = self.0.read(port);
            $register_type::from_register_value(raw)
        }

        $(
            #[doc=$text]
        )*
        pub fn $write_method_name(&mut self, value: $register_type) {
            let mut address = self.read_crt_address();
            address.set_crt_address($register_type::INDEX);
            self.write_crt_address(address);

            let port = if self.io_select_address_enabled() {
                port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_ON)
            } else {
                port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_OFF)
            };

            self.0.write(port, value.value());
        }
    };
}

impl <T: PortIo> RegisterHandler<T> {
    pub fn io_select_address_enabled(&mut self) -> bool {
        self.read_miscellaneous_output().flags().contains(MiscellaneousOutputRegisterFlags::IO_ADDRESS_SELECT)
    }

    read_register!(
        read_miscellaneous_output,
        MiscellaneousOutputRegister,
        port!(T::GeneralPorts::READ_MISCELLANEOUS_OUTPUT),
    );

    write_register!(
        write_miscellaneous_output,
        MiscellaneousOutputRegister,
        port!(T::GeneralPorts::WRITE_MISCELLANEOUS_OUTPUT),
    );

    read_register!(
        read_input_status_0,
        InputStatusRegister0,
        port!(T::GeneralPorts::READ_INPUT_STATUS_0),
    );

    pub fn read_input_status_1(&mut self) -> InputStatusRegister1 {
        let port = if self.io_select_address_enabled() {
            port!(T::GeneralPorts::READ_INPUT_STATUS_1_IO_SELECT_ON)
        } else {
            port!(T::GeneralPorts::READ_INPUT_STATUS_1_IO_SELECT_OFF)
        };
        let raw = self.0.read(port);
        InputStatusRegister1::from_register_value(raw)
    }

    read_register!(
        read_feature_control,
        FeatureControlRegister,
        port!(T::GeneralPorts::READ_FEATURE_CONTROL_REGISTER),
    );

    pub fn write_feature_control(&mut self, value: FeatureControlRegister) {
        let port = if self.io_select_address_enabled() {
            port!(T::GeneralPorts::WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_ON)
        } else {
            port!(T::GeneralPorts::WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_OFF)
        };
        self.0.write(port, value.value());
    }

    read_register!(
        read_video_subsystem_enable,
        VideoSubsystemEnableRegister,
        port!(T::GeneralPorts::READ_VIDEO_SUBSYSTEM_ENABLE_REGISTER),
    );

    write_register!(
        write_video_subsystem_enable,
        VideoSubsystemEnableRegister,
        port!(T::GeneralPorts::WRITE_VIDEO_SUBSYSTEM_ENABLE_REGISTER),
    );
}

impl <T: PortIo> RegisterHandler<T> {
    read_write_register!(
        read_sequencer_address,
        write_sequencer_address,
        SequencerAddressRegister,
        port!(T::SequencerPorts::ADDRESS_REGISTER),
    );

    sequencer_register!(
        read_reset,
        write_reset,
        ResetRegister,
    );

    sequencer_register!(
        read_clocking_mode,
        write_clocking_mode,
        ClockingModeRegister,
    );

    sequencer_register!(
        read_map_mask,
        write_map_mask,
        MapMaskRegister,
    );

    sequencer_register!(
        read_character_map_select,
        write_character_map_select,
        CharacterMapSelectRegister,
    );

    sequencer_register!(
        read_memory_mode,
        write_memory_mode,
        MemoryModeRegister,
    );
}

impl <T: PortIo> RegisterHandler<T> {
    pub fn read_crt_address(&mut self) -> CrtAddressRegister {
        let port = if self.io_select_address_enabled() {
            port!(T::CrtPorts::ADDRESS_REGISTER_IO_SELECT_ON)
        } else {
            port!(T::CrtPorts::ADDRESS_REGISTER_IO_SELECT_OFF)
        };
        let raw = self.0.read(port);
        CrtAddressRegister::from_register_value(raw)
    }

    pub fn write_crt_address(&mut self, value: CrtAddressRegister) {
        let port = if self.io_select_address_enabled() {
            port!(T::CrtPorts::ADDRESS_REGISTER_IO_SELECT_ON)
        } else {
            port!(T::CrtPorts::ADDRESS_REGISTER_IO_SELECT_OFF)
        };
        self.0.write(port, value.value());
    }

    crt_register!(
        read_horizontal_total,
        write_horizontal_total,
        HorizontalTotalRegister,
    );

    crt_register!(
        read_horizontal_display_enable_end,
        write_horizontal_display_enable_end,
        HorizontalDisplayEnableEndRegister,
    );

    crt_register!(
        read_start_horizontal_blanking,
        write_start_horizontal_blanking,
        StartHorizontalBlankingRegister,
    );

    crt_register!(
        read_end_horizontal_blanking,
        write_end_horizontal_blanking,
        EndHorizontalBlankingRegister,
    );

    crt_register!(
        read_start_horizontal_retrace_pulse,
        write_start_horizontal_retrace_pulse,
        StartHorizontalRetracePulseRegister,
    );

    crt_register!(
        read_end_horizontal_retrace,
        write_end_horizontal_retrace,
        EndHorizontalRetraceRegister,
    );

    crt_register!(
        read_vertical_total,
        write_vertical_total,
        VerticalTotalRegister,
    );

    crt_register!(
        read_overflow,
        write_overflow,
        OverflowRegister,
    );

    crt_register!(
        read_preset_row_scan,
        write_preset_row_scan,
        PresetRowScanRegister,
    );

    crt_register!(
        read_maximum_scan_line,
        write_maximum_scan_line,
        MaximumScanLineRegister,
    );

    crt_register!(
        read_cursor_start,
        write_cursor_start,
        CursorStartRegister,
    );

    crt_register!(
        read_cursor_end,
        write_cursor_end,
        CursorEndRegister,
    );

    crt_register!(
        read_start_address_high,
        write_start_address_high,
        StartAddressHighRegister,
    );

    crt_register!(
        read_start_address_low,
        write_start_address_low,
        StartAddressLowRegister,
    );

    crt_register!(
        read_cursor_location_high,
        write_cursor_location_high,
        CursorLocationHighRegister,
    );


    crt_register!(
        read_cursor_location_low,
        write_cursor_location_low,
        CursorLocationLowRegister,
    );

    crt_register!(
        read_vertical_retrace_start,
        write_vertical_retrace_start,
        VerticalRetraceStartRegister,
    );

    crt_register!(
        read_vertical_retrace_end,
        write_vertical_retrace_end,
        VerticalRetraceEndRegister,
    );

    crt_register!(
        read_vertical_display_enable_end,
        write_vertical_display_enable_end,
        VerticalDisplayEnableEndRegister,
    );

    crt_register!(
        read_offset,
        write_offset,
        OffsetRegister,
    );

    crt_register!(
        read_underline_location,
        write_underline_location,
        UnderlineLocationRegister,
    );

    crt_register!(
        read_start_vertical_blanking,
        write_start_vertical_blanking,
        StartVerticalBlankingRegister,
    );

    crt_register!(
        read_end_vertical_blanking,
        write_end_vertical_blanking,
        EndVerticalBlankingRegister,
    );

    crt_register!(
        read_crt_mode_control,
        write_crt_mode_control,
        CrtModeControlRegister,
    );

    crt_register!(
        read_line_compare,
        write_line_compare,
        LineCompareRegister,
    );
}

macro_rules! graphics_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $write_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> $register_type {
            let mut address = self.read_graphics_address();
            address.set_graphics_address($register_type::INDEX);
            self.write_graphics_address(address);

            let raw = self.0.read(port!(T::GraphicsPorts::DATA_REGISTER));
            $register_type::from_register_value(raw)
        }

        $(
            #[doc=$text]
        )*
        pub fn $write_method_name(&mut self, value: $register_type) {
            let mut address = self.read_graphics_address();
            address.set_graphics_address($register_type::INDEX);
            self.write_graphics_address(address);

            let port = port!(T::GraphicsPorts::DATA_REGISTER);

            self.0.write(port, value.value());
        }
    };
}

impl <T: PortIo> RegisterHandler<T> {
    read_write_register!(
        read_graphics_address,
        write_graphics_address,
        GraphicsAddressRegister,
        port!(T::GraphicsPorts::ADDRESS_REGISTER),
    );

    graphics_register!(
        read_set_slash_reset,
        write_set_slash_reset,
        SetSlashResetRegister,
    );

    graphics_register!(
        read_enable_set_slash_reset,
        write_enable_set_slash_reset,
        EnableSetSlashResetRegister,
    );

    graphics_register!(
        read_color_compare,
        write_color_compare,
        ColorCompareRegister,
    );

    graphics_register!(
        read_data_rotate,
        write_data_rotate,
        DataRotateRegister,
    );

    graphics_register!(
        read_map_select,
        write_read_map_select,
        ReadMapSelectRegister,
    );

    graphics_register!(
        read_graphics_mode,
        write_graphics_mode,
        GraphicsModeRegister,
    );

    graphics_register!(
        read_miscellaneous,
        write_miscellaneous,
        MiscellaneousRegister,
    );

    graphics_register!(
        read_color_do_not_care,
        write_color_do_not_care,
        ColorDoNotCareRegister,
    );

    graphics_register!(
        read_bit_mask,
        write_bit_mask,
        BitMaskRegister,
    );
}

macro_rules! attribute_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> ReadWrite<'_, T, $register_type> {
            ReadWrite::new(self)
        }
    };
}

impl <T: PortIo> RegisterHandler<T> {
    pub fn read_attribute_address(&mut self) -> AttributeAddressRegister {
        self.read_input_status_1();
        let raw = self.0.read(port!(T::AttributePorts::READ_ADDRESS));
        AttributeAddressRegister::from_register_value(raw)
    }

    pub fn write_attribute_address(&mut self, value: AttributeAddressRegister) {
        self.read_input_status_1();
        self.0.write(port!(T::AttributePorts::WRITE_ADDRESS), value.value());
    }

    fn read_attribute_controller_indexed_register(&mut self, index: u8) -> u8 {
        let mut address = self.read_attribute_address();
        address.set_register_index(index);
        self.write_attribute_address(address);

        self.0.read(port!(T::AttributePorts::READ_ADDRESS))
    }

    fn write_attribute_controller_indexed_register(&mut self, index: u8, value: u8) {
        let mut address = self.read_attribute_address();
        address.set_register_index(index);
        self.write_attribute_address(address);

        let port = port!(T::AttributePorts::WRITE_ADDRESS);
        self.0.write(port, value);
    }

    pub fn read_internal_palette(&mut self, index: InternalPaletteIndex) -> InternalPaletteRegister {
        let raw = self.read_attribute_controller_indexed_register(index.into());
        InternalPaletteRegister::from_register_value(raw)
    }

    pub fn write_internal_palette(&mut self, index: InternalPaletteIndex, value: InternalPaletteRegister) {
        self.write_attribute_controller_indexed_register(index.into(), value.value());
    }

    attribute_register!(
        attribute_mode_control,
        AttributeModeControlRegister,
    );

    attribute_register!(
        overscan_color,
        OverscanColorRegister,
    );

    attribute_register!(
        color_plane_enable,
        ColorPlaneEnableRegister,
    );

    attribute_register!(
        horizontal_pel_panning,
        HorizontalPelPanningRegister,
    );

    attribute_register!(
        read_color_select,
        ColorSelectRegister,
    );
}

impl <T: PortIo> RegisterHandler<T> {
    read_write_register!(
        read_palette_address_write_mode,
        write_palette_address_write_mode,
        PaletteAddressWriteModeRegister,
        port!(T::DacPorts::PALETTE_ADDRESS_WRITE_MODE),
    );

    write_register!(
        write_palette_address_read_mode,
        PaletteAddressReadModeRegister,
        port!(T::DacPorts::PALETTE_ADDRESS_READ_MODE),
    );

    read_register!(
        read_dac_state,
        DacStateRegister,
        port!(T::DacPorts::DAC_STATE),
    );

    read_write_register!(
        read_palette_data,
        write_palette_data,
        PaletteDataRegister,
        port!(T::DacPorts::PALETTE_DATA),
    );

    read_write_register!(
        read_pel_mask,
        write_pel_mask,
        PelMaskRegister,
        port!(T::DacPorts::PEL_MASK),
    );

    pub fn read_dac_palette(&mut self, start_from_index: u8, data: &mut [PaletteColor]) {
        let index = PaletteAddressReadModeRegister::new(start_from_index);
        self.write_palette_address_read_mode(index);

        let iterator = data.iter_mut().take((u8::max_value() - start_from_index + 1) as usize);

        for color in iterator {
           let r = self.read_palette_data();
           let g = self.read_palette_data();
           let b = self.read_palette_data();

           color.set_r(r.color_value());
           color.set_g(g.color_value());
           color.set_b(b.color_value());
        }
    }

    pub fn write_dac_palette(&mut self, start_from_index: u8, data: &[PaletteColor]) {
        let index = PaletteAddressWriteModeRegister::new(start_from_index);
        self.write_palette_address_write_mode(index);

        let iterator = data.iter().take((u8::max_value() - start_from_index + 1) as usize);
        for color in iterator {
           self.write_palette_data(color.r);
           self.write_palette_data(color.g);
           self.write_palette_data(color.b);
        }
    }
}

impl <T: PortIo> RegisterHandler<T> {
    pub fn crt(&mut self) -> CrtControllerValues<'_, T> {
        CrtControllerValues(self)
    }
}

#[derive(Debug)]
pub struct CrtControllerValues<'a, T: PortIo>(&'a mut RegisterHandler<T>);

impl <T: PortIo> CrtControllerValues<'_, T> {
    /// A 6-bit value.
    pub fn end_horizontal_blanking(&mut self) -> u8 {
        self.0.read_end_horizontal_retrace().end_blanking_bit_5() |
        self.0.read_end_horizontal_blanking().end_blanking_bits_from_0_to_4()
    }

    /// A 6-bit value.
    pub fn set_end_horizontal_blanking(&mut self, value: u8) {
        let mut r0 = self.0.read_end_horizontal_retrace();
        let mut r1 = self.0.read_end_horizontal_blanking();

        r0.set_end_blanking_bit_5(value);
        r1.set_end_blanking_bits_from_0_to_4(value);

        self.0.write_end_horizontal_retrace(r0);
        self.0.write_end_horizontal_blanking(r1);
    }

    /// A 10-bit value.
    pub fn vertical_total(&mut self) -> u16 {
        self.0.read_vertical_total().vertical_total_bits_from_0_to_7() |
        self.0.read_overflow().vertical_total_bits_8_and_9()
    }

    /// A 10-bit value.
    pub fn set_vertical_total(&mut self, value: u16) {
        let mut r0 = self.0.read_vertical_total();
        let mut r1 = self.0.read_overflow();

        r0.set_vertical_total_bits_from_0_to_7(value);
        r1.set_vertical_total_bits_8_and_9(value);

        self.0.write_vertical_total(r0);
        self.0.write_overflow(r1);
    }

    /// A 10-bit value.
    pub fn line_compare(&mut self) -> u16 {
        self.0.read_maximum_scan_line().line_compare_bit_9() |
        self.0.read_overflow().line_compare_bit_8() |
        self.0.read_line_compare().line_compare_target_bits_from_0_to_7()
    }

    /// A 10-bit value.
    pub fn set_line_compare(&mut self, value: u16) {
        let mut r0 = self.0.read_maximum_scan_line();
        let mut r1 = self.0.read_overflow();
        let mut r2 = self.0.read_line_compare();

        r0.set_line_compare_bit_9(value);
        r1.set_line_compare_bit_8(value);
        r2.set_line_compare_target_bits_from_0_to_7(value);

        self.0.write_maximum_scan_line(r0);
        self.0.write_overflow(r1);
        self.0.write_line_compare(r2);
    }
}

#[derive(Debug)]
pub struct ReadWrite<'a, T: PortIo, U> {
    registers: &'a mut RegisterHandler<T>,
    register_data: U,
}

impl <'a, T: PortIo, U: RegisterWithIndex + AttributeRegisterMarker> ReadWrite<'a, T, U> {
    pub fn new(
        registers: &'a mut RegisterHandler<T>,
    ) -> Self {
        let raw = registers.read_attribute_controller_indexed_register(U::INDEX);
        let register_data = U::from_register_value(raw);

        Self {
            register_data,
            registers,
        }
    }

    pub fn write(&mut self) {
        self.registers.write_attribute_controller_indexed_register(U::INDEX, self.register_data.value());
    }
}

impl <T: PortIo, U> core::ops::Deref for ReadWrite<'_, T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.register_data
    }
}

impl <T: PortIo, U> core::ops::DerefMut for ReadWrite<'_, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.register_data
    }
}
