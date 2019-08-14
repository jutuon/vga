
pub mod generated;

use core::fmt;

use self::generated::{
    general::GeneralRegisters,
    sequencer::SequencerRegisters,
    color_palette::{
        ColorPaletteRegisters,
        dacdata::PALETTE_DATA_R,
    },
    crt_controller::CrtControllerRegisters,
    attribute_controller::{
        AttributeControllerRegisters,
        AttributeControllerGroup,
        ar00::PALETTE_BITS_R,
    },
    register_trait::{
        RegisterIndexIoR,
        RegisterIndexIoW,
    },
};

use crate::io::{
    PortIo,
    GeneralIo,
    SequencerIo,
    ColorPaletteIo,
    CrtControllerIo,
    AttributeControllerIo,
};



pub struct VgaRegisters<T: PortIo> {
    io: T,
}

impl <T: PortIo> VgaRegisters<T> {
    pub fn new(io: T) -> Self {
        Self { io }
    }

    pub fn general(&mut self) -> GeneralRegisters<GeneralIo<'_, T>> {
        GeneralRegisters::new(GeneralIo::new(&mut self.io))
    }

    pub fn sequencer(&mut self) -> SequencerRegisters<SequencerIo<'_, T>> {
        SequencerRegisters::new(SequencerIo::new(&mut self.io))
    }

    pub fn crt_controller(&mut self) -> CrtControllerRegisters<CrtControllerIo<'_, T>> {
        CrtControllerRegisters::new(CrtControllerIo::new(&mut self.io))
    }

    pub fn color_palette(&mut self) -> ColorPaletteRegisters<ColorPaletteIo<'_, T>> {
        ColorPaletteRegisters::new(ColorPaletteIo::new(&mut self.io))
    }

    pub fn attribute_controller(&mut self) -> AttributeControllerRegisters<AttributeControllerIo<'_, T>> {
        AttributeControllerRegisters::new(AttributeControllerIo::new(&mut self.io))
    }

    /// Read attribute controller palette register.
    ///
    /// Clear video enable bit from register `ARX` before
    /// reading attribute controller palette registers.
    ///
    /// There are 16 palette registers in attribute controller.
    /// This method accepts index values from 0 to 15.
    pub fn read_attribute_controller_palette_register(&mut self, index: u8) -> Result<PALETTE_BITS_R, InvalidIndex> {
        if index > 15 {
            Err(InvalidIndex)
        } else {
            let mut io = AttributeControllerIo::new(&mut self.io);
            let value = RegisterIndexIoR::<AttributeControllerGroup, u8>::read(&mut io, index);

            Ok(PALETTE_BITS_R::from_register_value(value))
        }
    }

    /// Write attribute controller palette register.
    ///
    /// Clear video enable bit from register `ARX` before
    /// writing attribute controller palette registers.
    ///
    /// There are 16 palette registers in attribute controller.
    /// This method accepts index values from 0 to 15.
    ///
    /// Argument `vga_palette_index` is a 6-bit index to
    /// VGA color palette.
    pub fn write_attribute_controller_palette_register(&mut self, index: u8, mut vga_palette_index: u8) -> Result<(), InvalidIndex> {
        if index > 15 {
            Err(InvalidIndex)
        } else {
            const MASK: u8 = 0b0011_1111;
            // Remove additional bits from the new palette index.
            vga_palette_index &= MASK;

            let mut io = AttributeControllerIo::new(&mut self.io);
            let mut register_value = RegisterIndexIoR::<AttributeControllerGroup, u8>::read(&mut io, index);

            // Remove the current register palette index.
            register_value &= !MASK;
            // Set the new palette index to the register value.
            register_value |= vga_palette_index;

            RegisterIndexIoW::<AttributeControllerGroup, u8>::write(&mut io, index, register_value);

            Ok(())
        }
    }

    pub fn read_vga_color_palette_value(&mut self, index: u8) -> PaletteColor {
        let mut color = [PaletteColor::default()];
        self.read_vga_color_palette(index, &mut color);
        let [color] = color;
        color
    }

    pub fn write_vga_color_palette_value(&mut self, index: u8, color: PaletteColor) {
        let color = [color];
        self.write_vga_color_palette(index, &color);
    }

    pub fn read_vga_color_palette(&mut self, start_from_index: u8, data: &mut [PaletteColor]) {
        self.color_palette().dacrx().write(|w| w.palette_read_index().bits(start_from_index));

        let iterator = data.iter_mut().take((u8::max_value() as u16 - start_from_index as u16 + 1) as usize);

        for color in iterator {
           let r = self.color_palette().dacdata().read().palette_data();
           let g = self.color_palette().dacdata().read().palette_data();
           let b = self.color_palette().dacdata().read().palette_data();

           *color = PaletteColor { r, g, b };
        }
    }

    pub fn write_vga_color_palette(&mut self, start_from_index: u8, data: &[PaletteColor]) {
        self.color_palette().dacwx().write(|w| w.palette_write_index().bits(start_from_index));

        let iterator = data.iter().take((u8::max_value() as u16 - start_from_index as u16 + 1) as usize);

        for color in iterator {
           self.color_palette().dacdata().write(|w| w.palette_data().bits(color.r.bits()));
           self.color_palette().dacdata().write(|w| w.palette_data().bits(color.g.bits()));
           self.color_palette().dacdata().write(|w| w.palette_data().bits(color.b.bits()));
        }
    }
}

#[derive(Debug)]
pub struct InvalidIndex;


#[derive(Copy, Clone)]
pub struct PaletteColor{
    pub(crate) r: PALETTE_DATA_R,
    pub(crate) g: PALETTE_DATA_R,
    pub(crate) b: PALETTE_DATA_R,
}

impl fmt::Debug for PaletteColor {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "({},{},{})",
            self.r.bits(),
            self.g.bits(),
            self.b.bits()
        )
    }
}

impl Default for PaletteColor {
    fn default() -> Self {
        Self {
            r: PALETTE_DATA_R::from_register_value(0),
            g: PALETTE_DATA_R::from_register_value(0),
            b: PALETTE_DATA_R::from_register_value(0),
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
        self.r.bits()
    }

    /// A 6-bit value.
    pub fn g(&self) -> u8 {
        self.g.bits()
    }

    /// A 6-bit value.
    pub fn b(&self) -> u8 {
        self.b.bits()
    }

    /// A 6-bit value.
    pub fn set_r(&mut self, value: u8) {
        self.r = PALETTE_DATA_R::from_register_value(value);
    }

    /// A 6-bit value.
    pub fn set_g(&mut self, value: u8) {
        self.g = PALETTE_DATA_R::from_register_value(value);
    }

    /// A 6-bit value.
    pub fn set_b(&mut self, value: u8) {
        self.b = PALETTE_DATA_R::from_register_value(value);
    }
}
