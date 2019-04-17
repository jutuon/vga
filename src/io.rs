
macro_rules! port_consts {
    ($port_module:ident, $( $port:ident ),+ $(,)? ) => {
        $( const $port: Self = convert_u16_to_vga_port_id(crate::raw::$port_module::$port); )*
    };
}

#[derive(Debug, Copy, Clone)]
pub struct VgaPortId(u16);

impl VgaPortId {
    pub fn value(&self) -> u16 {
        self.0
    }
}

const fn convert_u16_to_vga_port_id(value: u16) -> VgaPortId {
    VgaPortId(value)
}

impl AttributePorts for VgaPortId {
    port_consts!(
        attribute_controller,
        READ_ADDRESS,
        WRITE_ADDRESS,
    );
}

impl CrtPorts for VgaPortId {
    port_consts!(
        crt_controller,
        ADDRESS_REGISTER_IO_SELECT_OFF,
        ADDRESS_REGISTER_IO_SELECT_ON,
        DATA_REGISTER_IO_SELECT_OFF,
        DATA_REGISTER_IO_SELECT_ON,
    );
}

impl GeneralPorts for VgaPortId {
    port_consts!(
        general,
        READ_MISCELLANEOUS_OUTPUT,
        READ_INPUT_STATUS_0,
        READ_INPUT_STATUS_1_IO_SELECT_OFF,
        READ_INPUT_STATUS_1_IO_SELECT_ON,
        READ_FEATURE_CONTROL_REGISTER,
        READ_VIDEO_SUBSYSTEM_ENABLE_REGISTER,
        WRITE_MISCELLANEOUS_OUTPUT,
        WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_OFF,
        WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_ON,
        WRITE_VIDEO_SUBSYSTEM_ENABLE_REGISTER,
    );
}

impl GraphicsPorts for VgaPortId {
    port_consts!(
        graphics_controller,
        ADDRESS_REGISTER,
        DATA_REGISTER,
    );
}

impl SequencerPorts for VgaPortId {
    port_consts!(
        sequencer,
        ADDRESS_REGISTER,
        DATA_REGISTER,
    );
}

impl DacPorts for VgaPortId {
    port_consts!(
        video_dac_palette,
        PALETTE_ADDRESS_WRITE_MODE,
        PALETTE_ADDRESS_READ_MODE,
        DAC_STATE,
        PALETTE_DATA,
        PEL_MASK,
    );
}

pub trait AttributePorts: Copy {
    const WRITE_ADDRESS: Self;
    const READ_ADDRESS: Self;
}

pub trait CrtPorts: Copy {
    const ADDRESS_REGISTER_IO_SELECT_OFF: Self;
    const ADDRESS_REGISTER_IO_SELECT_ON: Self;
    const DATA_REGISTER_IO_SELECT_OFF: Self;
    const DATA_REGISTER_IO_SELECT_ON: Self;
}

pub trait GeneralPorts: Copy {
    const READ_MISCELLANEOUS_OUTPUT: Self;
    const READ_INPUT_STATUS_0: Self;
    const READ_INPUT_STATUS_1_IO_SELECT_OFF: Self;
    const READ_INPUT_STATUS_1_IO_SELECT_ON: Self;
    const READ_FEATURE_CONTROL_REGISTER: Self;
    const READ_VIDEO_SUBSYSTEM_ENABLE_REGISTER: Self;

    const WRITE_MISCELLANEOUS_OUTPUT: Self;
    const WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_OFF: Self;
    const WRITE_FEATURE_CONTROL_REGISTER_IO_SELECT_ON: Self;
    const WRITE_VIDEO_SUBSYSTEM_ENABLE_REGISTER: Self;
}

pub trait GraphicsPorts: Copy {
    const ADDRESS_REGISTER: Self;
    const DATA_REGISTER: Self;
}

pub trait SequencerPorts: Copy {
    const ADDRESS_REGISTER: Self;
    const DATA_REGISTER: Self;
}

pub trait DacPorts: Copy {
    const PALETTE_ADDRESS_WRITE_MODE: Self;
    const PALETTE_ADDRESS_READ_MODE: Self;
    const DAC_STATE: Self;
    const PALETTE_DATA: Self;
    const PEL_MASK: Self;
}

pub trait PortIo {
    type PortId:
        Copy +
        AttributePorts +
        CrtPorts +
        GeneralPorts +
        GraphicsPorts +
        SequencerPorts +
        DacPorts;

    fn read(&self, port: Self::PortId) -> u8;
    fn write(&mut self, port: Self::PortId, data: u8);
}

/// You should only use this trait for debugging purposes.
pub trait PortIoAvailable<T: PortIo> {
    fn port_io(&self) -> &T;
    fn port_io_mut(&mut self) -> &mut T;
}

macro_rules! port {
    ($port_io_type:ident :: $port_trait:ident :: $port_name:ident) => {
        {
            <<$port_io_type as $crate::io::PortIo>::PortId as $crate::io::$port_trait>::$port_name
        }
    };
}
