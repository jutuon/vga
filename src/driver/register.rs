
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
    ($( #[doc=$text:literal] )* $read_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> ReadWrite<'_, T, $register_type> {
            ReadWrite::new(self, read_sequencer_function, write_sequencer_function)
        }
    };
}

macro_rules! crt_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> ReadWrite<'_, T, $register_type> {
            ReadWrite::new(self, read_crt_function, write_crt_function)
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
    fn read_sequencer_indexed_register(&mut self, index: u8) -> u8 {
        let mut address = self.read_sequencer_address();
        address.set_sequencer_address(index);
        self.write_sequencer_address(address);

        self.0.read(port!(T::SequencerPorts::DATA_REGISTER))
    }

    fn write_sequencer_indexed_register(&mut self, index: u8, value: u8) {
        let mut address = self.read_sequencer_address();
        address.set_sequencer_address(index);
        self.write_sequencer_address(address);

        self.0.write(port!(T::SequencerPorts::DATA_REGISTER), value);
    }

    read_write_register!(
        read_sequencer_address,
        write_sequencer_address,
        SequencerAddressRegister,
        port!(T::SequencerPorts::ADDRESS_REGISTER),
    );

    sequencer_register!(
        reset,
        ResetRegister,
    );

    sequencer_register!(
        clocking_mode,
        ClockingModeRegister,
    );

    sequencer_register!(
        map_mask,
        MapMaskRegister,
    );

    sequencer_register!(
        character_map_select,
        CharacterMapSelectRegister,
    );

    sequencer_register!(
        memory_mode,
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

    pub fn read_crt_controller_indexed_register(&mut self, index: u8) -> u8 {
        let mut address = self.read_crt_address();
        address.set_crt_address(index);
        self.write_crt_address(address);

        let port = if self.io_select_address_enabled() {
            port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_ON)
        } else {
            port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_OFF)
        };

        self.0.read(port)
    }

    fn write_crt_controller_indexed_register(&mut self, index: u8, value: u8) {
        let mut address = self.read_crt_address();
        address.set_crt_address(index);
        self.write_crt_address(address);

        let port = if self.io_select_address_enabled() {
            port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_ON)
        } else {
            port!(T::CrtPorts::DATA_REGISTER_IO_SELECT_OFF)
        };

        self.0.write(port, value);
    }

    crt_register!(
        horizontal_total,
        HorizontalTotalRegister,
    );

    crt_register!(
        horizontal_display_enable_end,
        HorizontalDisplayEnableEndRegister,
    );

    crt_register!(
        start_horizontal_blanking,
        StartHorizontalBlankingRegister,
    );

    crt_register!(
        end_horizontal_blanking,
        EndHorizontalBlankingRegister,
    );

    crt_register!(
        start_horizontal_retrace_pulse,
        StartHorizontalRetracePulseRegister,
    );

    crt_register!(
        end_horizontal_retrace,
        EndHorizontalRetraceRegister,
    );

    crt_register!(
        vertical_total,
        VerticalTotalRegister,
    );

    crt_register!(
        overflow,
        OverflowRegister,
    );

    crt_register!(
        preset_row_scan,
        PresetRowScanRegister,
    );

    crt_register!(
        maximum_scan_line,
        MaximumScanLineRegister,
    );

    crt_register!(
        cursor_start,
        CursorStartRegister,
    );

    crt_register!(
        cursor_end,
        CursorEndRegister,
    );

    crt_register!(
        start_address_high,
        StartAddressHighRegister,
    );

    crt_register!(
        start_address_low,
        StartAddressLowRegister,
    );

    crt_register!(
        cursor_location_high,
        CursorLocationHighRegister,
    );

    crt_register!(
        cursor_location_low,
        CursorLocationLowRegister,
    );

    crt_register!(
        vertical_retrace_start,
        VerticalRetraceStartRegister,
    );

    crt_register!(
        vertical_retrace_end,
        VerticalRetraceEndRegister,
    );

    crt_register!(
        vertical_display_enable_end,
        VerticalDisplayEnableEndRegister,
    );

    crt_register!(
        offset,
        OffsetRegister,
    );

    crt_register!(
        underline_location,
        UnderlineLocationRegister,
    );

    crt_register!(
        start_vertical_blanking,
        StartVerticalBlankingRegister,
    );

    crt_register!(
        end_vertical_blanking,
        EndVerticalBlankingRegister,
    );

    crt_register!(
        crt_mode_control,
        CrtModeControlRegister,
    );

    crt_register!(
        line_compare,
        LineCompareRegister,
    );
}

macro_rules! graphics_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> ReadWrite<'_, T, $register_type> {
            ReadWrite::new(self, read_graphics_function, write_graphics_function)
        }
    };
}

impl <T: PortIo> RegisterHandler<T> {
    fn read_graphics_controller_indexed_register(&mut self, index: u8) -> u8 {
        let mut address = self.read_graphics_address();
        address.set_graphics_address(index);
        self.write_graphics_address(address);

        self.0.read(port!(T::GraphicsPorts::DATA_REGISTER))
    }

    fn write_graphics_controller_indexed_register(&mut self, index: u8, value: u8) {
        let mut address = self.read_graphics_address();
        address.set_graphics_address(index);
        self.write_graphics_address(address);

        let port = port!(T::GraphicsPorts::DATA_REGISTER);

        self.0.write(port, value);
    }

    read_write_register!(
        read_graphics_address,
        write_graphics_address,
        GraphicsAddressRegister,
        port!(T::GraphicsPorts::ADDRESS_REGISTER),
    );

    graphics_register!(
        set_slash_reset,
        SetSlashResetRegister,
    );

    graphics_register!(
        enable_set_slash_reset,
        EnableSetSlashResetRegister,
    );

    graphics_register!(
        color_compare,
        ColorCompareRegister,
    );

    graphics_register!(
        data_rotate,
        DataRotateRegister,
    );

    graphics_register!(
        map_select,
        ReadMapSelectRegister,
    );

    graphics_register!(
        graphics_mode,
        GraphicsModeRegister,
    );

    graphics_register!(
        miscellaneous,
        MiscellaneousRegister,
    );

    graphics_register!(
        color_do_not_care,
        ColorDoNotCareRegister,
    );

    graphics_register!(
        bit_mask,
        BitMaskRegister,
    );
}

macro_rules! attribute_register {
    ($( #[doc=$text:literal] )* $read_method_name:ident, $register_type:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $read_method_name(&mut self) -> ReadWrite<'_, T, $register_type> {
            ReadWrite::new(self, read_attribute_function, write_attribute_function)
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
        color_select,
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
        self.0.end_horizontal_retrace().end_blanking_bit_5() |
        self.0.end_horizontal_blanking().end_blanking_bits_from_0_to_4()
    }

    /// A 6-bit value.
    pub fn set_end_horizontal_blanking(&mut self, value: u8) {
        self.0.end_horizontal_retrace().modify(|r| r.set_end_blanking_bit_5(value)).write();
        self.0.end_horizontal_blanking().modify(|r| r.set_end_blanking_bits_from_0_to_4(value));
    }

    /// A 10-bit value.
    pub fn vertical_total(&mut self) -> u16 {
        self.0.vertical_total().vertical_total_bits_from_0_to_7() |
        self.0.overflow().vertical_total_bits_8_and_9()
    }

    /// A 10-bit value.
    pub fn set_vertical_total(&mut self, value: u16) {
        self.0.vertical_total().modify(|r| r.set_vertical_total_bits_from_0_to_7(value)).write();
        self.0.overflow().modify(|r| r.set_vertical_total_bits_8_and_9(value)).write();
    }

    /// A 10-bit value.
    pub fn line_compare(&mut self) -> u16 {
        self.0.maximum_scan_line().line_compare_bit_9() |
        self.0.overflow().line_compare_bit_8() |
        self.0.line_compare().line_compare_target_bits_from_0_to_7()
    }

    /// A 10-bit value.
    pub fn set_line_compare(&mut self, value: u16) {
        self.0.maximum_scan_line().modify(|r| r.set_line_compare_bit_9(value)).write();
        self.0.overflow().modify(|r| r.set_line_compare_bit_8(value)).write();
        self.0.line_compare().modify(|r| r.set_line_compare_target_bits_from_0_to_7(value)).write();
    }
}

fn read_attribute_function<T: PortIo, U: RegisterWithIndex + AttributeRegisterMarker>(registers: &mut RegisterHandler<T>) -> (&mut RegisterHandler<T>, U) {
    let data = registers.read_attribute_controller_indexed_register(U::INDEX);
    let value = U::from_register_value(data);
    (registers, value)
}

fn write_attribute_function<T: PortIo, U: RegisterWithIndex + AttributeRegisterMarker>(register: &mut ReadWrite<'_, T, U>) {
    register.registers.write_attribute_controller_indexed_register(U::INDEX, register.value());
}

fn read_crt_function<T: PortIo, U: RegisterWithIndex + CrtControllerRegisterMarker>(registers: &mut RegisterHandler<T>) -> (&mut RegisterHandler<T>, U) {
    let data = registers.read_crt_controller_indexed_register(U::INDEX);
    let value = U::from_register_value(data);
    (registers, value)
}

fn write_crt_function<T: PortIo, U: RegisterWithIndex + CrtControllerRegisterMarker>(register: &mut ReadWrite<'_, T, U>) {
    register.registers.write_crt_controller_indexed_register(U::INDEX, register.value());
}

fn read_graphics_function<T: PortIo, U: RegisterWithIndex + GraphicsControllerRegisterMarker>(registers: &mut RegisterHandler<T>) -> (&mut RegisterHandler<T>, U) {
    let data = registers.read_graphics_controller_indexed_register(U::INDEX);
    let value = U::from_register_value(data);
    (registers, value)
}

fn write_graphics_function<T: PortIo, U: RegisterWithIndex + GraphicsControllerRegisterMarker>(register: &mut ReadWrite<'_, T, U>) {
    register.registers.write_graphics_controller_indexed_register(U::INDEX, register.value());
}

fn read_sequencer_function<T: PortIo, U: RegisterWithIndex + SequencerRegisterMarker>(registers: &mut RegisterHandler<T>) -> (&mut RegisterHandler<T>, U) {
    let data = registers.read_sequencer_indexed_register(U::INDEX);
    let value = U::from_register_value(data);
    (registers, value)
}

fn write_sequencer_function<T: PortIo, U: RegisterWithIndex + SequencerRegisterMarker>(register: &mut ReadWrite<'_, T, U>) {
    register.registers.write_sequencer_indexed_register(U::INDEX, register.value());
}


pub struct ReadWrite<'a, T: PortIo, U: Register> {
    registers: &'a mut RegisterHandler<T>,
    write_function: fn(&mut Self),
    register_data: U,
}

impl <T: PortIo, U: Register> core::fmt::Debug for ReadWrite<'_, T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ReadWrite")
    }
}

impl <'a, T: PortIo, U: Register> ReadWrite<'a, T, U> {
    pub fn new(
        registers: &'a mut RegisterHandler<T>,
        read_function: fn(&'a mut RegisterHandler<T>) -> (&'a mut RegisterHandler<T>, U),
        write_function: fn(&mut Self),
    ) -> Self {
        let (registers, register_data) = (read_function)(registers);

        Self {
            register_data,
            registers,
            write_function,
        }
    }

    pub fn write(&mut self) {
        (self.write_function)(self)
    }

    pub fn modify<V: FnMut(&mut U) -> &mut U>(&mut self, mut function: V) -> &mut Self {
        (function)(&mut self.register_data);
        self
    }
}

impl <T: PortIo, U: Register> core::ops::Deref for ReadWrite<'_, T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.register_data
    }
}

impl <T: PortIo, U: Register> core::ops::DerefMut for ReadWrite<'_, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.register_data
    }
}
