
use crate::io::PortIo;

use crate::raw::{
    general::*,
    sequencer::*,
    crt_controller::*,
    graphics_controller::*,
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
