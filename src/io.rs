
use core::slice;

use volatile::Volatile;


pub trait PortIo {
    fn read(&mut self, port: u16) -> u8;
    fn write(&mut self, port: u16, data: u8);
}

pub const VIDEO_RAM_START_ADDRESS: usize = 0xA0000;
pub const VIDEO_RAM_AREA_SIZE_IN_BYTES: usize = 128*1024;

/// Access to VGA memory mapped video RAM.
///
/// Methods of this trait must return
/// 128 KiB memory mapped video RAM area as a slice.
pub trait MemoryMappedIo {
    fn video_ram(&self) -> &[Volatile<u8>];
    fn video_ram_mut(&mut self) -> &mut [Volatile<u8>];
}

pub struct StandardVideoRamLocation {
    ram: &'static mut [Volatile<u8>],
}

impl MemoryMappedIo for StandardVideoRamLocation {
    fn video_ram(&self) -> &[Volatile<u8>] {
        self.ram
    }

    fn video_ram_mut(&mut self) -> &mut [Volatile<u8>] {
        self.ram
    }
}


impl StandardVideoRamLocation {
    /// Create handle to VGA video RAM located at address `0xA0000`.
    pub unsafe fn new() -> Self {
        let start = VIDEO_RAM_START_ADDRESS as *mut Volatile<u8>;

        Self {
            ram: slice::from_raw_parts_mut(start, VIDEO_RAM_AREA_SIZE_IN_BYTES),
        }
    }
}


use crate::raw::generated::{
    register_trait::{
        RegisterAbsIoR,
        RegisterAbsIoW,
        RegisterRelIoR,
        RegisterRelIoW,
        RegisterIndexIoR,
        RegisterIndexIoW,
        LocationAbsW,
    },
    general::{
        GeneralRegisters,
        MSR,
        GeneralGroup,
    },
    attribute_controller::{
        AttributeControllerGroup,
        ARX,
    },
    crt_controller::{
        CrtControllerGroup,
        CRX,
    },
    graphics_controller::{
        GraphicsControllerGroup,
        GRX,
    },
    sequencer::{
        SequencerGroup,
        SRX,
    },
    color_palette::{
        ColorPaletteGroup,
    },
};

macro_rules! io_handler_type {
    (pub struct $io_handler_type:tt) => {
        pub struct $io_handler_type<'a, T: PortIo> {
            io: &'a mut T,
        }

        impl <'a, T: PortIo> $io_handler_type<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self {
                    io
                }
            }
        }
    };
}

macro_rules! impl_abs_address_io {
    (r, $io_handler_type:tt, $register_group:ty ) => {
        impl <'a, T: PortIo> RegisterAbsIoR<$register_group, u8> for $io_handler_type<'a, T> {
            fn read(&mut self, address: u16) -> u8 {
                self.io.read(address)
            }
        }
    };

    (w, $io_handler_type:tt, $register_group:ty ) => {
        impl <'a, T: PortIo> RegisterAbsIoW<$register_group, u8> for $io_handler_type<'a, T> {
            fn write(&mut self, address: u16, data: u8) {
                self.io.write(address, data)
            }
        }
    };

    (rw, $io_handler_type:tt, $register_group:ty ) => {
        impl_abs_address_io!(r, $io_handler_type, $register_group);
        impl_abs_address_io!(w, $io_handler_type, $register_group);
    };
}

macro_rules! impl_rel_address_io {
    (r, $io_handler_type:tt, $register_group:ty ) => {
        impl <'a, T: PortIo> RegisterRelIoR<$register_group, u8> for $io_handler_type<'a, T> {
            fn read(&mut self, mut address: u16) -> u8 {
                if MSR::new(self).read().io_address_select().bit_is_set() {
                    address += 0x20
                }

                self.io.read(address)
            }
        }
    };

    (w, $io_handler_type:tt, $register_group:ty ) => {
        impl <'a, T: PortIo> RegisterRelIoW<$register_group, u8> for $io_handler_type<'a, T> {
            fn write(&mut self, mut address: u16, data: u8) {
                if MSR::new(self).read().io_address_select().bit_is_set() {
                    address += 0x20
                }

                self.io.write(address, data)
            }
        }
    };

    (rw, $io_handler_type:tt, $register_group:ty ) => {
        impl_rel_address_io!(r, $io_handler_type, $register_group);
        impl_rel_address_io!(w, $io_handler_type, $register_group);
    };
}

io_handler_type!(pub struct GeneralIo);
impl_abs_address_io!(rw, GeneralIo, GeneralGroup);
impl_rel_address_io!(rw, GeneralIo, GeneralGroup);

io_handler_type!(pub struct AttributeControllerIo);

impl <'a, T: PortIo> RegisterAbsIoR<AttributeControllerGroup, u8> for AttributeControllerIo<'a, T> {
    fn read(&mut self, address: u16) -> u8 {
        self.io.read(address)
    }
}

impl <'a, T: PortIo> RegisterAbsIoW<AttributeControllerGroup, u8> for AttributeControllerIo<'a, T> {
    fn write(&mut self, address: u16, data: u8) {
        // Reset flip-flop
        GeneralRegisters::new(GeneralIo::new(self.io)).st01().read();

        self.io.write(address, data);
    }
}

impl <'a, T: PortIo> RegisterIndexIoR<AttributeControllerGroup, u8> for AttributeControllerIo<'a, T> {
    fn read(&mut self, index: u8) -> u8 {
        ARX::new(self).modify(|_, w| w.attribute_controller_register_index().bits(index));
        self.io.read(0x3C1)
    }
}

impl <'a, T: PortIo> RegisterIndexIoW<AttributeControllerGroup, u8> for AttributeControllerIo<'a, T> {
    fn write(&mut self, index: u8, data: u8) {
        ARX::new(self).modify(|_, w| w.attribute_controller_register_index().bits(index));
        self.io.write(ARX::<Self>::ABS_ADDRESS_W, data);
    }
}

io_handler_type!(pub struct ColorPaletteIo);
impl_abs_address_io!(rw, ColorPaletteIo, ColorPaletteGroup);

io_handler_type!(pub struct CrtControllerIo);
impl_abs_address_io!(rw, CrtControllerIo, GeneralGroup);
impl_rel_address_io!(rw, CrtControllerIo, CrtControllerGroup);

/// Data port when IO Address Select bit from MSR register is clear.
const CRT_CONTROLLER_DATA_PORT: u16 = 0x3B5;

impl <'a, T: PortIo> RegisterIndexIoR<CrtControllerGroup, u8> for CrtControllerIo<'a, T> {
    fn read(&mut self, index: u8) -> u8 {
        CRX::new(self).modify(|_, w| w.crt_controller_index().bits(index));
        <Self as RegisterRelIoR<CrtControllerGroup, u8>>::read(self, CRT_CONTROLLER_DATA_PORT)
    }
}

impl <'a, T: PortIo> RegisterIndexIoW<CrtControllerGroup, u8> for CrtControllerIo<'a, T> {
    fn write(&mut self, index: u8, data: u8) {
        CRX::new(self).modify(|_, w| w.crt_controller_index().bits(index));
        <Self as RegisterRelIoW<CrtControllerGroup, u8>>::write(self, CRT_CONTROLLER_DATA_PORT, data);
    }
}

io_handler_type!(pub struct GraphicsControllerIo);
impl_abs_address_io!(rw, GraphicsControllerIo, GraphicsControllerGroup);

const GRAPHICS_CONTROLLER_DATA_PORT: u16 = 0x3CF;

impl <'a, T: PortIo> RegisterIndexIoR<GraphicsControllerGroup, u8> for GraphicsControllerIo<'a, T> {
    fn read(&mut self, index: u8) -> u8 {
        GRX::new(self).modify(|_, w| w.graphics_controller_register_index().bits(index));
        self.io.read(GRAPHICS_CONTROLLER_DATA_PORT)
    }
}

impl <'a, T: PortIo> RegisterIndexIoW<GraphicsControllerGroup, u8> for GraphicsControllerIo<'a, T> {
    fn write(&mut self, index: u8, data: u8) {
        GRX::new(self).modify(|_, w| w.graphics_controller_register_index().bits(index));
        self.io.write(GRAPHICS_CONTROLLER_DATA_PORT, data);
    }
}

io_handler_type!(pub struct SequencerIo);
impl_abs_address_io!(rw, SequencerIo, SequencerGroup);

const SEQUENCER_DATA_PORT: u16 = 0x3C5;

impl <'a, T: PortIo> RegisterIndexIoR<SequencerGroup, u8> for SequencerIo<'a, T> {
    fn read(&mut self, index: u8) -> u8 {
        SRX::new(self).modify(|_, w| w.sequencer_index().bits(index));
        self.io.read(SEQUENCER_DATA_PORT)
    }
}

impl <'a, T: PortIo> RegisterIndexIoW<SequencerGroup, u8> for SequencerIo<'a, T> {
    fn write(&mut self, index: u8, data: u8) {
        SRX::new(self).modify(|_, w| w.sequencer_index().bits(index));
        self.io.write(SEQUENCER_DATA_PORT, data);
    }
}
