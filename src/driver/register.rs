
use crate::io::PortIo;

use crate::raw::{
    general::*,
    sequencer::*,
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
