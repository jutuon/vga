

macro_rules! declare_register_type {
    ( $(#[doc=$text:literal] )* $name:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        #[derive(Debug, Clone, Copy)]
        pub struct $name(u8);

        impl $name {
            pub fn from_register_value(value: u8) -> Self {
                $name(value)
            }

            pub fn value(&self) -> u8 {
                self.0
            }
        }
    };
    ( $(#[doc=$text:literal] )* $name:ident, $flags_type:ident $(,)?) => {
        declare_register_type!(
            $(
                #[doc=$text]
            )*
            $name
        );

        impl $name {
            pub fn flags(&self) -> $flags_type {
                $flags_type::from_bits_truncate(self.0)
            }

            pub fn set_flags(&mut self, value: $flags_type) {
                crate::raw::remove_bits(&mut self.0, $flags_type::all().bits());
                self.0 |= value.bits();
            }
        }
    };
    ( $( #[doc=$text:literal] )* $name:ident, $index:literal $(,)?) => {
        declare_register_type!(
            $(
                #[doc=$text]
            )*
            $name
        );

        impl $name {
            pub const INDEX: u8 = $index;
        }
    };
    ( $(#[doc=$text:literal] )* $name:ident, $flags_type:ident, $index:literal $(,)?) => {
        declare_register_type!(
            $(
                #[doc=$text]
            )*
            $name,
            $flags_type,
        );

        impl $name {
            pub const INDEX: u8 = $index;
        }
    };

}

macro_rules! impl_from_enum_for_u8 {
    ($type:ty) => {
        impl core::convert::From<$type> for u8 {
            fn from(value: $type) -> u8 {
                value as u8
            }
        }
    };
}

macro_rules! register_value {
    ( $( #[doc=$text:literal] )* $getter_name:ident, $setter_name:ident, $mask:literal $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $getter_name(&self) -> u8 {
            self.0 & $mask
        }

        $(
            #[doc=$text]
        )*
        pub fn $setter_name(&mut self, value: u8) {
            crate::raw::remove_bits(&mut self.0, $mask);
            self.0 |= value & $mask;
        }
    };
    ( $( #[doc=$text:literal] )* $getter_name:ident, $setter_name:ident, $type:ty $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $getter_name(&self) -> $type {
            self.0 as $type
        }

        $(
            #[doc=$text]
        )*
        pub fn $setter_name(&mut self, value: $type) {
            self.0 = value as u8;
        }
    };
}

macro_rules! register_boolean {
    ( $( #[doc=$text:literal] )* $getter_name:ident, $setter_name:ident, $mask:literal $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $getter_name(&self) -> bool {
            self.0 & $mask == $mask
        }

        $(
            #[doc=$text]
        )*
        pub fn $setter_name(&mut self, value: bool) {
            crate::raw::remove_bits(&mut self.0, $mask);
            if value {
                self.0 |= $mask;
            }
        }
    };
}

macro_rules! register_enum {
    ( $( #[doc=$text:literal] )* $getter_name:ident, $setter_name:ident, $type_name:ident $(,)?) => {
        $(
            #[doc=$text]
        )*
        pub fn $getter_name(&self) -> Result<$type_name, crate::raw::UnknownValue> {
            $type_name::from_register_value(self.0)
        }

        $(
            #[doc=$text]
        )*
        pub fn $setter_name(&mut self, value: $type_name) {
            value.update_register_value(&mut self.0)
        }
    };
}

macro_rules! register_enum_with_unwrap {
    ( $( #[doc=$text:literal] )* $getter_name:tt, $setter_name:tt, $name:tt $(,)? ) => {
        $(
            #[doc=$text]
        )*
        pub fn $getter_name(&self) -> $name {
            $name::from_register_value(self.0).unwrap()
        }

        $(
            #[doc=$text]
        )*
        pub fn $setter_name(&mut self, value: $name) {
            value.update_register_value(&mut self.0)
        }
    };
}

macro_rules! declare_register_enum {
    (
        $(
            #[doc=$enum_text:literal]
        )*
        pub enum $name:ident {
            $(
                $(
                    #[doc=$variant_text:literal]
                )*
                $variant_name:ident = $value:literal
            ),+ $(,)?
        }
    ) => {
        #[repr(u8)]
        #[derive(Debug, TryFromPrimitive)]
        #[TryFromPrimitiveType="u8"]
        $(
            #[doc=$enum_text]
        )*
        pub enum $name {
            $(
                $(
                    #[doc=$variant_text]
                )*
                $variant_name = $value,
            )*
        }

        impl_from_enum_for_u8!($name);

        impl RegisterField for $name {
            const ALL_BITS_ON_MASK: u8 = $( $value |)* 0;
        }
    };
}

pub mod general;
pub mod sequencer;
pub mod crt_controller;
pub mod graphics_controller;
pub mod attribute_controller;
pub mod video_dac_palette;

pub fn extract_bit_from_u8(value: u8, i_old: BitIndexU8, i_new: BitIndexU16) -> u16 {
    let target_bit_as_first_bit = (value >> i_old as u8) & 1;
    (target_bit_as_first_bit as u16) << i_new as u16
}

pub fn extract_bit_from_u16(value: u16, i_old: BitIndexU16, i_new: BitIndexU8) -> u8 {
    let target_bit_as_first_bit = (value >> i_old as u16) & 1;
    (target_bit_as_first_bit as u8) << i_new as u8
}


#[repr(u8)]
#[derive(Debug)]
pub enum BitIndexU8 {
    I0 = 0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
}

#[repr(u8)]
#[derive(Debug)]
pub enum BitIndexU16 {
    I0 = 0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    I11,
    I12,
    I13,
    I14,
    I15,
}

pub fn remove_bits(register: &mut u8, all_flags_on: u8) {
    *register &= !all_flags_on;
}

#[derive(Debug)]
pub struct UnknownValue;


pub trait RegisterField: core::convert::TryFrom<u8> + Into<u8>  {
    const ALL_BITS_ON_MASK: u8;

    fn from_register_value(value: u8) -> Result<Self, UnknownValue> {
        let value = value & Self::ALL_BITS_ON_MASK;
        Self::try_from(value).map_err(|_| UnknownValue)
    }

    fn update_register_value(self, register: &mut u8) {
        remove_bits(register, Self::ALL_BITS_ON_MASK);
        *register |= self.into();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_bit_test1() {
        assert_eq!(extract_bit_from_u8(0b1000_0000, BitIndexU8::I7, BitIndexU16::I0), 1);
        assert_eq!(extract_bit_from_u8(0b0000_0001, BitIndexU8::I0, BitIndexU16::I0), 1);
        assert_eq!(extract_bit_from_u8(0b0000_0000, BitIndexU8::I0, BitIndexU16::I0), 0);
        assert_eq!(extract_bit_from_u8(0b0111_1111, BitIndexU8::I7, BitIndexU16::I0), 0);
        assert_eq!(extract_bit_from_u8(0b1111_1111, BitIndexU8::I7, BitIndexU16::I0), 1);
        assert_eq!(extract_bit_from_u8(0b1111_1111, BitIndexU8::I0, BitIndexU16::I0), 1);
    }

    #[test]
    fn extract_bit_test2() {
        assert_eq!(extract_bit_from_u8(1, BitIndexU8::I0, BitIndexU16::I0), 1);
        assert_eq!(extract_bit_from_u8(1, BitIndexU8::I0, BitIndexU16::I15), 0b1000_0000_0000_0000);
    }

    #[test]
    fn extract_bit_test3() {
        assert_eq!(extract_bit_from_u16(0b1000_0000, BitIndexU16::I7, BitIndexU8::I0), 1);
        assert_eq!(extract_bit_from_u16(0b0000_0001, BitIndexU16::I0, BitIndexU8::I0), 1);
        assert_eq!(extract_bit_from_u16(0b0000_0000, BitIndexU16::I0, BitIndexU8::I0), 0);
        assert_eq!(extract_bit_from_u16(0b0111_1111, BitIndexU16::I7, BitIndexU8::I0), 0);
        assert_eq!(extract_bit_from_u16(0b1111_1111, BitIndexU16::I7, BitIndexU8::I0), 1);
        assert_eq!(extract_bit_from_u16(0b1111_1111, BitIndexU16::I0, BitIndexU8::I0), 1);
        assert_eq!(extract_bit_from_u16(1 << 15, BitIndexU16::I15, BitIndexU8::I0), 1);
    }

    #[test]
    fn extract_bit_test4() {
        assert_eq!(extract_bit_from_u16(1, BitIndexU16::I0, BitIndexU8::I0), 1);
        assert_eq!(extract_bit_from_u16(1, BitIndexU16::I0, BitIndexU8::I7), 0b1000_0000);
    }

}
