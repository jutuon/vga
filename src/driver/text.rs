
//! API for VGA default text mode.

use volatile::Volatile;

use crate::io::{
    VIDEO_RAM_START_ADDRESS,
    MemoryMappedIo,
    PortIo,
};

use crate::raw::{
    VgaRegisters,
};

pub use vga_framebuffer::Colour;
use vga_framebuffer::Char;

pub const TEXT_BUFFER_FIST_BYTE_ADDRESS: usize = 0xB8000;

pub const VIDEO_RAM_BYTES_BEFORE_TEXT: usize =  TEXT_BUFFER_FIST_BYTE_ADDRESS - VIDEO_RAM_START_ADDRESS;
pub const VGA_TEXT_WIDTH: usize = 80;
pub const VGA_TEXT_HEIGHT: usize = 25;
pub const VGA_TEXT_CHAR_COUNT: usize = VGA_TEXT_WIDTH * VGA_TEXT_HEIGHT;
pub const TEXT_BUFFER_BYTE_COUNT: usize = 80 * 25 * 2;

pub struct TextMode<T: PortIo, U: MemoryMappedIo> {
    registers: VgaRegisters<T>,
    ram: U,
}

impl <T: PortIo, U: MemoryMappedIo> TextMode<T, U> {
    pub fn new(io: T, ram: U) -> Self {
        Self {
            registers: VgaRegisters::new(io),
            ram,
        }
    }

    pub fn vga_text_ram(&self) -> &[Volatile<u8>] {
        let (_, text_buffer_and_other_ram) = self.ram.video_ram().split_at(VIDEO_RAM_BYTES_BEFORE_TEXT);
        let (text_buffer, _) = text_buffer_and_other_ram.split_at(TEXT_BUFFER_BYTE_COUNT);
        text_buffer
    }

    pub fn vga_text_ram_mut(&mut self) -> &mut [Volatile<u8>] {
        let (_, text_buffer_and_other_ram) = self.ram.video_ram_mut().split_at_mut(VIDEO_RAM_BYTES_BEFORE_TEXT);
        let (text_buffer, _) = text_buffer_and_other_ram.split_at_mut(TEXT_BUFFER_BYTE_COUNT);
        text_buffer
    }

    pub fn vga_chars(&self) -> impl Iterator<Item=VgaCharRef<'_>> {
        VgaCharRef::raw_slice_to_ref_slice(self.vga_text_ram())
    }

    pub fn vga_chars_mut(&mut self) -> impl Iterator<Item=VgaCharRefMut<'_>> {
        VgaCharRefMut::raw_slice_to_ref_mut_slice(self.vga_text_ram_mut())
    }

    pub fn attribute_bit_7(&mut self, setting: AttributeBit7) {
        let value = setting == AttributeBit7::Blink;
        self.registers.attribute_controller().ar10().modify(|_, w| w.enable_blinking_slash_select_background_intensity().bit(value));
    }

    /// Panics if character index is out of bounds.
    pub fn read_char(&self, character_i: usize) -> VgaChar {
        self.vga_chars().nth(character_i).unwrap().read()
    }

    /// Panics if character index is out of bounds.
    pub fn write_char(&mut self, character_i: usize, vga_char: VgaChar) {
        self.vga_chars_mut().nth(character_i).unwrap().write(vga_char)
    }

    pub fn clear_screen(&mut self, value: VgaChar) {
        for mut vga_char in self.vga_chars_mut() {
            vga_char.write(value)
        }
    }

    pub fn lines(&self) -> impl Iterator<Item=VgaCharLine<'_>> {
        self.vga_text_ram().chunks_exact(VGA_TEXT_WIDTH * 2).map(|raw_line| {
            VgaCharLine {
                raw_line
            }
        })
    }

    pub fn lines_mut(&mut self) -> impl Iterator<Item=VgaCharLineMut<'_>> {
        self.vga_text_ram_mut().chunks_exact_mut(VGA_TEXT_WIDTH * 2).map(|raw_line| {
            VgaCharLineMut {
                raw_line
            }
        })
    }

    /// Panics if line index is out of bounds.
    pub fn copy_line_to(&mut self, src_i: usize, target_i: usize) {
        if src_i == target_i {
            return;
        }

        let src_and_target = self.lines_mut().enumerate().fold((None, None), |(src, target), (i, line)| {
            if i == src_i {
                return (Some(line), target)
            }

            if i == target_i {
                return (src, Some(line))
            }

            (src, target)
        });

        match src_and_target {
            (None, _) => panic!("source index '{}' is out of bounds", src_i),
            (_, None) => panic!("target index '{}' is out of bounds", target_i),
            (Some(src), Some(mut target)) => {
                target.write_line(&src)
            }
        }
    }

    pub fn scroll(&mut self) {
        self.scroll_range(..);
    }

    fn scroll_inclusive_range(&mut self, range: core::ops::RangeInclusive<usize>) {
        let (start_i, mut end_i) = range.into_inner();

        if end_i < start_i {
            return;
        }

        if start_i >= VGA_TEXT_HEIGHT {
            return;
        }

        if end_i >= VGA_TEXT_HEIGHT {
            end_i = VGA_TEXT_HEIGHT - 1;
        }

        let (mut iter, copy_count) = if start_i == 0 {
            (self.lines_mut().skip(0), end_i)
        } else {
            (self.lines_mut().skip(start_i - 1), end_i - start_i + 1)
        };

        let mut target = iter.next().unwrap();

        let iter = iter.take(copy_count);

        for src in iter {
            target.write_line(&src);
            target = src;
        }
    }

    pub fn scroll_range<R: core::ops::RangeBounds<usize>>(&mut self, range: R) {
        use core::ops::Bound;

        let start_i = match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) if i == usize::max_value() => return,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let end_i = match range.end_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) if i == 0 => return,
            Bound::Excluded(&i) => i - 1,
            Bound::Unbounded => VGA_TEXT_HEIGHT - 1,
        };

        self.scroll_inclusive_range(start_i..=end_i);
    }

    pub fn cursor_visibility(&mut self) -> bool {
        !self.registers.crt_controller().cr0a().read().text_cursor_off().bit()
    }

    pub fn set_cursor_visibility(&mut self, visible: bool) {
       self.registers.crt_controller().cr0a().modify(|_, w| w.text_cursor_off().bit(!visible));
    }

    /// Index zero is the top-most line of pixels in a character box. Index is a 5-bit value.
    /// Returns start and end index.
    pub fn cursor_height(&mut self) -> (u8, u8) {
        let start = self.registers.crt_controller().cr0a().read().text_cursor_start().bits();
        let end = self.registers.crt_controller().cr0b().read().text_cursor_end().bits();

        // TODO: Check that index is valid.

        (start, end)
    }

    /// Index zero is the top-most line of pixels in a character box. Index is a 5-bit value.
    ///
    /// If cursor height is out of character box bounds the cursor will not be visible.
    ///
    /// Note that when using the default VGA 9x16 font the max character box height index value is 15.
    ///
    /// Panics if `start > end`.
    pub fn set_cursor_height(&mut self, start: u8, end: u8) {
       assert!(start <= end, "error: start > end, start = {}, end = {}", start, end);
       self.registers.crt_controller().cr0a().modify(|_, w| w.text_cursor_start().bits(start));
       self.registers.crt_controller().cr0b().modify(|_, w| w.text_cursor_end().bits(end));
    }

    /// Get cursor position as character index.
    ///
    /// Returns error if character index is out of bounds.
    pub fn cursor_character_index(&mut self) -> Result<usize, IndexOutOfBounds> {
        let low_byte = self.registers.crt_controller().cr0f().read().text_cursor_location_low_byte().bits();
        let high_byte = self.registers.crt_controller().cr0e().read().text_cursor_location_high_byte().bits();

        let location = ((high_byte as u16) << 8) | low_byte as u16;

        if (location as usize) < VGA_TEXT_CHAR_COUNT {
            Ok(location as usize)
        } else {
            Err(IndexOutOfBounds(location))
        }
    }

    /// Panics if character index is out of bounds.
    pub fn set_cursor_character_index(&mut self, character_i: usize) {
        if character_i >= VGA_TEXT_CHAR_COUNT {
            panic!("Max value for character_i is '{}'", VGA_TEXT_CHAR_COUNT);
        }

        let low_byte = character_i as u8;
        let high_byte = (character_i >> 8) as u8;

        self.registers.crt_controller().cr0f().modify(|_, w| w.text_cursor_location_low_byte().bits(low_byte));
        self.registers.crt_controller().cr0e().modify(|_, w| w.text_cursor_location_high_byte().bits(high_byte));
    }
}

#[derive(Debug)]
pub struct IndexOutOfBounds(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeBit7 {
    Blink,
    Intensity,
}

pub struct VgaCharLine<'a> {
    raw_line: &'a [Volatile<u8>],
}

impl VgaCharLine<'_> {
    pub fn iter(&self) -> impl Iterator<Item=VgaCharRef<'_>> {
        VgaCharRef::raw_slice_to_ref_slice(self.raw_line)
    }
}

pub struct VgaCharLineMut<'a> {
    raw_line: &'a mut [Volatile<u8>],
}

impl VgaCharLineMut<'_> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item=VgaCharRefMut<'_>> {
        VgaCharRefMut::raw_slice_to_ref_mut_slice(self.raw_line)
    }

    pub fn iter(&self) -> impl Iterator<Item=VgaCharRef<'_>> {
        VgaCharRef::raw_slice_to_ref_slice(self.raw_line)
    }

    pub fn write_line<'a, T: Into<VgaCharLine<'a>>>(&mut self, src: T) {
        for (src, mut target) in src.into().iter().zip(self.iter_mut()) {
            target.write(src.read())
        }
    }

    pub fn clear_with(&mut self, value: VgaChar) {
        for mut target in self.iter_mut() {
            target.write(value)
        }
    }
}

impl <'a, 'b> From<&'b VgaCharLineMut<'a>> for VgaCharLine<'b> {
    fn from(value: &'b VgaCharLineMut<'a>) -> Self {
        Self {
            raw_line: value.raw_line
        }
    }
}

impl <'a> From<VgaCharLineMut<'a>> for VgaCharLine<'a> {
    fn from(value: VgaCharLineMut<'a>) -> Self {
        Self {
            raw_line: value.raw_line
        }
    }
}

pub struct VgaCharRef<'a> {
    character: &'a Volatile<u8>,
    attributes: &'a Volatile<u8>,
}

impl VgaCharRef<'_> {
    fn raw_slice_to_ref_slice(raw: &[Volatile<u8>]) -> impl Iterator<Item=VgaCharRef<'_>> {
        raw.chunks_exact(2).map(|data| {
            VgaCharRef {
                character: &data[0],
                attributes: &data[1],
            }
        })
    }

    pub fn read(&self) -> VgaChar {
        VgaChar {
            character: self.character.read(),
            attributes: self.attributes.read(),
        }
    }
}

pub struct VgaCharRefMut<'a> {
    character: &'a mut Volatile<u8>,
    attributes: &'a mut Volatile<u8>,
}

impl VgaCharRefMut<'_> {
    fn raw_slice_to_ref_mut_slice(raw: &mut [Volatile<u8>]) -> impl Iterator<Item=VgaCharRefMut<'_>> {
        raw.chunks_exact_mut(2).map(|data| {
            let (character, attributes) = data.split_first_mut().unwrap();
            VgaCharRefMut {
                character: character,
                attributes: &mut attributes[0],
            }
        })
    }

    fn as_ref(&self) -> VgaCharRef<'_> {
        VgaCharRef {
            character: self.character,
            attributes: self.attributes,
        }
    }

    pub fn write(&mut self, value: VgaChar) {
        self.character.write(value.character);
        self.attributes.write(value.attributes);
    }

    pub fn read(&self) -> VgaChar {
        self.as_ref().read()
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VgaChar {
    pub character: u8,
    pub attributes: u8,
}

impl VgaChar {
    const FOREGROUND_COLOR_MASK: u8 = 0b0000_0111;
    const FOREGROUND_INTENSITY_MASK: u8 = 0b0000_1000;
    const BACKGROUND_COLOR_MASK: u8 = 0b0111_0000;
    const BACKGROUND_INTENSITY_OR_BLINK_MASK: u8 = 0b1000_0000;

    pub fn empty() -> Self {
        Self {
            character: 0,
            attributes: 0,
        }
    }

    pub fn new(c: char) -> Self {
        Self::empty().character(c)
    }

    pub fn character(mut self, c: char) -> Self {
        self.character = Char::map_char(c).to_byte();
        self
    }

    pub fn foreground_color(mut self, color: Colour) -> Self {
        self.attributes &= !Self::FOREGROUND_COLOR_MASK;
        self.attributes |= color as u8;
        self
    }

    pub fn foreground_intensity(mut self, value: bool) -> Self {
        if value {
            self.attributes |= Self::FOREGROUND_INTENSITY_MASK;
        } else {
            self.attributes &= !Self::FOREGROUND_INTENSITY_MASK;
        }

        self
    }

    pub fn background_color(mut self, color: Colour) -> Self {
        self.attributes &= !Self::BACKGROUND_COLOR_MASK;
        let value = (color as u8) << 4;
        self.attributes |= value;

        self
    }

    fn bit_7(mut self, value: bool) -> Self {
        if value {
            self.attributes |= Self::BACKGROUND_INTENSITY_OR_BLINK_MASK;
        } else {
            self.attributes &= !Self::BACKGROUND_INTENSITY_OR_BLINK_MASK;
        }

        self
    }

    pub fn blink(self, value: bool) -> Self {
        self.bit_7(value)
    }

    pub fn background_intensity(self, value: bool) -> Self {
        self.bit_7(value)
    }
}
