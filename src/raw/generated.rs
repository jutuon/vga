#![allow(non_camel_case_types)]
#![doc = "Generated from register description `VGA`"]
#![doc = ""]
#![doc = "VGA Registers\n\nRegister information is from\n* [Intel\u{ae} OpenSource HD Graphics Programmer\u{2019}s Reference Manual (PRM) Volume 3 Part 1: VGA and Extended VGA Registers (Ivy Bridge)](https://01.org/sites/default/files/documentation/ivb_ihd_os_vol3_part1_0.pdf)\n* [IBM Personal System/2\u{ae} Hardware Interface Technical Reference](http://classiccomputers.info/down/IBM_PS2/documents/PS2_Hardware_Interface_Technical_Reference_May88.pdf)\n    * Video Subsystem chapter starts from PDF page 388.\n\nRegister addresses which depend on IO Address Select bit in the Miscellaneous\nOutput register are defined as relative. These relative addresses can be\nused as is when IO Address Select bit is clear. When bit is set add 0x20 to\nthe address before accessing the register with it. This converts the second\nhexadecimal digit from B to D.\n"]
pub mod register_trait {
    pub trait LocationIndexR {
        const INDEX_R: u8;
    }
    pub trait LocationAbsR {
        const ABS_ADDRESS_R: u16;
    }
    pub trait LocationRelR {
        const REL_ADDRESS_R: u16;
    }
    pub trait LocationIndexW {
        const INDEX_W: u8;
    }
    pub trait LocationAbsW {
        const ABS_ADDRESS_W: u16;
    }
    pub trait LocationRelW {
        const REL_ADDRESS_W: u16;
    }
    pub trait RegisterIndexIoR<T: RegisterGroup, U: Sized> {
        fn read(&mut self, index: u8) -> U;
    }
    pub trait RegisterIndexIoW<T: RegisterGroup, U: Sized> {
        fn write(&mut self, index: u8, value: U);
    }
    pub trait RegisterAbsIoR<T: RegisterGroup, U: Sized> {
        fn read(&mut self, abs_address: u16) -> U;
    }
    pub trait RegisterAbsIoW<T: RegisterGroup, U: Sized> {
        fn write(&mut self, abs_address: u16, value: U);
    }
    pub trait RegisterRelIoR<T: RegisterGroup, U: Sized> {
        fn read(&mut self, rel_address: u16) -> U;
    }
    pub trait RegisterRelIoW<T: RegisterGroup, U: Sized> {
        fn write(&mut self, rel_address: u16, value: U);
    }
    pub trait RegisterGroup {}
    pub trait InGroup {
        type Group: RegisterGroup;
    }
}
pub mod general {
    use super::register_trait::*;
    pub struct GeneralRegisters<
        T: RegisterAbsIoR<GeneralGroup, u8>
            + RegisterAbsIoW<GeneralGroup, u8>
            + RegisterRelIoR<GeneralGroup, u8>
            + RegisterRelIoW<GeneralGroup, u8>,
    > {
        io: T,
    }
    impl<
            T: RegisterAbsIoR<GeneralGroup, u8>
                + RegisterAbsIoW<GeneralGroup, u8>
                + RegisterRelIoR<GeneralGroup, u8>
                + RegisterRelIoW<GeneralGroup, u8>,
        > GeneralRegisters<T>
    {
        #[inline]
        pub fn new(io: T) -> Self {
            Self { io }
        }
        #[doc = "Miscellaneous Output Register"]
        #[inline]
        pub fn msr(&mut self) -> MSR<'_, T> {
            MSR { io: &mut self.io }
        }
        #[doc = "Input Status 0"]
        #[inline]
        pub fn st00(&mut self) -> ST00<'_, T> {
            ST00 { io: &mut self.io }
        }
        #[doc = "Input Status 1"]
        #[inline]
        pub fn st01(&mut self) -> ST01<'_, T> {
            ST01 { io: &mut self.io }
        }
        #[doc = "Feature Control"]
        #[inline]
        pub fn fcr(&mut self) -> FCR<'_, T> {
            FCR { io: &mut self.io }
        }
        #[inline]
        pub fn video_subsystem_enable(&mut self) -> VIDEO_SUBSYSTEM_ENABLE<'_, T> {
            VIDEO_SUBSYSTEM_ENABLE { io: &mut self.io }
        }
        pub fn debug_registers<F: FnMut(&dyn core::fmt::Debug)>(&mut self, mut f: F) {
            (f)(&self.msr().read());
            (f)(&self.st00().read());
            (f)(&self.st01().read());
            (f)(&self.fcr().read());
            (f)(&self.video_subsystem_enable().read());
        }
    }
    pub struct GeneralGroup;
    impl RegisterGroup for GeneralGroup {}
    #[doc = "Miscellaneous Output Register"]
    pub struct MSR<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>> {
        io: &'a mut T,
    }
    pub mod msr {
        use super::super::register_trait::*;
        use super::GeneralGroup;
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>>
            LocationAbsR for super::MSR<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 972;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>>
            LocationAbsW for super::MSR<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 962;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>> InGroup
            for super::MSR<'a, T>
        {
            type Group = GeneralGroup;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>> super::MSR<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("MSR")
                    .field(
                        "vertical_and_horizontal_sync_polarity",
                        &format_args!("{:?}", self.vertical_and_horizontal_sync_polarity()),
                    )
                    .field("clock_select", &format_args!("{:?}", self.clock_select()))
                    .field("enable_ram", &self.enable_ram().bit())
                    .field("io_address_select", &self.io_address_select().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:6 - Bit 7 is Vertical Sync Polarity and bit 6 is Horizontal Sync Polarity."]
            #[inline]
            pub fn vertical_and_horizontal_sync_polarity(
                &self,
            ) -> VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R {
                VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 3:2"]
            #[inline]
            pub fn clock_select(&self) -> CLOCK_SELECT_R {
                CLOCK_SELECT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn enable_ram(&self) -> ENABLE_RAM_R {
                ENABLE_RAM_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn io_address_select(&self) -> IO_ADDRESS_SELECT_R {
                IO_ADDRESS_SELECT_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `vertical_and_horizontal_sync_polarity`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R {
            RESERVED = 0,
            LINES_400 = 1,
            LINES_350 = 2,
            LINES_480 = 3,
        }
        impl VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R {
            #[doc = "Checks if the value of the field is `RESERVED`"]
            #[inline]
            pub fn is_reserved(&self) -> bool {
                *self == VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::RESERVED
            }
            #[doc = "Checks if the value of the field is `LINES_400`"]
            #[inline]
            pub fn is_lines_400(&self) -> bool {
                *self == VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::LINES_400
            }
            #[doc = "Checks if the value of the field is `LINES_350`"]
            #[inline]
            pub fn is_lines_350(&self) -> bool {
                *self == VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::LINES_350
            }
            #[doc = "Checks if the value of the field is `LINES_480`"]
            #[inline]
            pub fn is_lines_480(&self) -> bool {
                *self == VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::LINES_480
            }
            const _MASK: u8 = 192;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::RESERVED,
                    1 => VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::LINES_400,
                    2 => VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::LINES_350,
                    3 => VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_R::LINES_480,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `clock_select`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CLOCK_SELECT_R {
            #[doc = "Selects 25.175 MHz clock for 640/320 Horizontal PELs"]
            SELECT25 = 0,
            #[doc = "Selects 28.322 MHz clock for 720/360 Horizontal PELs"]
            SELECT28 = 1,
            #[doc = "Selects External Clock"]
            EXTERNAL_CLOCK = 2,
            RESERVED = 3,
        }
        impl CLOCK_SELECT_R {
            #[doc = "Checks if the value of the field is `SELECT25`"]
            #[inline]
            pub fn is_select25(&self) -> bool {
                *self == CLOCK_SELECT_R::SELECT25
            }
            #[doc = "Checks if the value of the field is `SELECT28`"]
            #[inline]
            pub fn is_select28(&self) -> bool {
                *self == CLOCK_SELECT_R::SELECT28
            }
            #[doc = "Checks if the value of the field is `EXTERNAL_CLOCK`"]
            #[inline]
            pub fn is_external_clock(&self) -> bool {
                *self == CLOCK_SELECT_R::EXTERNAL_CLOCK
            }
            #[doc = "Checks if the value of the field is `RESERVED`"]
            #[inline]
            pub fn is_reserved(&self) -> bool {
                *self == CLOCK_SELECT_R::RESERVED
            }
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => CLOCK_SELECT_R::SELECT25,
                    1 => CLOCK_SELECT_R::SELECT28,
                    2 => CLOCK_SELECT_R::EXTERNAL_CLOCK,
                    3 => CLOCK_SELECT_R::RESERVED,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Possible values of the field `enable_ram`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_RAM_R {
            _Reserved(bool),
        }
        impl ENABLE_RAM_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_RAM_R::_Reserved(true) => Self::_MASK,
                    ENABLE_RAM_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_RAM_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_RAM_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `io_address_select`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum IO_ADDRESS_SELECT_R {
            _Reserved(bool),
        }
        impl IO_ADDRESS_SELECT_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    IO_ADDRESS_SELECT_R::_Reserved(true) => Self::_MASK,
                    IO_ADDRESS_SELECT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                IO_ADDRESS_SELECT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    IO_ADDRESS_SELECT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:6 - Bit 7 is Vertical Sync Polarity and bit 6 is Horizontal Sync Polarity."]
            #[inline]
            pub fn vertical_and_horizontal_sync_polarity(
                &mut self,
            ) -> _VERTICAL_AND_HORIZONTAL_SYNC_POLARITY<'_> {
                _VERTICAL_AND_HORIZONTAL_SYNC_POLARITY { w: self }
            }
            #[doc = "Bits 3:2"]
            #[inline]
            pub fn clock_select(&mut self) -> _CLOCK_SELECT<'_> {
                _CLOCK_SELECT { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn enable_ram(&mut self) -> _ENABLE_RAM<'_> {
                _ENABLE_RAM { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn io_address_select(&mut self) -> _IO_ADDRESS_SELECT<'_> {
                _IO_ADDRESS_SELECT { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `vertical_and_horizontal_sync_polarity`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W {
            RESERVED = 0,
            LINES_400 = 1,
            LINES_350 = 2,
            LINES_480 = 3,
        }
        impl VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W {
            const _MASK: u8 = 192;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_AND_HORIZONTAL_SYNC_POLARITY<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_AND_HORIZONTAL_SYNC_POLARITY<'a> {
            const _MASK: u8 = 192;
            const _OFFSET: u8 = 6;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn reserved(self) -> &'a mut W {
                self.variant(VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W::RESERVED)
            }
            #[inline]
            pub fn lines_400(self) -> &'a mut W {
                self.variant(VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W::LINES_400)
            }
            #[inline]
            pub fn lines_350(self) -> &'a mut W {
                self.variant(VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W::LINES_350)
            }
            #[inline]
            pub fn lines_480(self) -> &'a mut W {
                self.variant(VERTICAL_AND_HORIZONTAL_SYNC_POLARITY_W::LINES_480)
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `clock_select`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CLOCK_SELECT_W {
            #[doc = "Selects 25.175 MHz clock for 640/320 Horizontal PELs"]
            SELECT25 = 0,
            #[doc = "Selects 28.322 MHz clock for 720/360 Horizontal PELs"]
            SELECT28 = 1,
            #[doc = "Selects External Clock"]
            EXTERNAL_CLOCK = 2,
            RESERVED = 3,
        }
        impl CLOCK_SELECT_W {
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _CLOCK_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _CLOCK_SELECT<'a> {
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: CLOCK_SELECT_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[doc = "Selects 25.175 MHz clock for 640/320 Horizontal PELs"]
            #[inline]
            pub fn select25(self) -> &'a mut W {
                self.variant(CLOCK_SELECT_W::SELECT25)
            }
            #[doc = "Selects 28.322 MHz clock for 720/360 Horizontal PELs"]
            #[inline]
            pub fn select28(self) -> &'a mut W {
                self.variant(CLOCK_SELECT_W::SELECT28)
            }
            #[doc = "Selects External Clock"]
            #[inline]
            pub fn external_clock(self) -> &'a mut W {
                self.variant(CLOCK_SELECT_W::EXTERNAL_CLOCK)
            }
            #[inline]
            pub fn reserved(self) -> &'a mut W {
                self.variant(CLOCK_SELECT_W::RESERVED)
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_RAM<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_RAM<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _IO_ADDRESS_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _IO_ADDRESS_SELECT<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Input Status 0"]
    pub struct ST00<'a, T: RegisterAbsIoR<GeneralGroup, u8>> {
        io: &'a mut T,
    }
    pub mod st00 {
        use super::super::register_trait::*;
        use super::GeneralGroup;
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8>> LocationAbsR for super::ST00<'a, T> {
            const ABS_ADDRESS_R: u16 = 962;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8>> InGroup for super::ST00<'a, T> {
            type Group = GeneralGroup;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8>> super::ST00<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("ST00")
                    .field("crt_interrupt", &self.crt_interrupt().bit())
                    .field("switch_sense_bit", &self.switch_sense_bit().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7"]
            #[inline]
            pub fn crt_interrupt(&self) -> CRT_INTERRUPT_R {
                CRT_INTERRUPT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn switch_sense_bit(&self) -> SWITCH_SENSE_BIT_R {
                SWITCH_SENSE_BIT_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `crt_interrupt`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CRT_INTERRUPT_R {
            _Reserved(bool),
        }
        impl CRT_INTERRUPT_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    CRT_INTERRUPT_R::_Reserved(true) => Self::_MASK,
                    CRT_INTERRUPT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                CRT_INTERRUPT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    CRT_INTERRUPT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `switch_sense_bit`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SWITCH_SENSE_BIT_R {
            _Reserved(bool),
        }
        impl SWITCH_SENSE_BIT_R {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SWITCH_SENSE_BIT_R::_Reserved(true) => Self::_MASK,
                    SWITCH_SENSE_BIT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SWITCH_SENSE_BIT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SWITCH_SENSE_BIT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
    }
    #[doc = "Input Status 1"]
    pub struct ST01<'a, T: RegisterRelIoR<GeneralGroup, u8>> {
        io: &'a mut T,
    }
    pub mod st01 {
        use super::super::register_trait::*;
        use super::GeneralGroup;
        impl<'a, T: RegisterRelIoR<GeneralGroup, u8>> LocationRelR for super::ST01<'a, T> {
            const REL_ADDRESS_R: u16 = 986;
        }
        impl<'a, T: RegisterRelIoR<GeneralGroup, u8>> InGroup for super::ST01<'a, T> {
            type Group = GeneralGroup;
        }
        impl<'a, T: RegisterRelIoR<GeneralGroup, u8>> super::ST01<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::REL_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("ST01")
                    .field("vertical_retrace", &self.vertical_retrace().bit())
                    .field("display_enable", &self.display_enable().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 3"]
            #[inline]
            pub fn vertical_retrace(&self) -> VERTICAL_RETRACE_R {
                VERTICAL_RETRACE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn display_enable(&self) -> DISPLAY_ENABLE_R {
                DISPLAY_ENABLE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_retrace`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_RETRACE_R {
            _Reserved(bool),
        }
        impl VERTICAL_RETRACE_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_RETRACE_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_RETRACE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_RETRACE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_RETRACE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `display_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DISPLAY_ENABLE_R {
            _Reserved(bool),
        }
        impl DISPLAY_ENABLE_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    DISPLAY_ENABLE_R::_Reserved(true) => Self::_MASK,
                    DISPLAY_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                DISPLAY_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    DISPLAY_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
    }
    #[doc = "Feature Control"]
    pub struct FCR<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterRelIoW<GeneralGroup, u8>> {
        io: &'a mut T,
    }
    pub mod fcr {
        use super::super::register_trait::*;
        use super::GeneralGroup;
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterRelIoW<GeneralGroup, u8>>
            LocationAbsR for super::FCR<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 970;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterRelIoW<GeneralGroup, u8>>
            LocationRelW for super::FCR<'a, T>
        {
            const REL_ADDRESS_W: u16 = 954;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterRelIoW<GeneralGroup, u8>> InGroup
            for super::FCR<'a, T>
        {
            type Group = GeneralGroup;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterRelIoW<GeneralGroup, u8>> super::FCR<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::REL_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("FCR").finish()
            }
        }
        impl R {}
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {}
    }
    pub struct VIDEO_SUBSYSTEM_ENABLE<
        'a,
        T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod video_subsystem_enable {
        use super::super::register_trait::*;
        use super::GeneralGroup;
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>>
            LocationAbsR for super::VIDEO_SUBSYSTEM_ENABLE<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 963;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>>
            LocationAbsW for super::VIDEO_SUBSYSTEM_ENABLE<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 963;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>> InGroup
            for super::VIDEO_SUBSYSTEM_ENABLE<'a, T>
        {
            type Group = GeneralGroup;
        }
        impl<'a, T: RegisterAbsIoR<GeneralGroup, u8> + RegisterAbsIoW<GeneralGroup, u8>>
            super::VIDEO_SUBSYSTEM_ENABLE<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("VIDEO_SUBSYSTEM_ENABLE")
                    .field(
                        "video_subsystem_enable",
                        &self.video_subsystem_enable().bit(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 0"]
            #[inline]
            pub fn video_subsystem_enable(&self) -> VIDEO_SUBSYSTEM_ENABLE_R {
                VIDEO_SUBSYSTEM_ENABLE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `video_subsystem_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VIDEO_SUBSYSTEM_ENABLE_R {
            _Reserved(bool),
        }
        impl VIDEO_SUBSYSTEM_ENABLE_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VIDEO_SUBSYSTEM_ENABLE_R::_Reserved(true) => Self::_MASK,
                    VIDEO_SUBSYSTEM_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VIDEO_SUBSYSTEM_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VIDEO_SUBSYSTEM_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 0"]
            #[inline]
            pub fn video_subsystem_enable(&mut self) -> _VIDEO_SUBSYSTEM_ENABLE<'_> {
                _VIDEO_SUBSYSTEM_ENABLE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VIDEO_SUBSYSTEM_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _VIDEO_SUBSYSTEM_ENABLE<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
}
pub mod sequencer {
    use super::register_trait::*;
    pub struct SequencerRegisters<
        T: RegisterAbsIoR<SequencerGroup, u8>
            + RegisterAbsIoW<SequencerGroup, u8>
            + RegisterIndexIoR<SequencerGroup, u8>
            + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: T,
    }
    impl<
            T: RegisterAbsIoR<SequencerGroup, u8>
                + RegisterAbsIoW<SequencerGroup, u8>
                + RegisterIndexIoR<SequencerGroup, u8>
                + RegisterIndexIoW<SequencerGroup, u8>,
        > SequencerRegisters<T>
    {
        #[inline]
        pub fn new(io: T) -> Self {
            Self { io }
        }
        #[doc = "Sequencer Index"]
        #[inline]
        pub fn srx(&mut self) -> SRX<'_, T> {
            SRX { io: &mut self.io }
        }
        #[doc = "Sequencer Reset"]
        #[inline]
        pub fn sr00(&mut self) -> SR00<'_, T> {
            SR00 { io: &mut self.io }
        }
        #[doc = "Clocking Mode"]
        #[inline]
        pub fn sr01(&mut self) -> SR01<'_, T> {
            SR01 { io: &mut self.io }
        }
        #[doc = "Plane/Map Mask"]
        #[inline]
        pub fn sr02(&mut self) -> SR02<'_, T> {
            SR02 { io: &mut self.io }
        }
        #[doc = "Character Font"]
        #[inline]
        pub fn sr03(&mut self) -> SR03<'_, T> {
            SR03 { io: &mut self.io }
        }
        #[doc = "Memory Mode Register"]
        #[inline]
        pub fn sr04(&mut self) -> SR04<'_, T> {
            SR04 { io: &mut self.io }
        }
        #[doc = "Horizontal Character Counter Reset"]
        #[inline]
        pub fn sr07(&mut self) -> SR07<'_, T> {
            SR07 { io: &mut self.io }
        }
        pub fn debug_registers<F: FnMut(&dyn core::fmt::Debug)>(&mut self, mut f: F) {
            (f)(&self.srx().read());
            (f)(&self.sr00().read());
            (f)(&self.sr01().read());
            (f)(&self.sr02().read());
            (f)(&self.sr03().read());
            (f)(&self.sr04().read());
            (f)(&self.sr07().read());
        }
    }
    pub struct SequencerGroup;
    impl RegisterGroup for SequencerGroup {}
    #[doc = "Sequencer Index"]
    pub struct SRX<'a, T: RegisterAbsIoR<SequencerGroup, u8> + RegisterAbsIoW<SequencerGroup, u8>> {
        io: &'a mut T,
    }
    pub mod srx {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<'a, T: RegisterAbsIoR<SequencerGroup, u8> + RegisterAbsIoW<SequencerGroup, u8>>
            LocationAbsR for super::SRX<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 964;
        }
        impl<'a, T: RegisterAbsIoR<SequencerGroup, u8> + RegisterAbsIoW<SequencerGroup, u8>>
            LocationAbsW for super::SRX<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 964;
        }
        impl<'a, T: RegisterAbsIoR<SequencerGroup, u8> + RegisterAbsIoW<SequencerGroup, u8>> InGroup
            for super::SRX<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<'a, T: RegisterAbsIoR<SequencerGroup, u8> + RegisterAbsIoW<SequencerGroup, u8>>
            super::SRX<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SRX")
                    .field("sequencer_index", &self.sequencer_index().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 2:0"]
            #[inline]
            pub fn sequencer_index(&self) -> SEQUENCER_INDEX_R {
                SEQUENCER_INDEX_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `sequencer_index`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SEQUENCER_INDEX_R {
            _Reserved(u8),
        }
        impl SEQUENCER_INDEX_R {
            const _MASK: u8 = 7;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    SEQUENCER_INDEX_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                SEQUENCER_INDEX_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    SEQUENCER_INDEX_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 2:0"]
            #[inline]
            pub fn sequencer_index(&mut self) -> _SEQUENCER_INDEX<'_> {
                _SEQUENCER_INDEX { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _SEQUENCER_INDEX<'a> {
            w: &'a mut W,
        }
        impl<'a> _SEQUENCER_INDEX<'a> {
            const _MASK: u8 = 7;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Sequencer Reset"]
    pub struct SR00<
        'a,
        T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod sr00 {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexR for super::SR00<'a, T>
        {
            const INDEX_R: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexW for super::SR00<'a, T>
        {
            const INDEX_W: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > InGroup for super::SR00<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > super::SR00<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SR00")
                    .field("synchronous_reset", &self.synchronous_reset().bit())
                    .field("asynchronous_reset", &self.asynchronous_reset().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 1"]
            #[inline]
            pub fn synchronous_reset(&self) -> SYNCHRONOUS_RESET_R {
                SYNCHRONOUS_RESET_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn asynchronous_reset(&self) -> ASYNCHRONOUS_RESET_R {
                ASYNCHRONOUS_RESET_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `synchronous_reset`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SYNCHRONOUS_RESET_R {
            _Reserved(bool),
        }
        impl SYNCHRONOUS_RESET_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SYNCHRONOUS_RESET_R::_Reserved(true) => Self::_MASK,
                    SYNCHRONOUS_RESET_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SYNCHRONOUS_RESET_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SYNCHRONOUS_RESET_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `asynchronous_reset`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ASYNCHRONOUS_RESET_R {
            _Reserved(bool),
        }
        impl ASYNCHRONOUS_RESET_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ASYNCHRONOUS_RESET_R::_Reserved(true) => Self::_MASK,
                    ASYNCHRONOUS_RESET_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ASYNCHRONOUS_RESET_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ASYNCHRONOUS_RESET_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 1"]
            #[inline]
            pub fn synchronous_reset(&mut self) -> _SYNCHRONOUS_RESET<'_> {
                _SYNCHRONOUS_RESET { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn asynchronous_reset(&mut self) -> _ASYNCHRONOUS_RESET<'_> {
                _ASYNCHRONOUS_RESET { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _SYNCHRONOUS_RESET<'a> {
            w: &'a mut W,
        }
        impl<'a> _SYNCHRONOUS_RESET<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ASYNCHRONOUS_RESET<'a> {
            w: &'a mut W,
        }
        impl<'a> _ASYNCHRONOUS_RESET<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Clocking Mode"]
    pub struct SR01<
        'a,
        T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod sr01 {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexR for super::SR01<'a, T>
        {
            const INDEX_R: u8 = 1;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexW for super::SR01<'a, T>
        {
            const INDEX_W: u8 = 1;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > InGroup for super::SR01<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > super::SR01<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SR01")
                    .field("screen_off", &self.screen_off().bit())
                    .field("shift_4", &self.shift_4().bit())
                    .field("dot_clock", &self.dot_clock().bit())
                    .field("shift_load", &self.shift_load().bit())
                    .field("dot_clocks_8_slash_9", &self.dot_clocks_8_slash_9().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 5"]
            #[inline]
            pub fn screen_off(&self) -> SCREEN_OFF_R {
                SCREEN_OFF_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn shift_4(&self) -> SHIFT_4_R {
                SHIFT_4_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn dot_clock(&self) -> DOT_CLOCK_R {
                DOT_CLOCK_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn shift_load(&self) -> SHIFT_LOAD_R {
                SHIFT_LOAD_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn dot_clocks_8_slash_9(&self) -> DOT_CLOCKS_8_SLASH_9_R {
                DOT_CLOCKS_8_SLASH_9_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `screen_off`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SCREEN_OFF_R {
            _Reserved(bool),
        }
        impl SCREEN_OFF_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SCREEN_OFF_R::_Reserved(true) => Self::_MASK,
                    SCREEN_OFF_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SCREEN_OFF_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SCREEN_OFF_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `shift_4`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SHIFT_4_R {
            _Reserved(bool),
        }
        impl SHIFT_4_R {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SHIFT_4_R::_Reserved(true) => Self::_MASK,
                    SHIFT_4_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SHIFT_4_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SHIFT_4_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `dot_clock`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DOT_CLOCK_R {
            _Reserved(bool),
        }
        impl DOT_CLOCK_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    DOT_CLOCK_R::_Reserved(true) => Self::_MASK,
                    DOT_CLOCK_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                DOT_CLOCK_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    DOT_CLOCK_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `shift_load`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SHIFT_LOAD_R {
            _Reserved(bool),
        }
        impl SHIFT_LOAD_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SHIFT_LOAD_R::_Reserved(true) => Self::_MASK,
                    SHIFT_LOAD_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SHIFT_LOAD_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SHIFT_LOAD_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `dot_clocks_8_slash_9`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DOT_CLOCKS_8_SLASH_9_R {
            _Reserved(bool),
        }
        impl DOT_CLOCKS_8_SLASH_9_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    DOT_CLOCKS_8_SLASH_9_R::_Reserved(true) => Self::_MASK,
                    DOT_CLOCKS_8_SLASH_9_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                DOT_CLOCKS_8_SLASH_9_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    DOT_CLOCKS_8_SLASH_9_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 5"]
            #[inline]
            pub fn screen_off(&mut self) -> _SCREEN_OFF<'_> {
                _SCREEN_OFF { w: self }
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn shift_4(&mut self) -> _SHIFT_4<'_> {
                _SHIFT_4 { w: self }
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn dot_clock(&mut self) -> _DOT_CLOCK<'_> {
                _DOT_CLOCK { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn shift_load(&mut self) -> _SHIFT_LOAD<'_> {
                _SHIFT_LOAD { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn dot_clocks_8_slash_9(&mut self) -> _DOT_CLOCKS_8_SLASH_9<'_> {
                _DOT_CLOCKS_8_SLASH_9 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _SCREEN_OFF<'a> {
            w: &'a mut W,
        }
        impl<'a> _SCREEN_OFF<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _SHIFT_4<'a> {
            w: &'a mut W,
        }
        impl<'a> _SHIFT_4<'a> {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _DOT_CLOCK<'a> {
            w: &'a mut W,
        }
        impl<'a> _DOT_CLOCK<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _SHIFT_LOAD<'a> {
            w: &'a mut W,
        }
        impl<'a> _SHIFT_LOAD<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _DOT_CLOCKS_8_SLASH_9<'a> {
            w: &'a mut W,
        }
        impl<'a> _DOT_CLOCKS_8_SLASH_9<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Plane/Map Mask"]
    pub struct SR02<
        'a,
        T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod sr02 {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexR for super::SR02<'a, T>
        {
            const INDEX_R: u8 = 2;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexW for super::SR02<'a, T>
        {
            const INDEX_W: u8 = 2;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > InGroup for super::SR02<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > super::SR02<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SR02")
                    .field("map_3_enable", &self.map_3_enable().bit())
                    .field("map_2_enable", &self.map_2_enable().bit())
                    .field("map_1_enable", &self.map_1_enable().bit())
                    .field("map_0_enable", &self.map_0_enable().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 3"]
            #[inline]
            pub fn map_3_enable(&self) -> MAP_3_ENABLE_R {
                MAP_3_ENABLE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn map_2_enable(&self) -> MAP_2_ENABLE_R {
                MAP_2_ENABLE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn map_1_enable(&self) -> MAP_1_ENABLE_R {
                MAP_1_ENABLE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn map_0_enable(&self) -> MAP_0_ENABLE_R {
                MAP_0_ENABLE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `map_3_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_3_ENABLE_R {
            _Reserved(bool),
        }
        impl MAP_3_ENABLE_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    MAP_3_ENABLE_R::_Reserved(true) => Self::_MASK,
                    MAP_3_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                MAP_3_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    MAP_3_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `map_2_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_2_ENABLE_R {
            _Reserved(bool),
        }
        impl MAP_2_ENABLE_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    MAP_2_ENABLE_R::_Reserved(true) => Self::_MASK,
                    MAP_2_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                MAP_2_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    MAP_2_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `map_1_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_1_ENABLE_R {
            _Reserved(bool),
        }
        impl MAP_1_ENABLE_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    MAP_1_ENABLE_R::_Reserved(true) => Self::_MASK,
                    MAP_1_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                MAP_1_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    MAP_1_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `map_0_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_0_ENABLE_R {
            _Reserved(bool),
        }
        impl MAP_0_ENABLE_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    MAP_0_ENABLE_R::_Reserved(true) => Self::_MASK,
                    MAP_0_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                MAP_0_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    MAP_0_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 3"]
            #[inline]
            pub fn map_3_enable(&mut self) -> _MAP_3_ENABLE<'_> {
                _MAP_3_ENABLE { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn map_2_enable(&mut self) -> _MAP_2_ENABLE<'_> {
                _MAP_2_ENABLE { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn map_1_enable(&mut self) -> _MAP_1_ENABLE<'_> {
                _MAP_1_ENABLE { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn map_0_enable(&mut self) -> _MAP_0_ENABLE<'_> {
                _MAP_0_ENABLE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_3_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_3_ENABLE<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_2_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_2_ENABLE<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_1_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_1_ENABLE<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_0_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_0_ENABLE<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Character Font"]
    pub struct SR03<
        'a,
        T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod sr03 {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexR for super::SR03<'a, T>
        {
            const INDEX_R: u8 = 3;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexW for super::SR03<'a, T>
        {
            const INDEX_W: u8 = 3;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > InGroup for super::SR03<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > super::SR03<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SR03")
                    .field("map_a_select_msb", &self.map_a_select_msb().bit())
                    .field("map_b_select_msb", &self.map_b_select_msb().bit())
                    .field("map_a_select", &self.map_a_select().bits())
                    .field("map_b_select", &self.map_b_select().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 5"]
            #[inline]
            pub fn map_a_select_msb(&self) -> MAP_A_SELECT_MSB_R {
                MAP_A_SELECT_MSB_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn map_b_select_msb(&self) -> MAP_B_SELECT_MSB_R {
                MAP_B_SELECT_MSB_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 3:2"]
            #[inline]
            pub fn map_a_select(&self) -> MAP_A_SELECT_R {
                MAP_A_SELECT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn map_b_select(&self) -> MAP_B_SELECT_R {
                MAP_B_SELECT_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `map_a_select_msb`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_A_SELECT_MSB_R {
            _Reserved(bool),
        }
        impl MAP_A_SELECT_MSB_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    MAP_A_SELECT_MSB_R::_Reserved(true) => Self::_MASK,
                    MAP_A_SELECT_MSB_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                MAP_A_SELECT_MSB_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    MAP_A_SELECT_MSB_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `map_b_select_msb`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_B_SELECT_MSB_R {
            _Reserved(bool),
        }
        impl MAP_B_SELECT_MSB_R {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    MAP_B_SELECT_MSB_R::_Reserved(true) => Self::_MASK,
                    MAP_B_SELECT_MSB_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                MAP_B_SELECT_MSB_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    MAP_B_SELECT_MSB_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `map_a_select`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_A_SELECT_R {
            _Reserved(u8),
        }
        impl MAP_A_SELECT_R {
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    MAP_A_SELECT_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                MAP_A_SELECT_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    MAP_A_SELECT_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Possible values of the field `map_b_select`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MAP_B_SELECT_R {
            _Reserved(u8),
        }
        impl MAP_B_SELECT_R {
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    MAP_B_SELECT_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                MAP_B_SELECT_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    MAP_B_SELECT_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 5"]
            #[inline]
            pub fn map_a_select_msb(&mut self) -> _MAP_A_SELECT_MSB<'_> {
                _MAP_A_SELECT_MSB { w: self }
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn map_b_select_msb(&mut self) -> _MAP_B_SELECT_MSB<'_> {
                _MAP_B_SELECT_MSB { w: self }
            }
            #[doc = "Bits 3:2"]
            #[inline]
            pub fn map_a_select(&mut self) -> _MAP_A_SELECT<'_> {
                _MAP_A_SELECT { w: self }
            }
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn map_b_select(&mut self) -> _MAP_B_SELECT<'_> {
                _MAP_B_SELECT { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_A_SELECT_MSB<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_A_SELECT_MSB<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_B_SELECT_MSB<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_B_SELECT_MSB<'a> {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_A_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_A_SELECT<'a> {
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _MAP_B_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _MAP_B_SELECT<'a> {
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Memory Mode Register"]
    pub struct SR04<
        'a,
        T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod sr04 {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexR for super::SR04<'a, T>
        {
            const INDEX_R: u8 = 4;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexW for super::SR04<'a, T>
        {
            const INDEX_W: u8 = 4;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > InGroup for super::SR04<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > super::SR04<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SR04")
                    .field("chain_4_mode", &self.chain_4_mode().bit())
                    .field("odd_slash_even_mode", &self.odd_slash_even_mode().bit())
                    .field(
                        "extended_memory_enable",
                        &self.extended_memory_enable().bit(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 3"]
            #[inline]
            pub fn chain_4_mode(&self) -> CHAIN_4_MODE_R {
                CHAIN_4_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn odd_slash_even_mode(&self) -> ODD_SLASH_EVEN_MODE_R {
                ODD_SLASH_EVEN_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn extended_memory_enable(&self) -> EXTENDED_MEMORY_ENABLE_R {
                EXTENDED_MEMORY_ENABLE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `chain_4_mode`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CHAIN_4_MODE_R {
            _Reserved(bool),
        }
        impl CHAIN_4_MODE_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    CHAIN_4_MODE_R::_Reserved(true) => Self::_MASK,
                    CHAIN_4_MODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                CHAIN_4_MODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    CHAIN_4_MODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `odd_slash_even_mode`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ODD_SLASH_EVEN_MODE_R {
            _Reserved(bool),
        }
        impl ODD_SLASH_EVEN_MODE_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ODD_SLASH_EVEN_MODE_R::_Reserved(true) => Self::_MASK,
                    ODD_SLASH_EVEN_MODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ODD_SLASH_EVEN_MODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ODD_SLASH_EVEN_MODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `extended_memory_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum EXTENDED_MEMORY_ENABLE_R {
            _Reserved(bool),
        }
        impl EXTENDED_MEMORY_ENABLE_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    EXTENDED_MEMORY_ENABLE_R::_Reserved(true) => Self::_MASK,
                    EXTENDED_MEMORY_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                EXTENDED_MEMORY_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    EXTENDED_MEMORY_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 3"]
            #[inline]
            pub fn chain_4_mode(&mut self) -> _CHAIN_4_MODE<'_> {
                _CHAIN_4_MODE { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn odd_slash_even_mode(&mut self) -> _ODD_SLASH_EVEN_MODE<'_> {
                _ODD_SLASH_EVEN_MODE { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn extended_memory_enable(&mut self) -> _EXTENDED_MEMORY_ENABLE<'_> {
                _EXTENDED_MEMORY_ENABLE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _CHAIN_4_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _CHAIN_4_MODE<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ODD_SLASH_EVEN_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _ODD_SLASH_EVEN_MODE<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _EXTENDED_MEMORY_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _EXTENDED_MEMORY_ENABLE<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Horizontal Character Counter Reset"]
    pub struct SR07<
        'a,
        T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod sr07 {
        use super::super::register_trait::*;
        use super::SequencerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexR for super::SR07<'a, T>
        {
            const INDEX_R: u8 = 7;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > LocationIndexW for super::SR07<'a, T>
        {
            const INDEX_W: u8 = 7;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > InGroup for super::SR07<'a, T>
        {
            type Group = SequencerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<SequencerGroup, u8> + RegisterIndexIoW<SequencerGroup, u8>,
            > super::SR07<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("SR07")
                    .field(
                        "horizontal_character_counter",
                        &self.horizontal_character_counter().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_character_counter(&self) -> HORIZONTAL_CHARACTER_COUNTER_R {
                HORIZONTAL_CHARACTER_COUNTER_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_character_counter`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_CHARACTER_COUNTER_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_CHARACTER_COUNTER_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_CHARACTER_COUNTER_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_CHARACTER_COUNTER_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_CHARACTER_COUNTER_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_character_counter(&mut self) -> _HORIZONTAL_CHARACTER_COUNTER<'_> {
                _HORIZONTAL_CHARACTER_COUNTER { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_CHARACTER_COUNTER<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_CHARACTER_COUNTER<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
}
pub mod crt_controller {
    use super::register_trait::*;
    pub struct CrtControllerRegisters<
        T: RegisterRelIoR<CrtControllerGroup, u8>
            + RegisterRelIoW<CrtControllerGroup, u8>
            + RegisterIndexIoR<CrtControllerGroup, u8>
            + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: T,
    }
    impl<
            T: RegisterRelIoR<CrtControllerGroup, u8>
                + RegisterRelIoW<CrtControllerGroup, u8>
                + RegisterIndexIoR<CrtControllerGroup, u8>
                + RegisterIndexIoW<CrtControllerGroup, u8>,
        > CrtControllerRegisters<T>
    {
        #[inline]
        pub fn new(io: T) -> Self {
            Self { io }
        }
        #[doc = "CRT Controller Index Register"]
        #[inline]
        pub fn crx(&mut self) -> CRX<'_, T> {
            CRX { io: &mut self.io }
        }
        #[doc = "Horizontal Total Register"]
        #[inline]
        pub fn cr00(&mut self) -> CR00<'_, T> {
            CR00 { io: &mut self.io }
        }
        #[doc = "Horizontal Display Enable End Register"]
        #[inline]
        pub fn cr01(&mut self) -> CR01<'_, T> {
            CR01 { io: &mut self.io }
        }
        #[doc = "Horizontal Blanking Start Register"]
        #[inline]
        pub fn cr02(&mut self) -> CR02<'_, T> {
            CR02 { io: &mut self.io }
        }
        #[doc = "Horizontal Blanking End Register"]
        #[inline]
        pub fn cr03(&mut self) -> CR03<'_, T> {
            CR03 { io: &mut self.io }
        }
        #[doc = "Horizontal Sync Start Register"]
        #[inline]
        pub fn cr04(&mut self) -> CR04<'_, T> {
            CR04 { io: &mut self.io }
        }
        #[doc = "Horizontal Sync End Register"]
        #[inline]
        pub fn cr05(&mut self) -> CR05<'_, T> {
            CR05 { io: &mut self.io }
        }
        #[doc = "Vertical Total Register"]
        #[inline]
        pub fn cr06(&mut self) -> CR06<'_, T> {
            CR06 { io: &mut self.io }
        }
        #[doc = "Overflow Register"]
        #[inline]
        pub fn cr07(&mut self) -> CR07<'_, T> {
            CR07 { io: &mut self.io }
        }
        #[doc = "Preset Row Scan Register"]
        #[inline]
        pub fn cr08(&mut self) -> CR08<'_, T> {
            CR08 { io: &mut self.io }
        }
        #[doc = "Maximum Scan Line Register"]
        #[inline]
        pub fn cr09(&mut self) -> CR09<'_, T> {
            CR09 { io: &mut self.io }
        }
        #[doc = "Text Cursor Start Register"]
        #[inline]
        pub fn cr0a(&mut self) -> CR0A<'_, T> {
            CR0A { io: &mut self.io }
        }
        #[doc = "Text Cursor End Register"]
        #[inline]
        pub fn cr0b(&mut self) -> CR0B<'_, T> {
            CR0B { io: &mut self.io }
        }
        #[doc = "Start Address High Register"]
        #[inline]
        pub fn cr0c(&mut self) -> CR0C<'_, T> {
            CR0C { io: &mut self.io }
        }
        #[doc = "Start Address Low Register"]
        #[inline]
        pub fn cr0d(&mut self) -> CR0D<'_, T> {
            CR0D { io: &mut self.io }
        }
        #[doc = "Text Cursor Location High Register"]
        #[inline]
        pub fn cr0e(&mut self) -> CR0E<'_, T> {
            CR0E { io: &mut self.io }
        }
        #[doc = "Text Cursor Location Low Register"]
        #[inline]
        pub fn cr0f(&mut self) -> CR0F<'_, T> {
            CR0F { io: &mut self.io }
        }
        #[doc = "Vertical Sync Start Register"]
        #[inline]
        pub fn cr10(&mut self) -> CR10<'_, T> {
            CR10 { io: &mut self.io }
        }
        #[doc = "Vertical Sync End Register"]
        #[inline]
        pub fn cr11(&mut self) -> CR11<'_, T> {
            CR11 { io: &mut self.io }
        }
        #[doc = "Vertical Display Enable End Register"]
        #[inline]
        pub fn cr12(&mut self) -> CR12<'_, T> {
            CR12 { io: &mut self.io }
        }
        #[doc = "Offset Register"]
        #[inline]
        pub fn cr13(&mut self) -> CR13<'_, T> {
            CR13 { io: &mut self.io }
        }
        #[doc = "Underline Location Register"]
        #[inline]
        pub fn cr14(&mut self) -> CR14<'_, T> {
            CR14 { io: &mut self.io }
        }
        #[doc = "Vertical Blanking Start Register"]
        #[inline]
        pub fn cr15(&mut self) -> CR15<'_, T> {
            CR15 { io: &mut self.io }
        }
        #[doc = "Vertical Blanking End Register"]
        #[inline]
        pub fn cr16(&mut self) -> CR16<'_, T> {
            CR16 { io: &mut self.io }
        }
        #[doc = "CRT Mode Control"]
        #[inline]
        pub fn cr17(&mut self) -> CR17<'_, T> {
            CR17 { io: &mut self.io }
        }
        #[doc = "Line Compare Register"]
        #[inline]
        pub fn cr18(&mut self) -> CR18<'_, T> {
            CR18 { io: &mut self.io }
        }
        pub fn debug_registers<F: FnMut(&dyn core::fmt::Debug)>(&mut self, mut f: F) {
            (f)(&self.crx().read());
            (f)(&self.cr00().read());
            (f)(&self.cr01().read());
            (f)(&self.cr02().read());
            (f)(&self.cr03().read());
            (f)(&self.cr04().read());
            (f)(&self.cr05().read());
            (f)(&self.cr06().read());
            (f)(&self.cr07().read());
            (f)(&self.cr08().read());
            (f)(&self.cr09().read());
            (f)(&self.cr0a().read());
            (f)(&self.cr0b().read());
            (f)(&self.cr0c().read());
            (f)(&self.cr0d().read());
            (f)(&self.cr0e().read());
            (f)(&self.cr0f().read());
            (f)(&self.cr10().read());
            (f)(&self.cr11().read());
            (f)(&self.cr12().read());
            (f)(&self.cr13().read());
            (f)(&self.cr14().read());
            (f)(&self.cr15().read());
            (f)(&self.cr16().read());
            (f)(&self.cr17().read());
            (f)(&self.cr18().read());
        }
    }
    pub struct CrtControllerGroup;
    impl RegisterGroup for CrtControllerGroup {}
    #[doc = "CRT Controller Index Register"]
    pub struct CRX<
        'a,
        T: RegisterRelIoR<CrtControllerGroup, u8> + RegisterRelIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod crx {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterRelIoR<CrtControllerGroup, u8> + RegisterRelIoW<CrtControllerGroup, u8>,
            > LocationRelR for super::CRX<'a, T>
        {
            const REL_ADDRESS_R: u16 = 948;
        }
        impl<
                'a,
                T: RegisterRelIoR<CrtControllerGroup, u8> + RegisterRelIoW<CrtControllerGroup, u8>,
            > LocationRelW for super::CRX<'a, T>
        {
            const REL_ADDRESS_W: u16 = 948;
        }
        impl<
                'a,
                T: RegisterRelIoR<CrtControllerGroup, u8> + RegisterRelIoW<CrtControllerGroup, u8>,
            > InGroup for super::CRX<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterRelIoR<CrtControllerGroup, u8> + RegisterRelIoW<CrtControllerGroup, u8>,
            > super::CRX<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::REL_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::REL_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CRX")
                    .field("crt_controller_index", &self.crt_controller_index().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn crt_controller_index(&self) -> CRT_CONTROLLER_INDEX_R {
                CRT_CONTROLLER_INDEX_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `crt_controller_index`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CRT_CONTROLLER_INDEX_R {
            _Reserved(u8),
        }
        impl CRT_CONTROLLER_INDEX_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    CRT_CONTROLLER_INDEX_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                CRT_CONTROLLER_INDEX_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    CRT_CONTROLLER_INDEX_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn crt_controller_index(&mut self) -> _CRT_CONTROLLER_INDEX<'_> {
                _CRT_CONTROLLER_INDEX { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _CRT_CONTROLLER_INDEX<'a> {
            w: &'a mut W,
        }
        impl<'a> _CRT_CONTROLLER_INDEX<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Horizontal Total Register"]
    pub struct CR00<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr00 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR00<'a, T>
        {
            const INDEX_R: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR00<'a, T>
        {
            const INDEX_W: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR00<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR00<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR00")
                    .field("horizontal_total", &self.horizontal_total().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_total(&self) -> HORIZONTAL_TOTAL_R {
                HORIZONTAL_TOTAL_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_total`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_TOTAL_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_TOTAL_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_TOTAL_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_TOTAL_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_TOTAL_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_total(&mut self) -> _HORIZONTAL_TOTAL<'_> {
                _HORIZONTAL_TOTAL { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_TOTAL<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_TOTAL<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Horizontal Display Enable End Register"]
    pub struct CR01<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr01 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR01<'a, T>
        {
            const INDEX_R: u8 = 1;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR01<'a, T>
        {
            const INDEX_W: u8 = 1;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR01<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR01<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR01")
                    .field(
                        "horizontal_display_enable_end",
                        &self.horizontal_display_enable_end().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_display_enable_end(&self) -> HORIZONTAL_DISPLAY_ENABLE_END_R {
                HORIZONTAL_DISPLAY_ENABLE_END_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_display_enable_end`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_DISPLAY_ENABLE_END_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_DISPLAY_ENABLE_END_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_DISPLAY_ENABLE_END_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_DISPLAY_ENABLE_END_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_DISPLAY_ENABLE_END_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_display_enable_end(&mut self) -> _HORIZONTAL_DISPLAY_ENABLE_END<'_> {
                _HORIZONTAL_DISPLAY_ENABLE_END { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_DISPLAY_ENABLE_END<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_DISPLAY_ENABLE_END<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Horizontal Blanking Start Register"]
    pub struct CR02<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr02 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR02<'a, T>
        {
            const INDEX_R: u8 = 2;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR02<'a, T>
        {
            const INDEX_W: u8 = 2;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR02<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR02<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR02")
                    .field(
                        "horizontal_blanking_start",
                        &self.horizontal_blanking_start().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_blanking_start(&self) -> HORIZONTAL_BLANKING_START_R {
                HORIZONTAL_BLANKING_START_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_blanking_start`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_BLANKING_START_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_BLANKING_START_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_BLANKING_START_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_BLANKING_START_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_BLANKING_START_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_blanking_start(&mut self) -> _HORIZONTAL_BLANKING_START<'_> {
                _HORIZONTAL_BLANKING_START { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_BLANKING_START<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_BLANKING_START<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Horizontal Blanking End Register"]
    pub struct CR03<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr03 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR03<'a, T>
        {
            const INDEX_R: u8 = 3;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR03<'a, T>
        {
            const INDEX_W: u8 = 3;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR03<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR03<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR03")
                    .field(
                        "display_enable_skew_control",
                        &format_args!("{:?}", self.display_enable_skew_control()),
                    )
                    .field(
                        "horizontal_blanking_end_bits_from_0_to_4",
                        &self.horizontal_blanking_end_bits_from_0_to_4().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn display_enable_skew_control(&self) -> DISPLAY_ENABLE_SKEW_CONTROL_R {
                DISPLAY_ENABLE_SKEW_CONTROL_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0 - Part 1/2 of a 6-bit value."]
            #[inline]
            pub fn horizontal_blanking_end_bits_from_0_to_4(
                &self,
            ) -> HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R {
                HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `display_enable_skew_control`"]
        #[doc = ""]
        #[doc = "Amount of skew."]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DISPLAY_ENABLE_SKEW_CONTROL_R {
            #[doc = "No character clock skew."]
            ZERO = 0,
            #[doc = "One character clock skew."]
            ONE = 1,
            #[doc = "Two character clock skew."]
            TWO = 2,
            #[doc = "Three character clock skew."]
            THREE = 3,
        }
        impl DISPLAY_ENABLE_SKEW_CONTROL_R {
            #[doc = "Checks if the value of the field is `ZERO`"]
            #[inline]
            pub fn is_zero(&self) -> bool {
                *self == DISPLAY_ENABLE_SKEW_CONTROL_R::ZERO
            }
            #[doc = "Checks if the value of the field is `ONE`"]
            #[inline]
            pub fn is_one(&self) -> bool {
                *self == DISPLAY_ENABLE_SKEW_CONTROL_R::ONE
            }
            #[doc = "Checks if the value of the field is `TWO`"]
            #[inline]
            pub fn is_two(&self) -> bool {
                *self == DISPLAY_ENABLE_SKEW_CONTROL_R::TWO
            }
            #[doc = "Checks if the value of the field is `THREE`"]
            #[inline]
            pub fn is_three(&self) -> bool {
                *self == DISPLAY_ENABLE_SKEW_CONTROL_R::THREE
            }
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => DISPLAY_ENABLE_SKEW_CONTROL_R::ZERO,
                    1 => DISPLAY_ENABLE_SKEW_CONTROL_R::ONE,
                    2 => DISPLAY_ENABLE_SKEW_CONTROL_R::TWO,
                    3 => DISPLAY_ENABLE_SKEW_CONTROL_R::THREE,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Possible values of the field `horizontal_blanking_end_bits_from_0_to_4`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn display_enable_skew_control(&mut self) -> _DISPLAY_ENABLE_SKEW_CONTROL<'_> {
                _DISPLAY_ENABLE_SKEW_CONTROL { w: self }
            }
            #[doc = "Bits 4:0 - Part 1/2 of a 6-bit value."]
            #[inline]
            pub fn horizontal_blanking_end_bits_from_0_to_4(
                &mut self,
            ) -> _HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4<'_> {
                _HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4 { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `display_enable_skew_control`"]
        #[doc = ""]
        #[doc = "Amount of skew."]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DISPLAY_ENABLE_SKEW_CONTROL_W {
            #[doc = "No character clock skew."]
            ZERO = 0,
            #[doc = "One character clock skew."]
            ONE = 1,
            #[doc = "Two character clock skew."]
            TWO = 2,
            #[doc = "Three character clock skew."]
            THREE = 3,
        }
        impl DISPLAY_ENABLE_SKEW_CONTROL_W {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _DISPLAY_ENABLE_SKEW_CONTROL<'a> {
            w: &'a mut W,
        }
        impl<'a> _DISPLAY_ENABLE_SKEW_CONTROL<'a> {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: DISPLAY_ENABLE_SKEW_CONTROL_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[doc = "No character clock skew."]
            #[inline]
            pub fn zero(self) -> &'a mut W {
                self.variant(DISPLAY_ENABLE_SKEW_CONTROL_W::ZERO)
            }
            #[doc = "One character clock skew."]
            #[inline]
            pub fn one(self) -> &'a mut W {
                self.variant(DISPLAY_ENABLE_SKEW_CONTROL_W::ONE)
            }
            #[doc = "Two character clock skew."]
            #[inline]
            pub fn two(self) -> &'a mut W {
                self.variant(DISPLAY_ENABLE_SKEW_CONTROL_W::TWO)
            }
            #[doc = "Three character clock skew."]
            #[inline]
            pub fn three(self) -> &'a mut W {
                self.variant(DISPLAY_ENABLE_SKEW_CONTROL_W::THREE)
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_BLANKING_END_BITS_FROM_0_TO_4<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Horizontal Sync Start Register"]
    pub struct CR04<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr04 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR04<'a, T>
        {
            const INDEX_R: u8 = 4;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR04<'a, T>
        {
            const INDEX_W: u8 = 4;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR04<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR04<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR04")
                    .field(
                        "horizontal_sync_start",
                        &self.horizontal_sync_start().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_sync_start(&self) -> HORIZONTAL_SYNC_START_R {
                HORIZONTAL_SYNC_START_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_sync_start`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_SYNC_START_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_SYNC_START_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_SYNC_START_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_SYNC_START_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_SYNC_START_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn horizontal_sync_start(&mut self) -> _HORIZONTAL_SYNC_START<'_> {
                _HORIZONTAL_SYNC_START { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_SYNC_START<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_SYNC_START<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Horizontal Sync End Register"]
    pub struct CR05<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr05 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR05<'a, T>
        {
            const INDEX_R: u8 = 5;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR05<'a, T>
        {
            const INDEX_W: u8 = 5;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR05<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR05<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR05")
                    .field(
                        "horizontal_blanking_end_bit_5",
                        &self.horizontal_blanking_end_bit_5().bit(),
                    )
                    .field(
                        "horizontal_sync_delay",
                        &self.horizontal_sync_delay().bits(),
                    )
                    .field("horizontal_sync_end", &self.horizontal_sync_end().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7 - Part 2/2 of a 6-bit value."]
            #[inline]
            pub fn horizontal_blanking_end_bit_5(&self) -> HORIZONTAL_BLANKING_END_BIT_5_R {
                HORIZONTAL_BLANKING_END_BIT_5_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn horizontal_sync_delay(&self) -> HORIZONTAL_SYNC_DELAY_R {
                HORIZONTAL_SYNC_DELAY_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn horizontal_sync_end(&self) -> HORIZONTAL_SYNC_END_R {
                HORIZONTAL_SYNC_END_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_blanking_end_bit_5`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_BLANKING_END_BIT_5_R {
            _Reserved(bool),
        }
        impl HORIZONTAL_BLANKING_END_BIT_5_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    HORIZONTAL_BLANKING_END_BIT_5_R::_Reserved(true) => Self::_MASK,
                    HORIZONTAL_BLANKING_END_BIT_5_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                HORIZONTAL_BLANKING_END_BIT_5_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    HORIZONTAL_BLANKING_END_BIT_5_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `horizontal_sync_delay`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_SYNC_DELAY_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_SYNC_DELAY_R {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_SYNC_DELAY_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_SYNC_DELAY_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_SYNC_DELAY_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Possible values of the field `horizontal_sync_end`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_SYNC_END_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_SYNC_END_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_SYNC_END_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_SYNC_END_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_SYNC_END_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 7 - Part 2/2 of a 6-bit value."]
            #[inline]
            pub fn horizontal_blanking_end_bit_5(&mut self) -> _HORIZONTAL_BLANKING_END_BIT_5<'_> {
                _HORIZONTAL_BLANKING_END_BIT_5 { w: self }
            }
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn horizontal_sync_delay(&mut self) -> _HORIZONTAL_SYNC_DELAY<'_> {
                _HORIZONTAL_SYNC_DELAY { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn horizontal_sync_end(&mut self) -> _HORIZONTAL_SYNC_END<'_> {
                _HORIZONTAL_SYNC_END { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_BLANKING_END_BIT_5<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_BLANKING_END_BIT_5<'a> {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_SYNC_DELAY<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_SYNC_DELAY<'a> {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_SYNC_END<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_SYNC_END<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Vertical Total Register"]
    pub struct CR06<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr06 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR06<'a, T>
        {
            const INDEX_R: u8 = 6;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR06<'a, T>
        {
            const INDEX_W: u8 = 6;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR06<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR06<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR06")
                    .field(
                        "vertical_total_bits_from_0_to_7",
                        &self.vertical_total_bits_from_0_to_7().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_total_bits_from_0_to_7(&self) -> VERTICAL_TOTAL_BITS_FROM_0_TO_7_R {
                VERTICAL_TOTAL_BITS_FROM_0_TO_7_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_total_bits_from_0_to_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_TOTAL_BITS_FROM_0_TO_7_R {
            _Reserved(u8),
        }
        impl VERTICAL_TOTAL_BITS_FROM_0_TO_7_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    VERTICAL_TOTAL_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                VERTICAL_TOTAL_BITS_FROM_0_TO_7_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    VERTICAL_TOTAL_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_total_bits_from_0_to_7(
                &mut self,
            ) -> _VERTICAL_TOTAL_BITS_FROM_0_TO_7<'_> {
                _VERTICAL_TOTAL_BITS_FROM_0_TO_7 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_TOTAL_BITS_FROM_0_TO_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_TOTAL_BITS_FROM_0_TO_7<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Overflow Register"]
    pub struct CR07<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr07 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR07<'a, T>
        {
            const INDEX_R: u8 = 7;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR07<'a, T>
        {
            const INDEX_W: u8 = 7;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR07<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR07<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR07")
                    .field(
                        "vertical_sync_start_bit_9",
                        &self.vertical_sync_start_bit_9().bit(),
                    )
                    .field(
                        "vertical_display_enable_end_bit_9",
                        &self.vertical_display_enable_end_bit_9().bit(),
                    )
                    .field("vertical_total_bit_9", &self.vertical_total_bit_9().bit())
                    .field("line_compare_bit_8", &self.line_compare_bit_8().bit())
                    .field(
                        "vertical_blanking_start_bit_8",
                        &self.vertical_blanking_start_bit_8().bit(),
                    )
                    .field(
                        "vertical_sync_start_bit_8",
                        &self.vertical_sync_start_bit_8().bit(),
                    )
                    .field(
                        "vertical_display_enable_end_bit_8",
                        &self.vertical_display_enable_end_bit_8().bit(),
                    )
                    .field("vertical_total_bit_8", &self.vertical_total_bit_8().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_sync_start_bit_9(&self) -> VERTICAL_SYNC_START_BIT_9_R {
                VERTICAL_SYNC_START_BIT_9_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 6 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_display_enable_end_bit_9(&self) -> VERTICAL_DISPLAY_ENABLE_END_BIT_9_R {
                VERTICAL_DISPLAY_ENABLE_END_BIT_9_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 5 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_total_bit_9(&self) -> VERTICAL_TOTAL_BIT_9_R {
                VERTICAL_TOTAL_BIT_9_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 4 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn line_compare_bit_8(&self) -> LINE_COMPARE_BIT_8_R {
                LINE_COMPARE_BIT_8_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 3 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_blanking_start_bit_8(&self) -> VERTICAL_BLANKING_START_BIT_8_R {
                VERTICAL_BLANKING_START_BIT_8_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_sync_start_bit_8(&self) -> VERTICAL_SYNC_START_BIT_8_R {
                VERTICAL_SYNC_START_BIT_8_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_display_enable_end_bit_8(&self) -> VERTICAL_DISPLAY_ENABLE_END_BIT_8_R {
                VERTICAL_DISPLAY_ENABLE_END_BIT_8_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_total_bit_8(&self) -> VERTICAL_TOTAL_BIT_8_R {
                VERTICAL_TOTAL_BIT_8_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_sync_start_bit_9`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_SYNC_START_BIT_9_R {
            _Reserved(bool),
        }
        impl VERTICAL_SYNC_START_BIT_9_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_SYNC_START_BIT_9_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_SYNC_START_BIT_9_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_SYNC_START_BIT_9_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_SYNC_START_BIT_9_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_display_enable_end_bit_9`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_DISPLAY_ENABLE_END_BIT_9_R {
            _Reserved(bool),
        }
        impl VERTICAL_DISPLAY_ENABLE_END_BIT_9_R {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_DISPLAY_ENABLE_END_BIT_9_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_DISPLAY_ENABLE_END_BIT_9_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_DISPLAY_ENABLE_END_BIT_9_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_DISPLAY_ENABLE_END_BIT_9_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_total_bit_9`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_TOTAL_BIT_9_R {
            _Reserved(bool),
        }
        impl VERTICAL_TOTAL_BIT_9_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_TOTAL_BIT_9_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_TOTAL_BIT_9_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_TOTAL_BIT_9_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_TOTAL_BIT_9_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `line_compare_bit_8`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum LINE_COMPARE_BIT_8_R {
            _Reserved(bool),
        }
        impl LINE_COMPARE_BIT_8_R {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    LINE_COMPARE_BIT_8_R::_Reserved(true) => Self::_MASK,
                    LINE_COMPARE_BIT_8_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                LINE_COMPARE_BIT_8_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    LINE_COMPARE_BIT_8_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_blanking_start_bit_8`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_BLANKING_START_BIT_8_R {
            _Reserved(bool),
        }
        impl VERTICAL_BLANKING_START_BIT_8_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_BLANKING_START_BIT_8_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_BLANKING_START_BIT_8_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_BLANKING_START_BIT_8_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_BLANKING_START_BIT_8_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_sync_start_bit_8`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_SYNC_START_BIT_8_R {
            _Reserved(bool),
        }
        impl VERTICAL_SYNC_START_BIT_8_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_SYNC_START_BIT_8_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_SYNC_START_BIT_8_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_SYNC_START_BIT_8_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_SYNC_START_BIT_8_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_display_enable_end_bit_8`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_DISPLAY_ENABLE_END_BIT_8_R {
            _Reserved(bool),
        }
        impl VERTICAL_DISPLAY_ENABLE_END_BIT_8_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_DISPLAY_ENABLE_END_BIT_8_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_DISPLAY_ENABLE_END_BIT_8_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_DISPLAY_ENABLE_END_BIT_8_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_DISPLAY_ENABLE_END_BIT_8_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_total_bit_8`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_TOTAL_BIT_8_R {
            _Reserved(bool),
        }
        impl VERTICAL_TOTAL_BIT_8_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_TOTAL_BIT_8_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_TOTAL_BIT_8_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_TOTAL_BIT_8_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_TOTAL_BIT_8_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 7 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_sync_start_bit_9(&mut self) -> _VERTICAL_SYNC_START_BIT_9<'_> {
                _VERTICAL_SYNC_START_BIT_9 { w: self }
            }
            #[doc = "Bit 6 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_display_enable_end_bit_9(
                &mut self,
            ) -> _VERTICAL_DISPLAY_ENABLE_END_BIT_9<'_> {
                _VERTICAL_DISPLAY_ENABLE_END_BIT_9 { w: self }
            }
            #[doc = "Bit 5 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_total_bit_9(&mut self) -> _VERTICAL_TOTAL_BIT_9<'_> {
                _VERTICAL_TOTAL_BIT_9 { w: self }
            }
            #[doc = "Bit 4 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn line_compare_bit_8(&mut self) -> _LINE_COMPARE_BIT_8<'_> {
                _LINE_COMPARE_BIT_8 { w: self }
            }
            #[doc = "Bit 3 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_blanking_start_bit_8(&mut self) -> _VERTICAL_BLANKING_START_BIT_8<'_> {
                _VERTICAL_BLANKING_START_BIT_8 { w: self }
            }
            #[doc = "Bit 2 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_sync_start_bit_8(&mut self) -> _VERTICAL_SYNC_START_BIT_8<'_> {
                _VERTICAL_SYNC_START_BIT_8 { w: self }
            }
            #[doc = "Bit 1 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_display_enable_end_bit_8(
                &mut self,
            ) -> _VERTICAL_DISPLAY_ENABLE_END_BIT_8<'_> {
                _VERTICAL_DISPLAY_ENABLE_END_BIT_8 { w: self }
            }
            #[doc = "Bit 0 - Part 2/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_total_bit_8(&mut self) -> _VERTICAL_TOTAL_BIT_8<'_> {
                _VERTICAL_TOTAL_BIT_8 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_SYNC_START_BIT_9<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_SYNC_START_BIT_9<'a> {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_DISPLAY_ENABLE_END_BIT_9<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_DISPLAY_ENABLE_END_BIT_9<'a> {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_TOTAL_BIT_9<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_TOTAL_BIT_9<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _LINE_COMPARE_BIT_8<'a> {
            w: &'a mut W,
        }
        impl<'a> _LINE_COMPARE_BIT_8<'a> {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_BLANKING_START_BIT_8<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_BLANKING_START_BIT_8<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_SYNC_START_BIT_8<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_SYNC_START_BIT_8<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_DISPLAY_ENABLE_END_BIT_8<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_DISPLAY_ENABLE_END_BIT_8<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_TOTAL_BIT_8<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_TOTAL_BIT_8<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Preset Row Scan Register"]
    pub struct CR08<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr08 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR08<'a, T>
        {
            const INDEX_R: u8 = 8;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR08<'a, T>
        {
            const INDEX_W: u8 = 8;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR08<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR08<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR08")
                    .field("byte_panning", &format_args!("{:?}", self.byte_panning()))
                    .field(
                        "starting_row_scan_count",
                        &self.starting_row_scan_count().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn byte_panning(&self) -> BYTE_PANNING_R {
                BYTE_PANNING_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn starting_row_scan_count(&self) -> STARTING_ROW_SCAN_COUNT_R {
                STARTING_ROW_SCAN_COUNT_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `byte_panning`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum BYTE_PANNING_R {
            ZERO = 0,
            ONE = 1,
            TWO = 2,
            THREE = 3,
        }
        impl BYTE_PANNING_R {
            #[doc = "Checks if the value of the field is `ZERO`"]
            #[inline]
            pub fn is_zero(&self) -> bool {
                *self == BYTE_PANNING_R::ZERO
            }
            #[doc = "Checks if the value of the field is `ONE`"]
            #[inline]
            pub fn is_one(&self) -> bool {
                *self == BYTE_PANNING_R::ONE
            }
            #[doc = "Checks if the value of the field is `TWO`"]
            #[inline]
            pub fn is_two(&self) -> bool {
                *self == BYTE_PANNING_R::TWO
            }
            #[doc = "Checks if the value of the field is `THREE`"]
            #[inline]
            pub fn is_three(&self) -> bool {
                *self == BYTE_PANNING_R::THREE
            }
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => BYTE_PANNING_R::ZERO,
                    1 => BYTE_PANNING_R::ONE,
                    2 => BYTE_PANNING_R::TWO,
                    3 => BYTE_PANNING_R::THREE,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Possible values of the field `starting_row_scan_count`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum STARTING_ROW_SCAN_COUNT_R {
            _Reserved(u8),
        }
        impl STARTING_ROW_SCAN_COUNT_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    STARTING_ROW_SCAN_COUNT_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                STARTING_ROW_SCAN_COUNT_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    STARTING_ROW_SCAN_COUNT_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn byte_panning(&mut self) -> _BYTE_PANNING<'_> {
                _BYTE_PANNING { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn starting_row_scan_count(&mut self) -> _STARTING_ROW_SCAN_COUNT<'_> {
                _STARTING_ROW_SCAN_COUNT { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `byte_panning`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum BYTE_PANNING_W {
            ZERO = 0,
            ONE = 1,
            TWO = 2,
            THREE = 3,
        }
        impl BYTE_PANNING_W {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _BYTE_PANNING<'a> {
            w: &'a mut W,
        }
        impl<'a> _BYTE_PANNING<'a> {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: BYTE_PANNING_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn zero(self) -> &'a mut W {
                self.variant(BYTE_PANNING_W::ZERO)
            }
            #[inline]
            pub fn one(self) -> &'a mut W {
                self.variant(BYTE_PANNING_W::ONE)
            }
            #[inline]
            pub fn two(self) -> &'a mut W {
                self.variant(BYTE_PANNING_W::TWO)
            }
            #[inline]
            pub fn three(self) -> &'a mut W {
                self.variant(BYTE_PANNING_W::THREE)
            }
        }
        #[doc = "Proxy"]
        pub struct _STARTING_ROW_SCAN_COUNT<'a> {
            w: &'a mut W,
        }
        impl<'a> _STARTING_ROW_SCAN_COUNT<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Maximum Scan Line Register"]
    pub struct CR09<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr09 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR09<'a, T>
        {
            const INDEX_R: u8 = 9;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR09<'a, T>
        {
            const INDEX_W: u8 = 9;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR09<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR09<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR09")
                    .field(
                        "double_scanning_enable",
                        &self.double_scanning_enable().bit(),
                    )
                    .field("line_compare_bit_9", &self.line_compare_bit_9().bit())
                    .field(
                        "vertical_blanking_start_bit_9",
                        &self.vertical_blanking_start_bit_9().bit(),
                    )
                    .field(
                        "starting_row_scan_count",
                        &self.starting_row_scan_count().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7"]
            #[inline]
            pub fn double_scanning_enable(&self) -> DOUBLE_SCANNING_ENABLE_R {
                DOUBLE_SCANNING_ENABLE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 6 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn line_compare_bit_9(&self) -> LINE_COMPARE_BIT_9_R {
                LINE_COMPARE_BIT_9_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 5 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_blanking_start_bit_9(&self) -> VERTICAL_BLANKING_START_BIT_9_R {
                VERTICAL_BLANKING_START_BIT_9_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn starting_row_scan_count(&self) -> STARTING_ROW_SCAN_COUNT_R {
                STARTING_ROW_SCAN_COUNT_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `double_scanning_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DOUBLE_SCANNING_ENABLE_R {
            _Reserved(bool),
        }
        impl DOUBLE_SCANNING_ENABLE_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    DOUBLE_SCANNING_ENABLE_R::_Reserved(true) => Self::_MASK,
                    DOUBLE_SCANNING_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                DOUBLE_SCANNING_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    DOUBLE_SCANNING_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `line_compare_bit_9`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum LINE_COMPARE_BIT_9_R {
            _Reserved(bool),
        }
        impl LINE_COMPARE_BIT_9_R {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    LINE_COMPARE_BIT_9_R::_Reserved(true) => Self::_MASK,
                    LINE_COMPARE_BIT_9_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                LINE_COMPARE_BIT_9_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    LINE_COMPARE_BIT_9_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_blanking_start_bit_9`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_BLANKING_START_BIT_9_R {
            _Reserved(bool),
        }
        impl VERTICAL_BLANKING_START_BIT_9_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_BLANKING_START_BIT_9_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_BLANKING_START_BIT_9_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_BLANKING_START_BIT_9_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_BLANKING_START_BIT_9_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `starting_row_scan_count`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum STARTING_ROW_SCAN_COUNT_R {
            _Reserved(u8),
        }
        impl STARTING_ROW_SCAN_COUNT_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    STARTING_ROW_SCAN_COUNT_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                STARTING_ROW_SCAN_COUNT_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    STARTING_ROW_SCAN_COUNT_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 7"]
            #[inline]
            pub fn double_scanning_enable(&mut self) -> _DOUBLE_SCANNING_ENABLE<'_> {
                _DOUBLE_SCANNING_ENABLE { w: self }
            }
            #[doc = "Bit 6 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn line_compare_bit_9(&mut self) -> _LINE_COMPARE_BIT_9<'_> {
                _LINE_COMPARE_BIT_9 { w: self }
            }
            #[doc = "Bit 5 - Part 3/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_blanking_start_bit_9(&mut self) -> _VERTICAL_BLANKING_START_BIT_9<'_> {
                _VERTICAL_BLANKING_START_BIT_9 { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn starting_row_scan_count(&mut self) -> _STARTING_ROW_SCAN_COUNT<'_> {
                _STARTING_ROW_SCAN_COUNT { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _DOUBLE_SCANNING_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _DOUBLE_SCANNING_ENABLE<'a> {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _LINE_COMPARE_BIT_9<'a> {
            w: &'a mut W,
        }
        impl<'a> _LINE_COMPARE_BIT_9<'a> {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_BLANKING_START_BIT_9<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_BLANKING_START_BIT_9<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _STARTING_ROW_SCAN_COUNT<'a> {
            w: &'a mut W,
        }
        impl<'a> _STARTING_ROW_SCAN_COUNT<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Text Cursor Start Register"]
    pub struct CR0A<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr0a {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR0A<'a, T>
        {
            const INDEX_R: u8 = 10;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR0A<'a, T>
        {
            const INDEX_W: u8 = 10;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR0A<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR0A<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR0A")
                    .field("text_cursor_off", &self.text_cursor_off().bit())
                    .field("text_cursor_start", &self.text_cursor_start().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 5"]
            #[inline]
            pub fn text_cursor_off(&self) -> TEXT_CURSOR_OFF_R {
                TEXT_CURSOR_OFF_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn text_cursor_start(&self) -> TEXT_CURSOR_START_R {
                TEXT_CURSOR_START_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `text_cursor_off`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_OFF_R {
            _Reserved(bool),
        }
        impl TEXT_CURSOR_OFF_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    TEXT_CURSOR_OFF_R::_Reserved(true) => Self::_MASK,
                    TEXT_CURSOR_OFF_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                TEXT_CURSOR_OFF_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    TEXT_CURSOR_OFF_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `text_cursor_start`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_START_R {
            _Reserved(u8),
        }
        impl TEXT_CURSOR_START_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    TEXT_CURSOR_START_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                TEXT_CURSOR_START_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    TEXT_CURSOR_START_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 5"]
            #[inline]
            pub fn text_cursor_off(&mut self) -> _TEXT_CURSOR_OFF<'_> {
                _TEXT_CURSOR_OFF { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn text_cursor_start(&mut self) -> _TEXT_CURSOR_START<'_> {
                _TEXT_CURSOR_START { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _TEXT_CURSOR_OFF<'a> {
            w: &'a mut W,
        }
        impl<'a> _TEXT_CURSOR_OFF<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _TEXT_CURSOR_START<'a> {
            w: &'a mut W,
        }
        impl<'a> _TEXT_CURSOR_START<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Text Cursor End Register"]
    pub struct CR0B<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr0b {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR0B<'a, T>
        {
            const INDEX_R: u8 = 11;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR0B<'a, T>
        {
            const INDEX_W: u8 = 11;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR0B<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR0B<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR0B")
                    .field(
                        "text_cursor_skew",
                        &format_args!("{:?}", self.text_cursor_skew()),
                    )
                    .field("text_cursor_end", &self.text_cursor_end().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn text_cursor_skew(&self) -> TEXT_CURSOR_SKEW_R {
                TEXT_CURSOR_SKEW_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn text_cursor_end(&self) -> TEXT_CURSOR_END_R {
                TEXT_CURSOR_END_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `text_cursor_skew`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_SKEW_R {
            #[doc = "No delay."]
            ZERO = 0,
            #[doc = "Delayed by 1 character clock."]
            ONE = 1,
            #[doc = "Delayed by 2 character clocks."]
            TWO = 2,
            #[doc = "Delayed by 3 character clock."]
            THREE = 3,
        }
        impl TEXT_CURSOR_SKEW_R {
            #[doc = "Checks if the value of the field is `ZERO`"]
            #[inline]
            pub fn is_zero(&self) -> bool {
                *self == TEXT_CURSOR_SKEW_R::ZERO
            }
            #[doc = "Checks if the value of the field is `ONE`"]
            #[inline]
            pub fn is_one(&self) -> bool {
                *self == TEXT_CURSOR_SKEW_R::ONE
            }
            #[doc = "Checks if the value of the field is `TWO`"]
            #[inline]
            pub fn is_two(&self) -> bool {
                *self == TEXT_CURSOR_SKEW_R::TWO
            }
            #[doc = "Checks if the value of the field is `THREE`"]
            #[inline]
            pub fn is_three(&self) -> bool {
                *self == TEXT_CURSOR_SKEW_R::THREE
            }
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => TEXT_CURSOR_SKEW_R::ZERO,
                    1 => TEXT_CURSOR_SKEW_R::ONE,
                    2 => TEXT_CURSOR_SKEW_R::TWO,
                    3 => TEXT_CURSOR_SKEW_R::THREE,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Possible values of the field `text_cursor_end`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_END_R {
            _Reserved(u8),
        }
        impl TEXT_CURSOR_END_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    TEXT_CURSOR_END_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                TEXT_CURSOR_END_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    TEXT_CURSOR_END_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn text_cursor_skew(&mut self) -> _TEXT_CURSOR_SKEW<'_> {
                _TEXT_CURSOR_SKEW { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn text_cursor_end(&mut self) -> _TEXT_CURSOR_END<'_> {
                _TEXT_CURSOR_END { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `text_cursor_skew`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_SKEW_W {
            #[doc = "No delay."]
            ZERO = 0,
            #[doc = "Delayed by 1 character clock."]
            ONE = 1,
            #[doc = "Delayed by 2 character clocks."]
            TWO = 2,
            #[doc = "Delayed by 3 character clock."]
            THREE = 3,
        }
        impl TEXT_CURSOR_SKEW_W {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _TEXT_CURSOR_SKEW<'a> {
            w: &'a mut W,
        }
        impl<'a> _TEXT_CURSOR_SKEW<'a> {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: TEXT_CURSOR_SKEW_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[doc = "No delay."]
            #[inline]
            pub fn zero(self) -> &'a mut W {
                self.variant(TEXT_CURSOR_SKEW_W::ZERO)
            }
            #[doc = "Delayed by 1 character clock."]
            #[inline]
            pub fn one(self) -> &'a mut W {
                self.variant(TEXT_CURSOR_SKEW_W::ONE)
            }
            #[doc = "Delayed by 2 character clocks."]
            #[inline]
            pub fn two(self) -> &'a mut W {
                self.variant(TEXT_CURSOR_SKEW_W::TWO)
            }
            #[doc = "Delayed by 3 character clock."]
            #[inline]
            pub fn three(self) -> &'a mut W {
                self.variant(TEXT_CURSOR_SKEW_W::THREE)
            }
        }
        #[doc = "Proxy"]
        pub struct _TEXT_CURSOR_END<'a> {
            w: &'a mut W,
        }
        impl<'a> _TEXT_CURSOR_END<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Start Address High Register"]
    pub struct CR0C<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr0c {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR0C<'a, T>
        {
            const INDEX_R: u8 = 12;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR0C<'a, T>
        {
            const INDEX_W: u8 = 12;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR0C<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR0C<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR0C")
                    .field(
                        "start_address_high_byte",
                        &self.start_address_high_byte().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 2/2 of a 16-bit value."]
            #[inline]
            pub fn start_address_high_byte(&self) -> START_ADDRESS_HIGH_BYTE_R {
                START_ADDRESS_HIGH_BYTE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `start_address_high_byte`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum START_ADDRESS_HIGH_BYTE_R {
            _Reserved(u8),
        }
        impl START_ADDRESS_HIGH_BYTE_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    START_ADDRESS_HIGH_BYTE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                START_ADDRESS_HIGH_BYTE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    START_ADDRESS_HIGH_BYTE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 2/2 of a 16-bit value."]
            #[inline]
            pub fn start_address_high_byte(&mut self) -> _START_ADDRESS_HIGH_BYTE<'_> {
                _START_ADDRESS_HIGH_BYTE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _START_ADDRESS_HIGH_BYTE<'a> {
            w: &'a mut W,
        }
        impl<'a> _START_ADDRESS_HIGH_BYTE<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Start Address Low Register"]
    pub struct CR0D<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr0d {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR0D<'a, T>
        {
            const INDEX_R: u8 = 13;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR0D<'a, T>
        {
            const INDEX_W: u8 = 13;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR0D<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR0D<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR0D")
                    .field(
                        "start_address_low_byte",
                        &self.start_address_low_byte().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/2 of a 16-bit value."]
            #[inline]
            pub fn start_address_low_byte(&self) -> START_ADDRESS_LOW_BYTE_R {
                START_ADDRESS_LOW_BYTE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `start_address_low_byte`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum START_ADDRESS_LOW_BYTE_R {
            _Reserved(u8),
        }
        impl START_ADDRESS_LOW_BYTE_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    START_ADDRESS_LOW_BYTE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                START_ADDRESS_LOW_BYTE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    START_ADDRESS_LOW_BYTE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/2 of a 16-bit value."]
            #[inline]
            pub fn start_address_low_byte(&mut self) -> _START_ADDRESS_LOW_BYTE<'_> {
                _START_ADDRESS_LOW_BYTE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _START_ADDRESS_LOW_BYTE<'a> {
            w: &'a mut W,
        }
        impl<'a> _START_ADDRESS_LOW_BYTE<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Text Cursor Location High Register"]
    pub struct CR0E<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr0e {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR0E<'a, T>
        {
            const INDEX_R: u8 = 14;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR0E<'a, T>
        {
            const INDEX_W: u8 = 14;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR0E<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR0E<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR0E")
                    .field(
                        "text_cursor_location_high_byte",
                        &self.text_cursor_location_high_byte().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 2/2 of a 16-bit value."]
            #[inline]
            pub fn text_cursor_location_high_byte(&self) -> TEXT_CURSOR_LOCATION_HIGH_BYTE_R {
                TEXT_CURSOR_LOCATION_HIGH_BYTE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `text_cursor_location_high_byte`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_LOCATION_HIGH_BYTE_R {
            _Reserved(u8),
        }
        impl TEXT_CURSOR_LOCATION_HIGH_BYTE_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    TEXT_CURSOR_LOCATION_HIGH_BYTE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                TEXT_CURSOR_LOCATION_HIGH_BYTE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    TEXT_CURSOR_LOCATION_HIGH_BYTE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 2/2 of a 16-bit value."]
            #[inline]
            pub fn text_cursor_location_high_byte(
                &mut self,
            ) -> _TEXT_CURSOR_LOCATION_HIGH_BYTE<'_> {
                _TEXT_CURSOR_LOCATION_HIGH_BYTE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _TEXT_CURSOR_LOCATION_HIGH_BYTE<'a> {
            w: &'a mut W,
        }
        impl<'a> _TEXT_CURSOR_LOCATION_HIGH_BYTE<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Text Cursor Location Low Register"]
    pub struct CR0F<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr0f {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR0F<'a, T>
        {
            const INDEX_R: u8 = 15;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR0F<'a, T>
        {
            const INDEX_W: u8 = 15;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR0F<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR0F<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR0F")
                    .field(
                        "text_cursor_location_low_byte",
                        &self.text_cursor_location_low_byte().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/2 of a 16-bit value."]
            #[inline]
            pub fn text_cursor_location_low_byte(&self) -> TEXT_CURSOR_LOCATION_LOW_BYTE_R {
                TEXT_CURSOR_LOCATION_LOW_BYTE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `text_cursor_location_low_byte`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TEXT_CURSOR_LOCATION_LOW_BYTE_R {
            _Reserved(u8),
        }
        impl TEXT_CURSOR_LOCATION_LOW_BYTE_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    TEXT_CURSOR_LOCATION_LOW_BYTE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                TEXT_CURSOR_LOCATION_LOW_BYTE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    TEXT_CURSOR_LOCATION_LOW_BYTE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/2 of a 16-bit value."]
            #[inline]
            pub fn text_cursor_location_low_byte(&mut self) -> _TEXT_CURSOR_LOCATION_LOW_BYTE<'_> {
                _TEXT_CURSOR_LOCATION_LOW_BYTE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _TEXT_CURSOR_LOCATION_LOW_BYTE<'a> {
            w: &'a mut W,
        }
        impl<'a> _TEXT_CURSOR_LOCATION_LOW_BYTE<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Vertical Sync Start Register"]
    pub struct CR10<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr10 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR10<'a, T>
        {
            const INDEX_R: u8 = 16;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR10<'a, T>
        {
            const INDEX_W: u8 = 16;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR10<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR10<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR10")
                    .field(
                        "vertical_sync_start_bits_from_0_to_7",
                        &self.vertical_sync_start_bits_from_0_to_7().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_sync_start_bits_from_0_to_7(
                &self,
            ) -> VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R {
                VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_sync_start_bits_from_0_to_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R {
            _Reserved(u8),
        }
        impl VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    VERTICAL_SYNC_START_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_sync_start_bits_from_0_to_7(
                &mut self,
            ) -> _VERTICAL_SYNC_START_BITS_FROM_0_TO_7<'_> {
                _VERTICAL_SYNC_START_BITS_FROM_0_TO_7 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_SYNC_START_BITS_FROM_0_TO_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_SYNC_START_BITS_FROM_0_TO_7<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Vertical Sync End Register"]
    pub struct CR11<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr11 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR11<'a, T>
        {
            const INDEX_R: u8 = 17;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR11<'a, T>
        {
            const INDEX_W: u8 = 17;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR11<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR11<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR11")
                    .field(
                        "protect_registers_from_0_to_7",
                        &self.protect_registers_from_0_to_7().bit(),
                    )
                    .field(
                        "select_5_refresh_cycles",
                        &self.select_5_refresh_cycles().bit(),
                    )
                    .field(
                        "vertical_interrupt_enable",
                        &self.vertical_interrupt_enable().bit(),
                    )
                    .field(
                        "vertical_interrupt_clear",
                        &self.vertical_interrupt_clear().bit(),
                    )
                    .field("vertical_sync_end", &self.vertical_sync_end().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7"]
            #[inline]
            pub fn protect_registers_from_0_to_7(&self) -> PROTECT_REGISTERS_FROM_0_TO_7_R {
                PROTECT_REGISTERS_FROM_0_TO_7_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn select_5_refresh_cycles(&self) -> SELECT_5_REFRESH_CYCLES_R {
                SELECT_5_REFRESH_CYCLES_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn vertical_interrupt_enable(&self) -> VERTICAL_INTERRUPT_ENABLE_R {
                VERTICAL_INTERRUPT_ENABLE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn vertical_interrupt_clear(&self) -> VERTICAL_INTERRUPT_CLEAR_R {
                VERTICAL_INTERRUPT_CLEAR_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn vertical_sync_end(&self) -> VERTICAL_SYNC_END_R {
                VERTICAL_SYNC_END_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `protect_registers_from_0_to_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PROTECT_REGISTERS_FROM_0_TO_7_R {
            _Reserved(bool),
        }
        impl PROTECT_REGISTERS_FROM_0_TO_7_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    PROTECT_REGISTERS_FROM_0_TO_7_R::_Reserved(true) => Self::_MASK,
                    PROTECT_REGISTERS_FROM_0_TO_7_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                PROTECT_REGISTERS_FROM_0_TO_7_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    PROTECT_REGISTERS_FROM_0_TO_7_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `select_5_refresh_cycles`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SELECT_5_REFRESH_CYCLES_R {
            _Reserved(bool),
        }
        impl SELECT_5_REFRESH_CYCLES_R {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SELECT_5_REFRESH_CYCLES_R::_Reserved(true) => Self::_MASK,
                    SELECT_5_REFRESH_CYCLES_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SELECT_5_REFRESH_CYCLES_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SELECT_5_REFRESH_CYCLES_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_interrupt_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_INTERRUPT_ENABLE_R {
            _Reserved(bool),
        }
        impl VERTICAL_INTERRUPT_ENABLE_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_INTERRUPT_ENABLE_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_INTERRUPT_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_INTERRUPT_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_INTERRUPT_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_interrupt_clear`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_INTERRUPT_CLEAR_R {
            _Reserved(bool),
        }
        impl VERTICAL_INTERRUPT_CLEAR_R {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VERTICAL_INTERRUPT_CLEAR_R::_Reserved(true) => Self::_MASK,
                    VERTICAL_INTERRUPT_CLEAR_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VERTICAL_INTERRUPT_CLEAR_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VERTICAL_INTERRUPT_CLEAR_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `vertical_sync_end`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_SYNC_END_R {
            _Reserved(u8),
        }
        impl VERTICAL_SYNC_END_R {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    VERTICAL_SYNC_END_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                VERTICAL_SYNC_END_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    VERTICAL_SYNC_END_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 7"]
            #[inline]
            pub fn protect_registers_from_0_to_7(&mut self) -> _PROTECT_REGISTERS_FROM_0_TO_7<'_> {
                _PROTECT_REGISTERS_FROM_0_TO_7 { w: self }
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn select_5_refresh_cycles(&mut self) -> _SELECT_5_REFRESH_CYCLES<'_> {
                _SELECT_5_REFRESH_CYCLES { w: self }
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn vertical_interrupt_enable(&mut self) -> _VERTICAL_INTERRUPT_ENABLE<'_> {
                _VERTICAL_INTERRUPT_ENABLE { w: self }
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn vertical_interrupt_clear(&mut self) -> _VERTICAL_INTERRUPT_CLEAR<'_> {
                _VERTICAL_INTERRUPT_CLEAR { w: self }
            }
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn vertical_sync_end(&mut self) -> _VERTICAL_SYNC_END<'_> {
                _VERTICAL_SYNC_END { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PROTECT_REGISTERS_FROM_0_TO_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _PROTECT_REGISTERS_FROM_0_TO_7<'a> {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _SELECT_5_REFRESH_CYCLES<'a> {
            w: &'a mut W,
        }
        impl<'a> _SELECT_5_REFRESH_CYCLES<'a> {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_INTERRUPT_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_INTERRUPT_ENABLE<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_INTERRUPT_CLEAR<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_INTERRUPT_CLEAR<'a> {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_SYNC_END<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_SYNC_END<'a> {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Vertical Display Enable End Register"]
    pub struct CR12<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr12 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR12<'a, T>
        {
            const INDEX_R: u8 = 18;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR12<'a, T>
        {
            const INDEX_W: u8 = 18;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR12<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR12<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR12")
                    .field(
                        "vertical_display_enable_end_bits_from_0_to_7",
                        &self.vertical_display_enable_end_bits_from_0_to_7().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_display_enable_end_bits_from_0_to_7(
                &self,
            ) -> VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R {
                VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_display_enable_end_bits_from_0_to_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R {
            _Reserved(u8),
        }
        impl VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_display_enable_end_bits_from_0_to_7(
                &mut self,
            ) -> _VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7<'_> {
                _VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_DISPLAY_ENABLE_END_BITS_FROM_0_TO_7<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Offset Register"]
    pub struct CR13<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr13 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR13<'a, T>
        {
            const INDEX_R: u8 = 19;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR13<'a, T>
        {
            const INDEX_W: u8 = 19;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR13<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR13<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR13")
                    .field("offset_bits", &self.offset_bits().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn offset_bits(&self) -> OFFSET_BITS_R {
                OFFSET_BITS_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `offset_bits`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum OFFSET_BITS_R {
            _Reserved(u8),
        }
        impl OFFSET_BITS_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    OFFSET_BITS_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                OFFSET_BITS_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    OFFSET_BITS_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn offset_bits(&mut self) -> _OFFSET_BITS<'_> {
                _OFFSET_BITS { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _OFFSET_BITS<'a> {
            w: &'a mut W,
        }
        impl<'a> _OFFSET_BITS<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Underline Location Register"]
    pub struct CR14<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr14 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR14<'a, T>
        {
            const INDEX_R: u8 = 20;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR14<'a, T>
        {
            const INDEX_W: u8 = 20;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR14<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR14<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR14")
                    .field("dword_mode", &self.dword_mode().bit())
                    .field("count_by_4", &self.count_by_4().bit())
                    .field("start_underline", &self.start_underline().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 6"]
            #[inline]
            pub fn dword_mode(&self) -> DWORD_MODE_R {
                DWORD_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn count_by_4(&self) -> COUNT_BY_4_R {
                COUNT_BY_4_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn start_underline(&self) -> START_UNDERLINE_R {
                START_UNDERLINE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `dword_mode`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DWORD_MODE_R {
            _Reserved(bool),
        }
        impl DWORD_MODE_R {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    DWORD_MODE_R::_Reserved(true) => Self::_MASK,
                    DWORD_MODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                DWORD_MODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    DWORD_MODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `count_by_4`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum COUNT_BY_4_R {
            _Reserved(bool),
        }
        impl COUNT_BY_4_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    COUNT_BY_4_R::_Reserved(true) => Self::_MASK,
                    COUNT_BY_4_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                COUNT_BY_4_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    COUNT_BY_4_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `start_underline`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum START_UNDERLINE_R {
            _Reserved(u8),
        }
        impl START_UNDERLINE_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    START_UNDERLINE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                START_UNDERLINE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    START_UNDERLINE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 6"]
            #[inline]
            pub fn dword_mode(&mut self) -> _DWORD_MODE<'_> {
                _DWORD_MODE { w: self }
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn count_by_4(&mut self) -> _COUNT_BY_4<'_> {
                _COUNT_BY_4 { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn start_underline(&mut self) -> _START_UNDERLINE<'_> {
                _START_UNDERLINE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _DWORD_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _DWORD_MODE<'a> {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _COUNT_BY_4<'a> {
            w: &'a mut W,
        }
        impl<'a> _COUNT_BY_4<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _START_UNDERLINE<'a> {
            w: &'a mut W,
        }
        impl<'a> _START_UNDERLINE<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Vertical Blanking Start Register"]
    pub struct CR15<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr15 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR15<'a, T>
        {
            const INDEX_R: u8 = 21;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR15<'a, T>
        {
            const INDEX_W: u8 = 21;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR15<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR15<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR15")
                    .field(
                        "vertical_blanking_start_bits_from_0_to_7",
                        &self.vertical_blanking_start_bits_from_0_to_7().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_blanking_start_bits_from_0_to_7(
                &self,
            ) -> VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R {
                VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_blanking_start_bits_from_0_to_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R {
            _Reserved(u8),
        }
        impl VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    VERTICAL_BLANKING_START_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn vertical_blanking_start_bits_from_0_to_7(
                &mut self,
            ) -> _VERTICAL_BLANKING_START_BITS_FROM_0_TO_7<'_> {
                _VERTICAL_BLANKING_START_BITS_FROM_0_TO_7 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_BLANKING_START_BITS_FROM_0_TO_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_BLANKING_START_BITS_FROM_0_TO_7<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Vertical Blanking End Register"]
    pub struct CR16<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr16 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR16<'a, T>
        {
            const INDEX_R: u8 = 22;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR16<'a, T>
        {
            const INDEX_W: u8 = 22;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR16<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR16<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR16")
                    .field(
                        "vertical_blanking_end",
                        &self.vertical_blanking_end().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn vertical_blanking_end(&self) -> VERTICAL_BLANKING_END_R {
                VERTICAL_BLANKING_END_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `vertical_blanking_end`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VERTICAL_BLANKING_END_R {
            _Reserved(u8),
        }
        impl VERTICAL_BLANKING_END_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    VERTICAL_BLANKING_END_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                VERTICAL_BLANKING_END_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    VERTICAL_BLANKING_END_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn vertical_blanking_end(&mut self) -> _VERTICAL_BLANKING_END<'_> {
                _VERTICAL_BLANKING_END { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VERTICAL_BLANKING_END<'a> {
            w: &'a mut W,
        }
        impl<'a> _VERTICAL_BLANKING_END<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "CRT Mode Control"]
    pub struct CR17<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr17 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR17<'a, T>
        {
            const INDEX_R: u8 = 23;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR17<'a, T>
        {
            const INDEX_W: u8 = 23;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR17<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR17<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR17")
                    .field("crt_controller_reset", &self.crt_controller_reset().bit())
                    .field(
                        "word_mode_or_byte_mode",
                        &self.word_mode_or_byte_mode().bit(),
                    )
                    .field("address_warp", &self.address_warp().bit())
                    .field("count_by_2", &self.count_by_2().bit())
                    .field(
                        "horizontal_retrace_select",
                        &self.horizontal_retrace_select().bit(),
                    )
                    .field(
                        "select_row_scan_counter",
                        &self.select_row_scan_counter().bit(),
                    )
                    .field(
                        "compatibility_mode_support",
                        &self.compatibility_mode_support().bit(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7"]
            #[inline]
            pub fn crt_controller_reset(&self) -> CRT_CONTROLLER_RESET_R {
                CRT_CONTROLLER_RESET_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn word_mode_or_byte_mode(&self) -> WORD_MODE_OR_BYTE_MODE_R {
                WORD_MODE_OR_BYTE_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn address_warp(&self) -> ADDRESS_WARP_R {
                ADDRESS_WARP_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn count_by_2(&self) -> COUNT_BY_2_R {
                COUNT_BY_2_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn horizontal_retrace_select(&self) -> HORIZONTAL_RETRACE_SELECT_R {
                HORIZONTAL_RETRACE_SELECT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn select_row_scan_counter(&self) -> SELECT_ROW_SCAN_COUNTER_R {
                SELECT_ROW_SCAN_COUNTER_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn compatibility_mode_support(&self) -> COMPATIBILITY_MODE_SUPPORT_R {
                COMPATIBILITY_MODE_SUPPORT_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `crt_controller_reset`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CRT_CONTROLLER_RESET_R {
            _Reserved(bool),
        }
        impl CRT_CONTROLLER_RESET_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    CRT_CONTROLLER_RESET_R::_Reserved(true) => Self::_MASK,
                    CRT_CONTROLLER_RESET_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                CRT_CONTROLLER_RESET_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    CRT_CONTROLLER_RESET_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `word_mode_or_byte_mode`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum WORD_MODE_OR_BYTE_MODE_R {
            _Reserved(bool),
        }
        impl WORD_MODE_OR_BYTE_MODE_R {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    WORD_MODE_OR_BYTE_MODE_R::_Reserved(true) => Self::_MASK,
                    WORD_MODE_OR_BYTE_MODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                WORD_MODE_OR_BYTE_MODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    WORD_MODE_OR_BYTE_MODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `address_warp`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ADDRESS_WARP_R {
            _Reserved(bool),
        }
        impl ADDRESS_WARP_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ADDRESS_WARP_R::_Reserved(true) => Self::_MASK,
                    ADDRESS_WARP_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ADDRESS_WARP_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ADDRESS_WARP_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `count_by_2`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum COUNT_BY_2_R {
            _Reserved(bool),
        }
        impl COUNT_BY_2_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    COUNT_BY_2_R::_Reserved(true) => Self::_MASK,
                    COUNT_BY_2_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                COUNT_BY_2_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    COUNT_BY_2_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `horizontal_retrace_select`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_RETRACE_SELECT_R {
            _Reserved(bool),
        }
        impl HORIZONTAL_RETRACE_SELECT_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    HORIZONTAL_RETRACE_SELECT_R::_Reserved(true) => Self::_MASK,
                    HORIZONTAL_RETRACE_SELECT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                HORIZONTAL_RETRACE_SELECT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    HORIZONTAL_RETRACE_SELECT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `select_row_scan_counter`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SELECT_ROW_SCAN_COUNTER_R {
            _Reserved(bool),
        }
        impl SELECT_ROW_SCAN_COUNTER_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SELECT_ROW_SCAN_COUNTER_R::_Reserved(true) => Self::_MASK,
                    SELECT_ROW_SCAN_COUNTER_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SELECT_ROW_SCAN_COUNTER_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SELECT_ROW_SCAN_COUNTER_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `compatibility_mode_support`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum COMPATIBILITY_MODE_SUPPORT_R {
            _Reserved(bool),
        }
        impl COMPATIBILITY_MODE_SUPPORT_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    COMPATIBILITY_MODE_SUPPORT_R::_Reserved(true) => Self::_MASK,
                    COMPATIBILITY_MODE_SUPPORT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                COMPATIBILITY_MODE_SUPPORT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    COMPATIBILITY_MODE_SUPPORT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 7"]
            #[inline]
            pub fn crt_controller_reset(&mut self) -> _CRT_CONTROLLER_RESET<'_> {
                _CRT_CONTROLLER_RESET { w: self }
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn word_mode_or_byte_mode(&mut self) -> _WORD_MODE_OR_BYTE_MODE<'_> {
                _WORD_MODE_OR_BYTE_MODE { w: self }
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn address_warp(&mut self) -> _ADDRESS_WARP<'_> {
                _ADDRESS_WARP { w: self }
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn count_by_2(&mut self) -> _COUNT_BY_2<'_> {
                _COUNT_BY_2 { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn horizontal_retrace_select(&mut self) -> _HORIZONTAL_RETRACE_SELECT<'_> {
                _HORIZONTAL_RETRACE_SELECT { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn select_row_scan_counter(&mut self) -> _SELECT_ROW_SCAN_COUNTER<'_> {
                _SELECT_ROW_SCAN_COUNTER { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn compatibility_mode_support(&mut self) -> _COMPATIBILITY_MODE_SUPPORT<'_> {
                _COMPATIBILITY_MODE_SUPPORT { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _CRT_CONTROLLER_RESET<'a> {
            w: &'a mut W,
        }
        impl<'a> _CRT_CONTROLLER_RESET<'a> {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _WORD_MODE_OR_BYTE_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _WORD_MODE_OR_BYTE_MODE<'a> {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ADDRESS_WARP<'a> {
            w: &'a mut W,
        }
        impl<'a> _ADDRESS_WARP<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _COUNT_BY_2<'a> {
            w: &'a mut W,
        }
        impl<'a> _COUNT_BY_2<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_RETRACE_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_RETRACE_SELECT<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _SELECT_ROW_SCAN_COUNTER<'a> {
            w: &'a mut W,
        }
        impl<'a> _SELECT_ROW_SCAN_COUNTER<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _COMPATIBILITY_MODE_SUPPORT<'a> {
            w: &'a mut W,
        }
        impl<'a> _COMPATIBILITY_MODE_SUPPORT<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Line Compare Register"]
    pub struct CR18<
        'a,
        T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod cr18 {
        use super::super::register_trait::*;
        use super::CrtControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexR for super::CR18<'a, T>
        {
            const INDEX_R: u8 = 24;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > LocationIndexW for super::CR18<'a, T>
        {
            const INDEX_W: u8 = 24;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > InGroup for super::CR18<'a, T>
        {
            type Group = CrtControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<CrtControllerGroup, u8> + RegisterIndexIoW<CrtControllerGroup, u8>,
            > super::CR18<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("CR18")
                    .field(
                        "line_compare_target_bits_from_0_to_7",
                        &self.line_compare_target_bits_from_0_to_7().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn line_compare_target_bits_from_0_to_7(
                &self,
            ) -> LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R {
                LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `line_compare_target_bits_from_0_to_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R {
            _Reserved(u8),
        }
        impl LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    LINE_COMPARE_TARGET_BITS_FROM_0_TO_7_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0 - Part 1/3 of a 10-bit value."]
            #[inline]
            pub fn line_compare_target_bits_from_0_to_7(
                &mut self,
            ) -> _LINE_COMPARE_TARGET_BITS_FROM_0_TO_7<'_> {
                _LINE_COMPARE_TARGET_BITS_FROM_0_TO_7 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _LINE_COMPARE_TARGET_BITS_FROM_0_TO_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _LINE_COMPARE_TARGET_BITS_FROM_0_TO_7<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
}
pub mod graphics_controller {
    use super::register_trait::*;
    pub struct GraphicsControllerRegisters<
        T: RegisterAbsIoR<GraphicsControllerGroup, u8>
            + RegisterAbsIoW<GraphicsControllerGroup, u8>
            + RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: T,
    }
    impl<
            T: RegisterAbsIoR<GraphicsControllerGroup, u8>
                + RegisterAbsIoW<GraphicsControllerGroup, u8>
                + RegisterIndexIoR<GraphicsControllerGroup, u8>
                + RegisterIndexIoW<GraphicsControllerGroup, u8>,
        > GraphicsControllerRegisters<T>
    {
        #[inline]
        pub fn new(io: T) -> Self {
            Self { io }
        }
        #[doc = "Graphics Controller Index Register"]
        #[inline]
        pub fn grx(&mut self) -> GRX<'_, T> {
            GRX { io: &mut self.io }
        }
        #[doc = "Set/Reset Register"]
        #[inline]
        pub fn gr00(&mut self) -> GR00<'_, T> {
            GR00 { io: &mut self.io }
        }
        #[doc = "Enable Set/Reset Register"]
        #[inline]
        pub fn gr01(&mut self) -> GR01<'_, T> {
            GR01 { io: &mut self.io }
        }
        #[doc = "Color Compare Register"]
        #[inline]
        pub fn gr02(&mut self) -> GR02<'_, T> {
            GR02 { io: &mut self.io }
        }
        #[doc = "Data Rotate Register"]
        #[inline]
        pub fn gr03(&mut self) -> GR03<'_, T> {
            GR03 { io: &mut self.io }
        }
        #[doc = "Read Plane Select Register"]
        #[inline]
        pub fn gr04(&mut self) -> GR04<'_, T> {
            GR04 { io: &mut self.io }
        }
        #[doc = "Graphics Mode Register"]
        #[inline]
        pub fn gr05(&mut self) -> GR05<'_, T> {
            GR05 { io: &mut self.io }
        }
        #[doc = "Miscellaneous Register"]
        #[inline]
        pub fn gr06(&mut self) -> GR06<'_, T> {
            GR06 { io: &mut self.io }
        }
        #[doc = "Color Don't Care Register"]
        #[inline]
        pub fn gr07(&mut self) -> GR07<'_, T> {
            GR07 { io: &mut self.io }
        }
        #[doc = "Bit Mask Register"]
        #[inline]
        pub fn gr08(&mut self) -> GR08<'_, T> {
            GR08 { io: &mut self.io }
        }
        pub fn debug_registers<F: FnMut(&dyn core::fmt::Debug)>(&mut self, mut f: F) {
            (f)(&self.grx().read());
            (f)(&self.gr00().read());
            (f)(&self.gr01().read());
            (f)(&self.gr02().read());
            (f)(&self.gr03().read());
            (f)(&self.gr04().read());
            (f)(&self.gr05().read());
            (f)(&self.gr06().read());
            (f)(&self.gr07().read());
            (f)(&self.gr08().read());
        }
    }
    pub struct GraphicsControllerGroup;
    impl RegisterGroup for GraphicsControllerGroup {}
    #[doc = "Graphics Controller Index Register"]
    pub struct GRX<
        'a,
        T: RegisterAbsIoR<GraphicsControllerGroup, u8> + RegisterAbsIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod grx {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterAbsIoR<GraphicsControllerGroup, u8>
                    + RegisterAbsIoW<GraphicsControllerGroup, u8>,
            > LocationAbsR for super::GRX<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 974;
        }
        impl<
                'a,
                T: RegisterAbsIoR<GraphicsControllerGroup, u8>
                    + RegisterAbsIoW<GraphicsControllerGroup, u8>,
            > LocationAbsW for super::GRX<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 974;
        }
        impl<
                'a,
                T: RegisterAbsIoR<GraphicsControllerGroup, u8>
                    + RegisterAbsIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GRX<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterAbsIoR<GraphicsControllerGroup, u8>
                    + RegisterAbsIoW<GraphicsControllerGroup, u8>,
            > super::GRX<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GRX")
                    .field(
                        "graphics_controller_register_index",
                        &self.graphics_controller_register_index().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 3:0 - Graphics controller register index."]
            #[inline]
            pub fn graphics_controller_register_index(
                &self,
            ) -> GRAPHICS_CONTROLLER_REGISTER_INDEX_R {
                GRAPHICS_CONTROLLER_REGISTER_INDEX_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `graphics_controller_register_index`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum GRAPHICS_CONTROLLER_REGISTER_INDEX_R {
            _Reserved(u8),
        }
        impl GRAPHICS_CONTROLLER_REGISTER_INDEX_R {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    GRAPHICS_CONTROLLER_REGISTER_INDEX_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                GRAPHICS_CONTROLLER_REGISTER_INDEX_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    GRAPHICS_CONTROLLER_REGISTER_INDEX_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 3:0 - Graphics controller register index."]
            #[inline]
            pub fn graphics_controller_register_index(
                &mut self,
            ) -> _GRAPHICS_CONTROLLER_REGISTER_INDEX<'_> {
                _GRAPHICS_CONTROLLER_REGISTER_INDEX { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _GRAPHICS_CONTROLLER_REGISTER_INDEX<'a> {
            w: &'a mut W,
        }
        impl<'a> _GRAPHICS_CONTROLLER_REGISTER_INDEX<'a> {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Set/Reset Register"]
    pub struct GR00<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr00 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR00<'a, T>
        {
            const INDEX_R: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR00<'a, T>
        {
            const INDEX_W: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR00<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR00<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR00")
                    .field(
                        "set_slash_reset_plane",
                        &self.set_slash_reset_plane().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn set_slash_reset_plane(&self) -> SET_SLASH_RESET_PLANE_R {
                SET_SLASH_RESET_PLANE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `set_slash_reset_plane`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SET_SLASH_RESET_PLANE_R {
            _Reserved(u8),
        }
        impl SET_SLASH_RESET_PLANE_R {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    SET_SLASH_RESET_PLANE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                SET_SLASH_RESET_PLANE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    SET_SLASH_RESET_PLANE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn set_slash_reset_plane(&mut self) -> _SET_SLASH_RESET_PLANE<'_> {
                _SET_SLASH_RESET_PLANE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _SET_SLASH_RESET_PLANE<'a> {
            w: &'a mut W,
        }
        impl<'a> _SET_SLASH_RESET_PLANE<'a> {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Enable Set/Reset Register"]
    pub struct GR01<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr01 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR01<'a, T>
        {
            const INDEX_R: u8 = 1;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR01<'a, T>
        {
            const INDEX_W: u8 = 1;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR01<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR01<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR01")
                    .field(
                        "enable_set_slash_reset_plane",
                        &self.enable_set_slash_reset_plane().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn enable_set_slash_reset_plane(&self) -> ENABLE_SET_SLASH_RESET_PLANE_R {
                ENABLE_SET_SLASH_RESET_PLANE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `enable_set_slash_reset_plane`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_SET_SLASH_RESET_PLANE_R {
            _Reserved(u8),
        }
        impl ENABLE_SET_SLASH_RESET_PLANE_R {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    ENABLE_SET_SLASH_RESET_PLANE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                ENABLE_SET_SLASH_RESET_PLANE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    ENABLE_SET_SLASH_RESET_PLANE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn enable_set_slash_reset_plane(&mut self) -> _ENABLE_SET_SLASH_RESET_PLANE<'_> {
                _ENABLE_SET_SLASH_RESET_PLANE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_SET_SLASH_RESET_PLANE<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_SET_SLASH_RESET_PLANE<'a> {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Color Compare Register"]
    pub struct GR02<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr02 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR02<'a, T>
        {
            const INDEX_R: u8 = 2;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR02<'a, T>
        {
            const INDEX_W: u8 = 2;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR02<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR02<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR02")
                    .field("color_compare_plane", &self.color_compare_plane().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn color_compare_plane(&self) -> COLOR_COMPARE_PLANE_R {
                COLOR_COMPARE_PLANE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `color_compare_plane`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum COLOR_COMPARE_PLANE_R {
            _Reserved(u8),
        }
        impl COLOR_COMPARE_PLANE_R {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    COLOR_COMPARE_PLANE_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                COLOR_COMPARE_PLANE_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    COLOR_COMPARE_PLANE_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn color_compare_plane(&mut self) -> _COLOR_COMPARE_PLANE<'_> {
                _COLOR_COMPARE_PLANE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _COLOR_COMPARE_PLANE<'a> {
            w: &'a mut W,
        }
        impl<'a> _COLOR_COMPARE_PLANE<'a> {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Data Rotate Register"]
    pub struct GR03<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr03 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR03<'a, T>
        {
            const INDEX_R: u8 = 3;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR03<'a, T>
        {
            const INDEX_W: u8 = 3;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR03<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR03<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR03")
                    .field(
                        "function_select",
                        &format_args!("{:?}", self.function_select()),
                    )
                    .field("rotate_count", &self.rotate_count().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 4:3"]
            #[inline]
            pub fn function_select(&self) -> FUNCTION_SELECT_R {
                FUNCTION_SELECT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 2:0"]
            #[inline]
            pub fn rotate_count(&self) -> ROTATE_COUNT_R {
                ROTATE_COUNT_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `function_select`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum FUNCTION_SELECT_R {
            ZERO = 0,
            ONE = 1,
            TWO = 2,
            THREE = 3,
        }
        impl FUNCTION_SELECT_R {
            #[doc = "Checks if the value of the field is `ZERO`"]
            #[inline]
            pub fn is_zero(&self) -> bool {
                *self == FUNCTION_SELECT_R::ZERO
            }
            #[doc = "Checks if the value of the field is `ONE`"]
            #[inline]
            pub fn is_one(&self) -> bool {
                *self == FUNCTION_SELECT_R::ONE
            }
            #[doc = "Checks if the value of the field is `TWO`"]
            #[inline]
            pub fn is_two(&self) -> bool {
                *self == FUNCTION_SELECT_R::TWO
            }
            #[doc = "Checks if the value of the field is `THREE`"]
            #[inline]
            pub fn is_three(&self) -> bool {
                *self == FUNCTION_SELECT_R::THREE
            }
            const _MASK: u8 = 24;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => FUNCTION_SELECT_R::ZERO,
                    1 => FUNCTION_SELECT_R::ONE,
                    2 => FUNCTION_SELECT_R::TWO,
                    3 => FUNCTION_SELECT_R::THREE,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Possible values of the field `rotate_count`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ROTATE_COUNT_R {
            _Reserved(u8),
        }
        impl ROTATE_COUNT_R {
            const _MASK: u8 = 7;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    ROTATE_COUNT_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                ROTATE_COUNT_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    ROTATE_COUNT_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 4:3"]
            #[inline]
            pub fn function_select(&mut self) -> _FUNCTION_SELECT<'_> {
                _FUNCTION_SELECT { w: self }
            }
            #[doc = "Bits 2:0"]
            #[inline]
            pub fn rotate_count(&mut self) -> _ROTATE_COUNT<'_> {
                _ROTATE_COUNT { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `function_select`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum FUNCTION_SELECT_W {
            ZERO = 0,
            ONE = 1,
            TWO = 2,
            THREE = 3,
        }
        impl FUNCTION_SELECT_W {
            const _MASK: u8 = 24;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _FUNCTION_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _FUNCTION_SELECT<'a> {
            const _MASK: u8 = 24;
            const _OFFSET: u8 = 3;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: FUNCTION_SELECT_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn zero(self) -> &'a mut W {
                self.variant(FUNCTION_SELECT_W::ZERO)
            }
            #[inline]
            pub fn one(self) -> &'a mut W {
                self.variant(FUNCTION_SELECT_W::ONE)
            }
            #[inline]
            pub fn two(self) -> &'a mut W {
                self.variant(FUNCTION_SELECT_W::TWO)
            }
            #[inline]
            pub fn three(self) -> &'a mut W {
                self.variant(FUNCTION_SELECT_W::THREE)
            }
        }
        #[doc = "Proxy"]
        pub struct _ROTATE_COUNT<'a> {
            w: &'a mut W,
        }
        impl<'a> _ROTATE_COUNT<'a> {
            const _MASK: u8 = 7;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Read Plane Select Register"]
    pub struct GR04<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr04 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR04<'a, T>
        {
            const INDEX_R: u8 = 4;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR04<'a, T>
        {
            const INDEX_W: u8 = 4;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR04<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR04<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR04")
                    .field(
                        "read_plane_select",
                        &format_args!("{:?}", self.read_plane_select()),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn read_plane_select(&self) -> READ_PLANE_SELECT_R {
                READ_PLANE_SELECT_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `read_plane_select`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum READ_PLANE_SELECT_R {
            PLANE_0 = 0,
            PLANE_1 = 1,
            PLANE_2 = 2,
            PLANE_3 = 3,
        }
        impl READ_PLANE_SELECT_R {
            #[doc = "Checks if the value of the field is `PLANE_0`"]
            #[inline]
            pub fn is_plane_0(&self) -> bool {
                *self == READ_PLANE_SELECT_R::PLANE_0
            }
            #[doc = "Checks if the value of the field is `PLANE_1`"]
            #[inline]
            pub fn is_plane_1(&self) -> bool {
                *self == READ_PLANE_SELECT_R::PLANE_1
            }
            #[doc = "Checks if the value of the field is `PLANE_2`"]
            #[inline]
            pub fn is_plane_2(&self) -> bool {
                *self == READ_PLANE_SELECT_R::PLANE_2
            }
            #[doc = "Checks if the value of the field is `PLANE_3`"]
            #[inline]
            pub fn is_plane_3(&self) -> bool {
                *self == READ_PLANE_SELECT_R::PLANE_3
            }
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => READ_PLANE_SELECT_R::PLANE_0,
                    1 => READ_PLANE_SELECT_R::PLANE_1,
                    2 => READ_PLANE_SELECT_R::PLANE_2,
                    3 => READ_PLANE_SELECT_R::PLANE_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn read_plane_select(&mut self) -> _READ_PLANE_SELECT<'_> {
                _READ_PLANE_SELECT { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `read_plane_select`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum READ_PLANE_SELECT_W {
            PLANE_0 = 0,
            PLANE_1 = 1,
            PLANE_2 = 2,
            PLANE_3 = 3,
        }
        impl READ_PLANE_SELECT_W {
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _READ_PLANE_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _READ_PLANE_SELECT<'a> {
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: READ_PLANE_SELECT_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn plane_0(self) -> &'a mut W {
                self.variant(READ_PLANE_SELECT_W::PLANE_0)
            }
            #[inline]
            pub fn plane_1(self) -> &'a mut W {
                self.variant(READ_PLANE_SELECT_W::PLANE_1)
            }
            #[inline]
            pub fn plane_2(self) -> &'a mut W {
                self.variant(READ_PLANE_SELECT_W::PLANE_2)
            }
            #[inline]
            pub fn plane_3(self) -> &'a mut W {
                self.variant(READ_PLANE_SELECT_W::PLANE_3)
            }
        }
    }
    #[doc = "Graphics Mode Register"]
    pub struct GR05<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr05 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR05<'a, T>
        {
            const INDEX_R: u8 = 5;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR05<'a, T>
        {
            const INDEX_W: u8 = 5;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR05<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR05<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR05")
                    .field(
                        "shift_register_control",
                        &self.shift_register_control().bits(),
                    )
                    .field("odd_slash_even_mode", &self.odd_slash_even_mode().bit())
                    .field("read_mode", &self.read_mode().bit())
                    .field("write_mode", &format_args!("{:?}", self.write_mode()))
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn shift_register_control(&self) -> SHIFT_REGISTER_CONTROL_R {
                SHIFT_REGISTER_CONTROL_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn odd_slash_even_mode(&self) -> ODD_SLASH_EVEN_MODE_R {
                ODD_SLASH_EVEN_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn read_mode(&self) -> READ_MODE_R {
                READ_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn write_mode(&self) -> WRITE_MODE_R {
                WRITE_MODE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `shift_register_control`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SHIFT_REGISTER_CONTROL_R {
            _Reserved(u8),
        }
        impl SHIFT_REGISTER_CONTROL_R {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    SHIFT_REGISTER_CONTROL_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                SHIFT_REGISTER_CONTROL_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    SHIFT_REGISTER_CONTROL_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Possible values of the field `odd_slash_even_mode`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ODD_SLASH_EVEN_MODE_R {
            _Reserved(bool),
        }
        impl ODD_SLASH_EVEN_MODE_R {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ODD_SLASH_EVEN_MODE_R::_Reserved(true) => Self::_MASK,
                    ODD_SLASH_EVEN_MODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ODD_SLASH_EVEN_MODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ODD_SLASH_EVEN_MODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `read_mode`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum READ_MODE_R {
            _Reserved(bool),
        }
        impl READ_MODE_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    READ_MODE_R::_Reserved(true) => Self::_MASK,
                    READ_MODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                READ_MODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    READ_MODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `write_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum WRITE_MODE_R {
            MODE_0 = 0,
            MODE_1 = 1,
            MODE_2 = 2,
            MODE_3 = 3,
        }
        impl WRITE_MODE_R {
            #[doc = "Checks if the value of the field is `MODE_0`"]
            #[inline]
            pub fn is_mode_0(&self) -> bool {
                *self == WRITE_MODE_R::MODE_0
            }
            #[doc = "Checks if the value of the field is `MODE_1`"]
            #[inline]
            pub fn is_mode_1(&self) -> bool {
                *self == WRITE_MODE_R::MODE_1
            }
            #[doc = "Checks if the value of the field is `MODE_2`"]
            #[inline]
            pub fn is_mode_2(&self) -> bool {
                *self == WRITE_MODE_R::MODE_2
            }
            #[doc = "Checks if the value of the field is `MODE_3`"]
            #[inline]
            pub fn is_mode_3(&self) -> bool {
                *self == WRITE_MODE_R::MODE_3
            }
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => WRITE_MODE_R::MODE_0,
                    1 => WRITE_MODE_R::MODE_1,
                    2 => WRITE_MODE_R::MODE_2,
                    3 => WRITE_MODE_R::MODE_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 6:5"]
            #[inline]
            pub fn shift_register_control(&mut self) -> _SHIFT_REGISTER_CONTROL<'_> {
                _SHIFT_REGISTER_CONTROL { w: self }
            }
            #[doc = "Bit 4"]
            #[inline]
            pub fn odd_slash_even_mode(&mut self) -> _ODD_SLASH_EVEN_MODE<'_> {
                _ODD_SLASH_EVEN_MODE { w: self }
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn read_mode(&mut self) -> _READ_MODE<'_> {
                _READ_MODE { w: self }
            }
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn write_mode(&mut self) -> _WRITE_MODE<'_> {
                _WRITE_MODE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _SHIFT_REGISTER_CONTROL<'a> {
            w: &'a mut W,
        }
        impl<'a> _SHIFT_REGISTER_CONTROL<'a> {
            const _MASK: u8 = 96;
            const _OFFSET: u8 = 5;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ODD_SLASH_EVEN_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _ODD_SLASH_EVEN_MODE<'a> {
            const _MASK: u8 = 16;
            const _OFFSET: u8 = 4;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _READ_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _READ_MODE<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `write_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum WRITE_MODE_W {
            MODE_0 = 0,
            MODE_1 = 1,
            MODE_2 = 2,
            MODE_3 = 3,
        }
        impl WRITE_MODE_W {
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _WRITE_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _WRITE_MODE<'a> {
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: WRITE_MODE_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn mode_0(self) -> &'a mut W {
                self.variant(WRITE_MODE_W::MODE_0)
            }
            #[inline]
            pub fn mode_1(self) -> &'a mut W {
                self.variant(WRITE_MODE_W::MODE_1)
            }
            #[inline]
            pub fn mode_2(self) -> &'a mut W {
                self.variant(WRITE_MODE_W::MODE_2)
            }
            #[inline]
            pub fn mode_3(self) -> &'a mut W {
                self.variant(WRITE_MODE_W::MODE_3)
            }
        }
    }
    #[doc = "Miscellaneous Register"]
    pub struct GR06<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr06 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR06<'a, T>
        {
            const INDEX_R: u8 = 6;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR06<'a, T>
        {
            const INDEX_W: u8 = 6;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR06<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR06<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR06")
                    .field(
                        "memory_map_mode",
                        &format_args!("{:?}", self.memory_map_mode()),
                    )
                    .field("chain_odd_slash_even", &self.chain_odd_slash_even().bit())
                    .field(
                        "graphics_slash_text_mode",
                        &format_args!("{:?}", self.graphics_slash_text_mode()),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 3:2"]
            #[inline]
            pub fn memory_map_mode(&self) -> MEMORY_MAP_MODE_R {
                MEMORY_MAP_MODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn chain_odd_slash_even(&self) -> CHAIN_ODD_SLASH_EVEN_R {
                CHAIN_ODD_SLASH_EVEN_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn graphics_slash_text_mode(&self) -> GRAPHICS_SLASH_TEXT_MODE_R {
                GRAPHICS_SLASH_TEXT_MODE_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `memory_map_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MEMORY_MAP_MODE_R {
            #[doc = "0xA0000 - 0xBFFFF"]
            MODE_0 = 0,
            #[doc = "0xA0000 - 0xAFFFF"]
            MODE_1 = 1,
            #[doc = "0xB0000 - 0xB7FFF"]
            MODE_2 = 2,
            #[doc = "0xB8000 - 0xBFFFF"]
            MODE_3 = 3,
        }
        impl MEMORY_MAP_MODE_R {
            #[doc = "Checks if the value of the field is `MODE_0`"]
            #[inline]
            pub fn is_mode_0(&self) -> bool {
                *self == MEMORY_MAP_MODE_R::MODE_0
            }
            #[doc = "Checks if the value of the field is `MODE_1`"]
            #[inline]
            pub fn is_mode_1(&self) -> bool {
                *self == MEMORY_MAP_MODE_R::MODE_1
            }
            #[doc = "Checks if the value of the field is `MODE_2`"]
            #[inline]
            pub fn is_mode_2(&self) -> bool {
                *self == MEMORY_MAP_MODE_R::MODE_2
            }
            #[doc = "Checks if the value of the field is `MODE_3`"]
            #[inline]
            pub fn is_mode_3(&self) -> bool {
                *self == MEMORY_MAP_MODE_R::MODE_3
            }
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => MEMORY_MAP_MODE_R::MODE_0,
                    1 => MEMORY_MAP_MODE_R::MODE_1,
                    2 => MEMORY_MAP_MODE_R::MODE_2,
                    3 => MEMORY_MAP_MODE_R::MODE_3,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
        #[doc = "Possible values of the field `chain_odd_slash_even`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum CHAIN_ODD_SLASH_EVEN_R {
            _Reserved(bool),
        }
        impl CHAIN_ODD_SLASH_EVEN_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    CHAIN_ODD_SLASH_EVEN_R::_Reserved(true) => Self::_MASK,
                    CHAIN_ODD_SLASH_EVEN_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                CHAIN_ODD_SLASH_EVEN_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    CHAIN_ODD_SLASH_EVEN_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `graphics_slash_text_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum GRAPHICS_SLASH_TEXT_MODE_R {
            TEXT = 0,
            GRAPHICS = 1,
        }
        impl GRAPHICS_SLASH_TEXT_MODE_R {
            #[doc = "Checks if the value of the field is `TEXT`"]
            #[inline]
            pub fn is_text(&self) -> bool {
                *self == GRAPHICS_SLASH_TEXT_MODE_R::TEXT
            }
            #[doc = "Checks if the value of the field is `GRAPHICS`"]
            #[inline]
            pub fn is_graphics(&self) -> bool {
                *self == GRAPHICS_SLASH_TEXT_MODE_R::GRAPHICS
            }
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => GRAPHICS_SLASH_TEXT_MODE_R::TEXT,
                    1 => GRAPHICS_SLASH_TEXT_MODE_R::GRAPHICS,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits() == 1
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 3:2"]
            #[inline]
            pub fn memory_map_mode(&mut self) -> _MEMORY_MAP_MODE<'_> {
                _MEMORY_MAP_MODE { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn chain_odd_slash_even(&mut self) -> _CHAIN_ODD_SLASH_EVEN<'_> {
                _CHAIN_ODD_SLASH_EVEN { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn graphics_slash_text_mode(&mut self) -> _GRAPHICS_SLASH_TEXT_MODE<'_> {
                _GRAPHICS_SLASH_TEXT_MODE { w: self }
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `memory_map_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum MEMORY_MAP_MODE_W {
            #[doc = "0xA0000 - 0xBFFFF"]
            MODE_0 = 0,
            #[doc = "0xA0000 - 0xAFFFF"]
            MODE_1 = 1,
            #[doc = "0xB0000 - 0xB7FFF"]
            MODE_2 = 2,
            #[doc = "0xB8000 - 0xBFFFF"]
            MODE_3 = 3,
        }
        impl MEMORY_MAP_MODE_W {
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _MEMORY_MAP_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _MEMORY_MAP_MODE<'a> {
            const _MASK: u8 = 12;
            const _OFFSET: u8 = 2;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: MEMORY_MAP_MODE_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[doc = "0xA0000 - 0xBFFFF"]
            #[inline]
            pub fn mode_0(self) -> &'a mut W {
                self.variant(MEMORY_MAP_MODE_W::MODE_0)
            }
            #[doc = "0xA0000 - 0xAFFFF"]
            #[inline]
            pub fn mode_1(self) -> &'a mut W {
                self.variant(MEMORY_MAP_MODE_W::MODE_1)
            }
            #[doc = "0xB0000 - 0xB7FFF"]
            #[inline]
            pub fn mode_2(self) -> &'a mut W {
                self.variant(MEMORY_MAP_MODE_W::MODE_2)
            }
            #[doc = "0xB8000 - 0xBFFFF"]
            #[inline]
            pub fn mode_3(self) -> &'a mut W {
                self.variant(MEMORY_MAP_MODE_W::MODE_3)
            }
        }
        #[doc = "Proxy"]
        pub struct _CHAIN_ODD_SLASH_EVEN<'a> {
            w: &'a mut W,
        }
        impl<'a> _CHAIN_ODD_SLASH_EVEN<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `graphics_slash_text_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum GRAPHICS_SLASH_TEXT_MODE_W {
            TEXT = 0,
            GRAPHICS = 1,
        }
        impl GRAPHICS_SLASH_TEXT_MODE_W {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _GRAPHICS_SLASH_TEXT_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _GRAPHICS_SLASH_TEXT_MODE<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: GRAPHICS_SLASH_TEXT_MODE_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn text(self) -> &'a mut W {
                self.variant(GRAPHICS_SLASH_TEXT_MODE_W::TEXT)
            }
            #[inline]
            pub fn graphics(self) -> &'a mut W {
                self.variant(GRAPHICS_SLASH_TEXT_MODE_W::GRAPHICS)
            }
        }
    }
    #[doc = "Color Don't Care Register"]
    pub struct GR07<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr07 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR07<'a, T>
        {
            const INDEX_R: u8 = 7;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR07<'a, T>
        {
            const INDEX_W: u8 = 7;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR07<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR07<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR07")
                    .field("ignore_color_plane_3", &self.ignore_color_plane_3().bit())
                    .field("ignore_color_plane_2", &self.ignore_color_plane_2().bit())
                    .field("ignore_color_plane_1", &self.ignore_color_plane_1().bit())
                    .field("ignore_color_plane_0", &self.ignore_color_plane_0().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 3"]
            #[inline]
            pub fn ignore_color_plane_3(&self) -> IGNORE_COLOR_PLANE_3_R {
                IGNORE_COLOR_PLANE_3_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn ignore_color_plane_2(&self) -> IGNORE_COLOR_PLANE_2_R {
                IGNORE_COLOR_PLANE_2_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn ignore_color_plane_1(&self) -> IGNORE_COLOR_PLANE_1_R {
                IGNORE_COLOR_PLANE_1_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn ignore_color_plane_0(&self) -> IGNORE_COLOR_PLANE_0_R {
                IGNORE_COLOR_PLANE_0_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `ignore_color_plane_3`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum IGNORE_COLOR_PLANE_3_R {
            _Reserved(bool),
        }
        impl IGNORE_COLOR_PLANE_3_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    IGNORE_COLOR_PLANE_3_R::_Reserved(true) => Self::_MASK,
                    IGNORE_COLOR_PLANE_3_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                IGNORE_COLOR_PLANE_3_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    IGNORE_COLOR_PLANE_3_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `ignore_color_plane_2`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum IGNORE_COLOR_PLANE_2_R {
            _Reserved(bool),
        }
        impl IGNORE_COLOR_PLANE_2_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    IGNORE_COLOR_PLANE_2_R::_Reserved(true) => Self::_MASK,
                    IGNORE_COLOR_PLANE_2_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                IGNORE_COLOR_PLANE_2_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    IGNORE_COLOR_PLANE_2_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `ignore_color_plane_1`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum IGNORE_COLOR_PLANE_1_R {
            _Reserved(bool),
        }
        impl IGNORE_COLOR_PLANE_1_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    IGNORE_COLOR_PLANE_1_R::_Reserved(true) => Self::_MASK,
                    IGNORE_COLOR_PLANE_1_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                IGNORE_COLOR_PLANE_1_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    IGNORE_COLOR_PLANE_1_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `ignore_color_plane_0`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum IGNORE_COLOR_PLANE_0_R {
            _Reserved(bool),
        }
        impl IGNORE_COLOR_PLANE_0_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    IGNORE_COLOR_PLANE_0_R::_Reserved(true) => Self::_MASK,
                    IGNORE_COLOR_PLANE_0_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                IGNORE_COLOR_PLANE_0_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    IGNORE_COLOR_PLANE_0_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 3"]
            #[inline]
            pub fn ignore_color_plane_3(&mut self) -> _IGNORE_COLOR_PLANE_3<'_> {
                _IGNORE_COLOR_PLANE_3 { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn ignore_color_plane_2(&mut self) -> _IGNORE_COLOR_PLANE_2<'_> {
                _IGNORE_COLOR_PLANE_2 { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn ignore_color_plane_1(&mut self) -> _IGNORE_COLOR_PLANE_1<'_> {
                _IGNORE_COLOR_PLANE_1 { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn ignore_color_plane_0(&mut self) -> _IGNORE_COLOR_PLANE_0<'_> {
                _IGNORE_COLOR_PLANE_0 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _IGNORE_COLOR_PLANE_3<'a> {
            w: &'a mut W,
        }
        impl<'a> _IGNORE_COLOR_PLANE_3<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _IGNORE_COLOR_PLANE_2<'a> {
            w: &'a mut W,
        }
        impl<'a> _IGNORE_COLOR_PLANE_2<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _IGNORE_COLOR_PLANE_1<'a> {
            w: &'a mut W,
        }
        impl<'a> _IGNORE_COLOR_PLANE_1<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _IGNORE_COLOR_PLANE_0<'a> {
            w: &'a mut W,
        }
        impl<'a> _IGNORE_COLOR_PLANE_0<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Bit Mask Register"]
    pub struct GR08<
        'a,
        T: RegisterIndexIoR<GraphicsControllerGroup, u8>
            + RegisterIndexIoW<GraphicsControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod gr08 {
        use super::super::register_trait::*;
        use super::GraphicsControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexR for super::GR08<'a, T>
        {
            const INDEX_R: u8 = 8;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > LocationIndexW for super::GR08<'a, T>
        {
            const INDEX_W: u8 = 8;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > InGroup for super::GR08<'a, T>
        {
            type Group = GraphicsControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<GraphicsControllerGroup, u8>
                    + RegisterIndexIoW<GraphicsControllerGroup, u8>,
            > super::GR08<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("GR08")
                    .field("bit_mask", &self.bit_mask().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn bit_mask(&self) -> BIT_MASK_R {
                BIT_MASK_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `bit_mask`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum BIT_MASK_R {
            _Reserved(u8),
        }
        impl BIT_MASK_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    BIT_MASK_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                BIT_MASK_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    BIT_MASK_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn bit_mask(&mut self) -> _BIT_MASK<'_> {
                _BIT_MASK { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _BIT_MASK<'a> {
            w: &'a mut W,
        }
        impl<'a> _BIT_MASK<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
}
pub mod attribute_controller {
    use super::register_trait::*;
    pub struct AttributeControllerRegisters<
        T: RegisterAbsIoR<AttributeControllerGroup, u8>
            + RegisterAbsIoW<AttributeControllerGroup, u8>
            + RegisterIndexIoR<AttributeControllerGroup, u8>
            + RegisterIndexIoW<AttributeControllerGroup, u8>,
    > {
        io: T,
    }
    impl<
            T: RegisterAbsIoR<AttributeControllerGroup, u8>
                + RegisterAbsIoW<AttributeControllerGroup, u8>
                + RegisterIndexIoR<AttributeControllerGroup, u8>
                + RegisterIndexIoW<AttributeControllerGroup, u8>,
        > AttributeControllerRegisters<T>
    {
        #[inline]
        pub fn new(io: T) -> Self {
            Self { io }
        }
        #[doc = "Attribute Controller Index Register"]
        #[inline]
        pub fn arx(&mut self) -> ARX<'_, T> {
            ARX { io: &mut self.io }
        }
        #[doc = "Palette Register 0\n\nTotal number of palette registers is 16.\nThese registers have indexes from 0x0 to 0xF.\n"]
        #[inline]
        pub fn ar00(&mut self) -> AR00<'_, T> {
            AR00 { io: &mut self.io }
        }
        #[doc = "Mode Control Register"]
        #[inline]
        pub fn ar10(&mut self) -> AR10<'_, T> {
            AR10 { io: &mut self.io }
        }
        #[doc = "Memory Plane Enable Register"]
        #[inline]
        pub fn ar12(&mut self) -> AR12<'_, T> {
            AR12 { io: &mut self.io }
        }
        #[doc = "Horizontal Pixel Panning Register"]
        #[inline]
        pub fn ar13(&mut self) -> AR13<'_, T> {
            AR13 { io: &mut self.io }
        }
        #[doc = "Color Select Register"]
        #[inline]
        pub fn ar14(&mut self) -> AR14<'_, T> {
            AR14 { io: &mut self.io }
        }
        pub fn debug_registers<F: FnMut(&dyn core::fmt::Debug)>(&mut self, mut f: F) {
            (f)(&self.arx().read());
            (f)(&self.ar00().read());
            (f)(&self.ar10().read());
            (f)(&self.ar12().read());
            (f)(&self.ar13().read());
            (f)(&self.ar14().read());
        }
    }
    pub struct AttributeControllerGroup;
    impl RegisterGroup for AttributeControllerGroup {}
    #[doc = "Attribute Controller Index Register"]
    pub struct ARX<
        'a,
        T: RegisterAbsIoR<AttributeControllerGroup, u8> + RegisterAbsIoW<AttributeControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod arx {
        use super::super::register_trait::*;
        use super::AttributeControllerGroup;
        impl<
                'a,
                T: RegisterAbsIoR<AttributeControllerGroup, u8>
                    + RegisterAbsIoW<AttributeControllerGroup, u8>,
            > LocationAbsR for super::ARX<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 960;
        }
        impl<
                'a,
                T: RegisterAbsIoR<AttributeControllerGroup, u8>
                    + RegisterAbsIoW<AttributeControllerGroup, u8>,
            > LocationAbsW for super::ARX<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 960;
        }
        impl<
                'a,
                T: RegisterAbsIoR<AttributeControllerGroup, u8>
                    + RegisterAbsIoW<AttributeControllerGroup, u8>,
            > InGroup for super::ARX<'a, T>
        {
            type Group = AttributeControllerGroup;
        }
        impl<
                'a,
                T: RegisterAbsIoR<AttributeControllerGroup, u8>
                    + RegisterAbsIoW<AttributeControllerGroup, u8>,
            > super::ARX<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("ARX")
                    .field("video_enable", &self.video_enable().bit())
                    .field(
                        "attribute_controller_register_index",
                        &self.attribute_controller_register_index().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 5"]
            #[inline]
            pub fn video_enable(&self) -> VIDEO_ENABLE_R {
                VIDEO_ENABLE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn attribute_controller_register_index(
                &self,
            ) -> ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R {
                ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `video_enable`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum VIDEO_ENABLE_R {
            _Reserved(bool),
        }
        impl VIDEO_ENABLE_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    VIDEO_ENABLE_R::_Reserved(true) => Self::_MASK,
                    VIDEO_ENABLE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                VIDEO_ENABLE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    VIDEO_ENABLE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `attribute_controller_register_index`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R {
            _Reserved(u8),
        }
        impl ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    ATTRIBUTE_CONTROLLER_REGISTER_INDEX_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 5"]
            #[inline]
            pub fn video_enable(&mut self) -> _VIDEO_ENABLE<'_> {
                _VIDEO_ENABLE { w: self }
            }
            #[doc = "Bits 4:0"]
            #[inline]
            pub fn attribute_controller_register_index(
                &mut self,
            ) -> _ATTRIBUTE_CONTROLLER_REGISTER_INDEX<'_> {
                _ATTRIBUTE_CONTROLLER_REGISTER_INDEX { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _VIDEO_ENABLE<'a> {
            w: &'a mut W,
        }
        impl<'a> _VIDEO_ENABLE<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ATTRIBUTE_CONTROLLER_REGISTER_INDEX<'a> {
            w: &'a mut W,
        }
        impl<'a> _ATTRIBUTE_CONTROLLER_REGISTER_INDEX<'a> {
            const _MASK: u8 = 31;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Palette Register 0\n\nTotal number of palette registers is 16.\nThese registers have indexes from 0x0 to 0xF.\n"]
    pub struct AR00<
        'a,
        T: RegisterIndexIoR<AttributeControllerGroup, u8>
            + RegisterIndexIoW<AttributeControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod ar00 {
        use super::super::register_trait::*;
        use super::AttributeControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexR for super::AR00<'a, T>
        {
            const INDEX_R: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexW for super::AR00<'a, T>
        {
            const INDEX_W: u8 = 0;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > InGroup for super::AR00<'a, T>
        {
            type Group = AttributeControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > super::AR00<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("AR00")
                    .field("palette_bits", &self.palette_bits().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 5:0"]
            #[inline]
            pub fn palette_bits(&self) -> PALETTE_BITS_R {
                PALETTE_BITS_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `palette_bits`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PALETTE_BITS_R {
            _Reserved(u8),
        }
        impl PALETTE_BITS_R {
            const _MASK: u8 = 63;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    PALETTE_BITS_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                PALETTE_BITS_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    PALETTE_BITS_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 5:0"]
            #[inline]
            pub fn palette_bits(&mut self) -> _PALETTE_BITS<'_> {
                _PALETTE_BITS { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_BITS<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_BITS<'a> {
            const _MASK: u8 = 63;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Mode Control Register"]
    pub struct AR10<
        'a,
        T: RegisterIndexIoR<AttributeControllerGroup, u8>
            + RegisterIndexIoW<AttributeControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod ar10 {
        use super::super::register_trait::*;
        use super::AttributeControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexR for super::AR10<'a, T>
        {
            const INDEX_R: u8 = 16;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexW for super::AR10<'a, T>
        {
            const INDEX_W: u8 = 16;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > InGroup for super::AR10<'a, T>
        {
            type Group = AttributeControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > super::AR10<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("AR10")
                    .field(
                        "palette_bits_p5_p4_select",
                        &self.palette_bits_p5_p4_select().bit(),
                    )
                    .field(
                        "pixel_width_slash_clock_select",
                        &self.pixel_width_slash_clock_select().bit(),
                    )
                    .field(
                        "pixel_panning_compatibility",
                        &self.pixel_panning_compatibility().bit(),
                    )
                    .field(
                        "enable_blinking_slash_select_background_intensity",
                        &self
                            .enable_blinking_slash_select_background_intensity()
                            .bit(),
                    )
                    .field(
                        "enable_line_graphics_character_code",
                        &self.enable_line_graphics_character_code().bit(),
                    )
                    .field("select_display_type", &self.select_display_type().bit())
                    .field(
                        "graphics_slash_alphanumeric_mode",
                        &format_args!("{:?}", self.graphics_slash_alphanumeric_mode()),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 7"]
            #[inline]
            pub fn palette_bits_p5_p4_select(&self) -> PALETTE_BITS_P5_P4_SELECT_R {
                PALETTE_BITS_P5_P4_SELECT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn pixel_width_slash_clock_select(&self) -> PIXEL_WIDTH_SLASH_CLOCK_SELECT_R {
                PIXEL_WIDTH_SLASH_CLOCK_SELECT_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn pixel_panning_compatibility(&self) -> PIXEL_PANNING_COMPATIBILITY_R {
                PIXEL_PANNING_COMPATIBILITY_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn enable_blinking_slash_select_background_intensity(
                &self,
            ) -> ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R {
                ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R::from_register_value(
                    self.raw_bits,
                )
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn enable_line_graphics_character_code(
                &self,
            ) -> ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R {
                ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn select_display_type(&self) -> SELECT_DISPLAY_TYPE_R {
                SELECT_DISPLAY_TYPE_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn graphics_slash_alphanumeric_mode(&self) -> GRAPHICS_SLASH_ALPHANUMERIC_MODE_R {
                GRAPHICS_SLASH_ALPHANUMERIC_MODE_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `palette_bits_p5_p4_select`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PALETTE_BITS_P5_P4_SELECT_R {
            _Reserved(bool),
        }
        impl PALETTE_BITS_P5_P4_SELECT_R {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    PALETTE_BITS_P5_P4_SELECT_R::_Reserved(true) => Self::_MASK,
                    PALETTE_BITS_P5_P4_SELECT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                PALETTE_BITS_P5_P4_SELECT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    PALETTE_BITS_P5_P4_SELECT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `pixel_width_slash_clock_select`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PIXEL_WIDTH_SLASH_CLOCK_SELECT_R {
            _Reserved(bool),
        }
        impl PIXEL_WIDTH_SLASH_CLOCK_SELECT_R {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    PIXEL_WIDTH_SLASH_CLOCK_SELECT_R::_Reserved(true) => Self::_MASK,
                    PIXEL_WIDTH_SLASH_CLOCK_SELECT_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                PIXEL_WIDTH_SLASH_CLOCK_SELECT_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    PIXEL_WIDTH_SLASH_CLOCK_SELECT_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `pixel_panning_compatibility`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PIXEL_PANNING_COMPATIBILITY_R {
            _Reserved(bool),
        }
        impl PIXEL_PANNING_COMPATIBILITY_R {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    PIXEL_PANNING_COMPATIBILITY_R::_Reserved(true) => Self::_MASK,
                    PIXEL_PANNING_COMPATIBILITY_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                PIXEL_PANNING_COMPATIBILITY_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    PIXEL_PANNING_COMPATIBILITY_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `enable_blinking_slash_select_background_intensity`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R {
            _Reserved(bool),
        }
        impl ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R::_Reserved(true) => {
                        Self::_MASK
                    }
                    ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `enable_line_graphics_character_code`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R {
            _Reserved(bool),
        }
        impl ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R::_Reserved(true) => Self::_MASK,
                    ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_LINE_GRAPHICS_CHARACTER_CODE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `select_display_type`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum SELECT_DISPLAY_TYPE_R {
            _Reserved(bool),
        }
        impl SELECT_DISPLAY_TYPE_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    SELECT_DISPLAY_TYPE_R::_Reserved(true) => Self::_MASK,
                    SELECT_DISPLAY_TYPE_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                SELECT_DISPLAY_TYPE_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    SELECT_DISPLAY_TYPE_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `graphics_slash_alphanumeric_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum GRAPHICS_SLASH_ALPHANUMERIC_MODE_R {
            ALPHANUMERIC_MODE = 0,
            GRAPHICS_MODE = 1,
        }
        impl GRAPHICS_SLASH_ALPHANUMERIC_MODE_R {
            #[doc = "Checks if the value of the field is `ALPHANUMERIC_MODE`"]
            #[inline]
            pub fn is_alphanumeric_mode(&self) -> bool {
                *self == GRAPHICS_SLASH_ALPHANUMERIC_MODE_R::ALPHANUMERIC_MODE
            }
            #[doc = "Checks if the value of the field is `GRAPHICS_MODE`"]
            #[inline]
            pub fn is_graphics_mode(&self) -> bool {
                *self == GRAPHICS_SLASH_ALPHANUMERIC_MODE_R::GRAPHICS_MODE
            }
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => GRAPHICS_SLASH_ALPHANUMERIC_MODE_R::ALPHANUMERIC_MODE,
                    1 => GRAPHICS_SLASH_ALPHANUMERIC_MODE_R::GRAPHICS_MODE,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
            #[inline]
            pub fn bit(&self) -> bool {
                self.bits() == 1
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 7"]
            #[inline]
            pub fn palette_bits_p5_p4_select(&mut self) -> _PALETTE_BITS_P5_P4_SELECT<'_> {
                _PALETTE_BITS_P5_P4_SELECT { w: self }
            }
            #[doc = "Bit 6"]
            #[inline]
            pub fn pixel_width_slash_clock_select(
                &mut self,
            ) -> _PIXEL_WIDTH_SLASH_CLOCK_SELECT<'_> {
                _PIXEL_WIDTH_SLASH_CLOCK_SELECT { w: self }
            }
            #[doc = "Bit 5"]
            #[inline]
            pub fn pixel_panning_compatibility(&mut self) -> _PIXEL_PANNING_COMPATIBILITY<'_> {
                _PIXEL_PANNING_COMPATIBILITY { w: self }
            }
            #[doc = "Bit 3"]
            #[inline]
            pub fn enable_blinking_slash_select_background_intensity(
                &mut self,
            ) -> _ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY<'_> {
                _ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn enable_line_graphics_character_code(
                &mut self,
            ) -> _ENABLE_LINE_GRAPHICS_CHARACTER_CODE<'_> {
                _ENABLE_LINE_GRAPHICS_CHARACTER_CODE { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn select_display_type(&mut self) -> _SELECT_DISPLAY_TYPE<'_> {
                _SELECT_DISPLAY_TYPE { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn graphics_slash_alphanumeric_mode(
                &mut self,
            ) -> _GRAPHICS_SLASH_ALPHANUMERIC_MODE<'_> {
                _GRAPHICS_SLASH_ALPHANUMERIC_MODE { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_BITS_P5_P4_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_BITS_P5_P4_SELECT<'a> {
            const _MASK: u8 = 128;
            const _OFFSET: u8 = 7;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _PIXEL_WIDTH_SLASH_CLOCK_SELECT<'a> {
            w: &'a mut W,
        }
        impl<'a> _PIXEL_WIDTH_SLASH_CLOCK_SELECT<'a> {
            const _MASK: u8 = 64;
            const _OFFSET: u8 = 6;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _PIXEL_PANNING_COMPATIBILITY<'a> {
            w: &'a mut W,
        }
        impl<'a> _PIXEL_PANNING_COMPATIBILITY<'a> {
            const _MASK: u8 = 32;
            const _OFFSET: u8 = 5;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_BLINKING_SLASH_SELECT_BACKGROUND_INTENSITY<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_LINE_GRAPHICS_CHARACTER_CODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_LINE_GRAPHICS_CHARACTER_CODE<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _SELECT_DISPLAY_TYPE<'a> {
            w: &'a mut W,
        }
        impl<'a> _SELECT_DISPLAY_TYPE<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[repr(u8)]
        #[doc = "Values that can be written to the field `graphics_slash_alphanumeric_mode`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum GRAPHICS_SLASH_ALPHANUMERIC_MODE_W {
            ALPHANUMERIC_MODE = 0,
            GRAPHICS_MODE = 1,
        }
        impl GRAPHICS_SLASH_ALPHANUMERIC_MODE_W {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
        }
        #[doc = "Proxy"]
        pub struct _GRAPHICS_SLASH_ALPHANUMERIC_MODE<'a> {
            w: &'a mut W,
        }
        impl<'a> _GRAPHICS_SLASH_ALPHANUMERIC_MODE<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
            #[doc = "Writes `variant` to the field"]
            #[inline]
            pub fn variant(self, variant: GRAPHICS_SLASH_ALPHANUMERIC_MODE_W) -> &'a mut W {
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= variant.to_register_value();
                self.w
            }
            #[inline]
            pub fn alphanumeric_mode(self) -> &'a mut W {
                self.variant(GRAPHICS_SLASH_ALPHANUMERIC_MODE_W::ALPHANUMERIC_MODE)
            }
            #[inline]
            pub fn graphics_mode(self) -> &'a mut W {
                self.variant(GRAPHICS_SLASH_ALPHANUMERIC_MODE_W::GRAPHICS_MODE)
            }
        }
    }
    #[doc = "Memory Plane Enable Register"]
    pub struct AR12<
        'a,
        T: RegisterIndexIoR<AttributeControllerGroup, u8>
            + RegisterIndexIoW<AttributeControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod ar12 {
        use super::super::register_trait::*;
        use super::AttributeControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexR for super::AR12<'a, T>
        {
            const INDEX_R: u8 = 18;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexW for super::AR12<'a, T>
        {
            const INDEX_W: u8 = 18;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > InGroup for super::AR12<'a, T>
        {
            type Group = AttributeControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > super::AR12<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("AR12")
                    .field("enable_plane_3", &self.enable_plane_3().bit())
                    .field("enable_plane_2", &self.enable_plane_2().bit())
                    .field("enable_plane_1", &self.enable_plane_1().bit())
                    .field("enable_plane_0", &self.enable_plane_0().bit())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 3"]
            #[inline]
            pub fn enable_plane_3(&self) -> ENABLE_PLANE_3_R {
                ENABLE_PLANE_3_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn enable_plane_2(&self) -> ENABLE_PLANE_2_R {
                ENABLE_PLANE_2_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn enable_plane_1(&self) -> ENABLE_PLANE_1_R {
                ENABLE_PLANE_1_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn enable_plane_0(&self) -> ENABLE_PLANE_0_R {
                ENABLE_PLANE_0_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `enable_plane_3`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_PLANE_3_R {
            _Reserved(bool),
        }
        impl ENABLE_PLANE_3_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_PLANE_3_R::_Reserved(true) => Self::_MASK,
                    ENABLE_PLANE_3_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_PLANE_3_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_PLANE_3_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `enable_plane_2`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_PLANE_2_R {
            _Reserved(bool),
        }
        impl ENABLE_PLANE_2_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_PLANE_2_R::_Reserved(true) => Self::_MASK,
                    ENABLE_PLANE_2_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_PLANE_2_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_PLANE_2_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `enable_plane_1`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_PLANE_1_R {
            _Reserved(bool),
        }
        impl ENABLE_PLANE_1_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_PLANE_1_R::_Reserved(true) => Self::_MASK,
                    ENABLE_PLANE_1_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_PLANE_1_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_PLANE_1_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `enable_plane_0`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ENABLE_PLANE_0_R {
            _Reserved(bool),
        }
        impl ENABLE_PLANE_0_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ENABLE_PLANE_0_R::_Reserved(true) => Self::_MASK,
                    ENABLE_PLANE_0_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ENABLE_PLANE_0_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ENABLE_PLANE_0_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 3"]
            #[inline]
            pub fn enable_plane_3(&mut self) -> _ENABLE_PLANE_3<'_> {
                _ENABLE_PLANE_3 { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn enable_plane_2(&mut self) -> _ENABLE_PLANE_2<'_> {
                _ENABLE_PLANE_2 { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn enable_plane_1(&mut self) -> _ENABLE_PLANE_1<'_> {
                _ENABLE_PLANE_1 { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn enable_plane_0(&mut self) -> _ENABLE_PLANE_0<'_> {
                _ENABLE_PLANE_0 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_PLANE_3<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_PLANE_3<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_PLANE_2<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_PLANE_2<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_PLANE_1<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_PLANE_1<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ENABLE_PLANE_0<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENABLE_PLANE_0<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
    #[doc = "Horizontal Pixel Panning Register"]
    pub struct AR13<
        'a,
        T: RegisterIndexIoR<AttributeControllerGroup, u8>
            + RegisterIndexIoW<AttributeControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod ar13 {
        use super::super::register_trait::*;
        use super::AttributeControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexR for super::AR13<'a, T>
        {
            const INDEX_R: u8 = 19;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexW for super::AR13<'a, T>
        {
            const INDEX_W: u8 = 19;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > InGroup for super::AR13<'a, T>
        {
            type Group = AttributeControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > super::AR13<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("AR13")
                    .field(
                        "horizontal_pixel_shift",
                        &self.horizontal_pixel_shift().bits(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn horizontal_pixel_shift(&self) -> HORIZONTAL_PIXEL_SHIFT_R {
                HORIZONTAL_PIXEL_SHIFT_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `horizontal_pixel_shift`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum HORIZONTAL_PIXEL_SHIFT_R {
            _Reserved(u8),
        }
        impl HORIZONTAL_PIXEL_SHIFT_R {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    HORIZONTAL_PIXEL_SHIFT_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                HORIZONTAL_PIXEL_SHIFT_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    HORIZONTAL_PIXEL_SHIFT_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 3:0"]
            #[inline]
            pub fn horizontal_pixel_shift(&mut self) -> _HORIZONTAL_PIXEL_SHIFT<'_> {
                _HORIZONTAL_PIXEL_SHIFT { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _HORIZONTAL_PIXEL_SHIFT<'a> {
            w: &'a mut W,
        }
        impl<'a> _HORIZONTAL_PIXEL_SHIFT<'a> {
            const _MASK: u8 = 15;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Color Select Register"]
    pub struct AR14<
        'a,
        T: RegisterIndexIoR<AttributeControllerGroup, u8>
            + RegisterIndexIoW<AttributeControllerGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod ar14 {
        use super::super::register_trait::*;
        use super::AttributeControllerGroup;
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexR for super::AR14<'a, T>
        {
            const INDEX_R: u8 = 20;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > LocationIndexW for super::AR14<'a, T>
        {
            const INDEX_W: u8 = 20;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > InGroup for super::AR14<'a, T>
        {
            type Group = AttributeControllerGroup;
        }
        impl<
                'a,
                T: RegisterIndexIoR<AttributeControllerGroup, u8>
                    + RegisterIndexIoW<AttributeControllerGroup, u8>,
            > super::AR14<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::INDEX_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::INDEX_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("AR14")
                    .field("palette_bit_7", &self.palette_bit_7().bit())
                    .field("palette_bit_6", &self.palette_bit_6().bit())
                    .field(
                        "alternate_palette_bit_5",
                        &self.alternate_palette_bit_5().bit(),
                    )
                    .field(
                        "alternate_palette_bit_4",
                        &self.alternate_palette_bit_4().bit(),
                    )
                    .finish()
            }
        }
        impl R {
            #[doc = "Bit 3"]
            #[inline]
            pub fn palette_bit_7(&self) -> PALETTE_BIT_7_R {
                PALETTE_BIT_7_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn palette_bit_6(&self) -> PALETTE_BIT_6_R {
                PALETTE_BIT_6_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn alternate_palette_bit_5(&self) -> ALTERNATE_PALETTE_BIT_5_R {
                ALTERNATE_PALETTE_BIT_5_R::from_register_value(self.raw_bits)
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn alternate_palette_bit_4(&self) -> ALTERNATE_PALETTE_BIT_4_R {
                ALTERNATE_PALETTE_BIT_4_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `palette_bit_7`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PALETTE_BIT_7_R {
            _Reserved(bool),
        }
        impl PALETTE_BIT_7_R {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    PALETTE_BIT_7_R::_Reserved(true) => Self::_MASK,
                    PALETTE_BIT_7_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                PALETTE_BIT_7_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    PALETTE_BIT_7_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `palette_bit_6`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PALETTE_BIT_6_R {
            _Reserved(bool),
        }
        impl PALETTE_BIT_6_R {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    PALETTE_BIT_6_R::_Reserved(true) => Self::_MASK,
                    PALETTE_BIT_6_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                PALETTE_BIT_6_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    PALETTE_BIT_6_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `alternate_palette_bit_5`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ALTERNATE_PALETTE_BIT_5_R {
            _Reserved(bool),
        }
        impl ALTERNATE_PALETTE_BIT_5_R {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ALTERNATE_PALETTE_BIT_5_R::_Reserved(true) => Self::_MASK,
                    ALTERNATE_PALETTE_BIT_5_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ALTERNATE_PALETTE_BIT_5_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ALTERNATE_PALETTE_BIT_5_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Possible values of the field `alternate_palette_bit_4`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum ALTERNATE_PALETTE_BIT_4_R {
            _Reserved(bool),
        }
        impl ALTERNATE_PALETTE_BIT_4_R {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                match *self {
                    ALTERNATE_PALETTE_BIT_4_R::_Reserved(true) => Self::_MASK,
                    ALTERNATE_PALETTE_BIT_4_R::_Reserved(false) => 0,
                }
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                ALTERNATE_PALETTE_BIT_4_R::_Reserved(value == Self::_MASK)
            }
            #[inline]
            pub fn bit(&self) -> bool {
                match *self {
                    ALTERNATE_PALETTE_BIT_4_R::_Reserved(value) => value,
                }
            }
            #[doc = "Returns `false` if the bit is clear (0)"]
            #[inline]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = "Returns `true` if the bit is set (1)"]
            #[inline]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bit 3"]
            #[inline]
            pub fn palette_bit_7(&mut self) -> _PALETTE_BIT_7<'_> {
                _PALETTE_BIT_7 { w: self }
            }
            #[doc = "Bit 2"]
            #[inline]
            pub fn palette_bit_6(&mut self) -> _PALETTE_BIT_6<'_> {
                _PALETTE_BIT_6 { w: self }
            }
            #[doc = "Bit 1"]
            #[inline]
            pub fn alternate_palette_bit_5(&mut self) -> _ALTERNATE_PALETTE_BIT_5<'_> {
                _ALTERNATE_PALETTE_BIT_5 { w: self }
            }
            #[doc = "Bit 0"]
            #[inline]
            pub fn alternate_palette_bit_4(&mut self) -> _ALTERNATE_PALETTE_BIT_4<'_> {
                _ALTERNATE_PALETTE_BIT_4 { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_BIT_7<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_BIT_7<'a> {
            const _MASK: u8 = 8;
            const _OFFSET: u8 = 3;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_BIT_6<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_BIT_6<'a> {
            const _MASK: u8 = 4;
            const _OFFSET: u8 = 2;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ALTERNATE_PALETTE_BIT_5<'a> {
            w: &'a mut W,
        }
        impl<'a> _ALTERNATE_PALETTE_BIT_5<'a> {
            const _MASK: u8 = 2;
            const _OFFSET: u8 = 1;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
        #[doc = "Proxy"]
        pub struct _ALTERNATE_PALETTE_BIT_4<'a> {
            w: &'a mut W,
        }
        impl<'a> _ALTERNATE_PALETTE_BIT_4<'a> {
            const _MASK: u8 = 1;
            const _OFFSET: u8 = 0;
            #[doc = "Sets the field bit"]
            #[inline]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = "Clears the field bit"]
            #[inline]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bit(self, value: bool) -> &'a mut W {
                if value {
                    self.w.raw_bits |= Self::_MASK;
                } else {
                    self.w.raw_bits &= !Self::_MASK;
                }
                self.w
            }
        }
    }
}
pub mod color_palette {
    use super::register_trait::*;
    pub struct ColorPaletteRegisters<
        T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
    > {
        io: T,
    }
    impl<T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>>
        ColorPaletteRegisters<T>
    {
        #[inline]
        pub fn new(io: T) -> Self {
            Self { io }
        }
        #[doc = "Pixel Data Mask Register"]
        #[inline]
        pub fn dacmask(&mut self) -> DACMASK<'_, T> {
            DACMASK { io: &mut self.io }
        }
        #[doc = "DAC State Register"]
        #[inline]
        pub fn dacstate(&mut self) -> DACSTATE<'_, T> {
            DACSTATE { io: &mut self.io }
        }
        #[doc = "Palette Read Index Register"]
        #[inline]
        pub fn dacrx(&mut self) -> DACRX<'_, T> {
            DACRX { io: &mut self.io }
        }
        #[doc = "Palette Write Index Register"]
        #[inline]
        pub fn dacwx(&mut self) -> DACWX<'_, T> {
            DACWX { io: &mut self.io }
        }
        #[doc = "Palette Data Register"]
        #[inline]
        pub fn dacdata(&mut self) -> DACDATA<'_, T> {
            DACDATA { io: &mut self.io }
        }
        pub fn debug_registers<F: FnMut(&dyn core::fmt::Debug)>(&mut self, mut f: F) {
            (f)(&self.dacmask().read());
            (f)(&self.dacstate().read());
            (f)(&self.dacdata().read());
        }
    }
    pub struct ColorPaletteGroup;
    impl RegisterGroup for ColorPaletteGroup {}
    #[doc = "Pixel Data Mask Register"]
    pub struct DACMASK<
        'a,
        T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod dacmask {
        use super::super::register_trait::*;
        use super::ColorPaletteGroup;
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > LocationAbsR for super::DACMASK<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 966;
        }
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > LocationAbsW for super::DACMASK<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 966;
        }
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > InGroup for super::DACMASK<'a, T>
        {
            type Group = ColorPaletteGroup;
        }
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > super::DACMASK<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("DACMASK")
                    .field("pixel_data_mask", &self.pixel_data_mask().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn pixel_data_mask(&self) -> PIXEL_DATA_MASK_R {
                PIXEL_DATA_MASK_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `pixel_data_mask`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PIXEL_DATA_MASK_R {
            _Reserved(u8),
        }
        impl PIXEL_DATA_MASK_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    PIXEL_DATA_MASK_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                PIXEL_DATA_MASK_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    PIXEL_DATA_MASK_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn pixel_data_mask(&mut self) -> _PIXEL_DATA_MASK<'_> {
                _PIXEL_DATA_MASK { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PIXEL_DATA_MASK<'a> {
            w: &'a mut W,
        }
        impl<'a> _PIXEL_DATA_MASK<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "DAC State Register"]
    pub struct DACSTATE<'a, T: RegisterAbsIoR<ColorPaletteGroup, u8>> {
        io: &'a mut T,
    }
    pub mod dacstate {
        use super::super::register_trait::*;
        use super::ColorPaletteGroup;
        impl<'a, T: RegisterAbsIoR<ColorPaletteGroup, u8>> LocationAbsR for super::DACSTATE<'a, T> {
            const ABS_ADDRESS_R: u16 = 967;
        }
        impl<'a, T: RegisterAbsIoR<ColorPaletteGroup, u8>> InGroup for super::DACSTATE<'a, T> {
            type Group = ColorPaletteGroup;
        }
        impl<'a, T: RegisterAbsIoR<ColorPaletteGroup, u8>> super::DACSTATE<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("DACSTATE")
                    .field("dac_state", &format_args!("{:?}", self.dac_state()))
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 1:0"]
            #[inline]
            pub fn dac_state(&self) -> DAC_STATE_R {
                DAC_STATE_R::from_register_value(self.raw_bits)
            }
        }
        #[repr(u8)]
        #[doc = "Possible values of the field `dac_state`"]
        #[doc = ""]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum DAC_STATE_R {
            PALETTE_WRITE_INDEX_REGISTER = 0,
            RESERVED_1 = 1,
            RESERVED_2 = 2,
            PALETTE_READ_INDEX_REGISTER = 3,
        }
        impl DAC_STATE_R {
            #[doc = "Checks if the value of the field is `PALETTE_WRITE_INDEX_REGISTER`"]
            #[inline]
            pub fn is_palette_write_index_register(&self) -> bool {
                *self == DAC_STATE_R::PALETTE_WRITE_INDEX_REGISTER
            }
            #[doc = "Checks if the value of the field is `RESERVED_1`"]
            #[inline]
            pub fn is_reserved_1(&self) -> bool {
                *self == DAC_STATE_R::RESERVED_1
            }
            #[doc = "Checks if the value of the field is `RESERVED_2`"]
            #[inline]
            pub fn is_reserved_2(&self) -> bool {
                *self == DAC_STATE_R::RESERVED_2
            }
            #[doc = "Checks if the value of the field is `PALETTE_READ_INDEX_REGISTER`"]
            #[inline]
            pub fn is_palette_read_index_register(&self) -> bool {
                *self == DAC_STATE_R::PALETTE_READ_INDEX_REGISTER
            }
            const _MASK: u8 = 3;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = *self as u8;
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                match value {
                    0 => DAC_STATE_R::PALETTE_WRITE_INDEX_REGISTER,
                    1 => DAC_STATE_R::RESERVED_1,
                    2 => DAC_STATE_R::RESERVED_2,
                    3 => DAC_STATE_R::PALETTE_READ_INDEX_REGISTER,
                    _ => unreachable!(),
                }
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                *self as u8
            }
        }
    }
    #[doc = "Palette Read Index Register"]
    pub struct DACRX<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> {
        io: &'a mut T,
    }
    pub mod dacrx {
        use super::super::register_trait::*;
        use super::ColorPaletteGroup;
        impl<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> LocationAbsW for super::DACRX<'a, T> {
            const ABS_ADDRESS_W: u16 = 967;
        }
        impl<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> InGroup for super::DACRX<'a, T> {
            type Group = ColorPaletteGroup;
        }
        impl<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> super::DACRX<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn palette_read_index(&mut self) -> _PALETTE_READ_INDEX<'_> {
                _PALETTE_READ_INDEX { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_READ_INDEX<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_READ_INDEX<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Palette Write Index Register"]
    pub struct DACWX<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> {
        io: &'a mut T,
    }
    pub mod dacwx {
        use super::super::register_trait::*;
        use super::ColorPaletteGroup;
        impl<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> LocationAbsW for super::DACWX<'a, T> {
            const ABS_ADDRESS_W: u16 = 968;
        }
        impl<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> InGroup for super::DACWX<'a, T> {
            type Group = ColorPaletteGroup;
        }
        impl<'a, T: RegisterAbsIoW<ColorPaletteGroup, u8>> super::DACWX<'a, T> {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn palette_write_index(&mut self) -> _PALETTE_WRITE_INDEX<'_> {
                _PALETTE_WRITE_INDEX { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_WRITE_INDEX<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_WRITE_INDEX<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
    #[doc = "Palette Data Register"]
    pub struct DACDATA<
        'a,
        T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
    > {
        io: &'a mut T,
    }
    pub mod dacdata {
        use super::super::register_trait::*;
        use super::ColorPaletteGroup;
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > LocationAbsR for super::DACDATA<'a, T>
        {
            const ABS_ADDRESS_R: u16 = 969;
        }
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > LocationAbsW for super::DACDATA<'a, T>
        {
            const ABS_ADDRESS_W: u16 = 969;
        }
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > InGroup for super::DACDATA<'a, T>
        {
            type Group = ColorPaletteGroup;
        }
        impl<
                'a,
                T: RegisterAbsIoR<ColorPaletteGroup, u8> + RegisterAbsIoW<ColorPaletteGroup, u8>,
            > super::DACDATA<'a, T>
        {
            pub fn new(io: &'a mut T) -> Self {
                Self { io }
            }
            #[doc = "Modifies the contents of the register"]
            #[inline]
            pub fn modify<F>(&mut self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let r = self.read();
                let mut w = W {
                    raw_bits: r.raw_bits,
                };
                (f)(&r, &mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
            #[doc = "Reads the contents of the register"]
            #[inline]
            pub fn read(&mut self) -> R {
                R {
                    raw_bits: self.io.read(Self::ABS_ADDRESS_R),
                }
            }
            #[doc = "Writes to the register"]
            #[inline]
            pub fn write<F>(&mut self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W { raw_bits: 0 };
                (f)(&mut w);
                self.io.write(Self::ABS_ADDRESS_W, w.raw_bits);
            }
        }
        #[doc = "Value to write to the register"]
        pub struct R {
            raw_bits: u8,
        }
        impl core::fmt::Debug for R {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_struct("DACDATA")
                    .field("palette_data", &self.palette_data().bits())
                    .finish()
            }
        }
        impl R {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn palette_data(&self) -> PALETTE_DATA_R {
                PALETTE_DATA_R::from_register_value(self.raw_bits)
            }
        }
        #[doc = "Possible values of the field `palette_data`"]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PALETTE_DATA_R {
            _Reserved(u8),
        }
        impl PALETTE_DATA_R {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[inline]
            pub fn to_register_value(&self) -> u8 {
                let value = match *self {
                    PALETTE_DATA_R::_Reserved(value) => value,
                };
                let value = value << Self::_OFFSET;
                value
            }
            #[inline]
            pub fn from_register_value(value: u8) -> Self {
                let value = value & Self::_MASK;
                let value = value >> Self::_OFFSET;
                PALETTE_DATA_R::_Reserved(value)
            }
            #[doc = "Value of the field as raw bits"]
            #[inline]
            pub fn bits(&self) -> u8 {
                match *self {
                    PALETTE_DATA_R::_Reserved(value) => value,
                }
            }
        }
        #[doc = "Value read from the register"]
        pub struct W {
            raw_bits: u8,
        }
        impl W {
            #[doc = "Bits 7:0"]
            #[inline]
            pub fn palette_data(&mut self) -> _PALETTE_DATA<'_> {
                _PALETTE_DATA { w: self }
            }
        }
        #[doc = "Proxy"]
        pub struct _PALETTE_DATA<'a> {
            w: &'a mut W,
        }
        impl<'a> _PALETTE_DATA<'a> {
            const _MASK: u8 = 255;
            const _OFFSET: u8 = 0;
            #[doc = "Writes raw bits to the field"]
            #[inline]
            pub fn bits(self, value: u8) -> &'a mut W {
                let value = value << Self::_OFFSET;
                let value = value & Self::_MASK;
                self.w.raw_bits &= !Self::_MASK;
                self.w.raw_bits |= value;
                self.w
            }
        }
    }
}
