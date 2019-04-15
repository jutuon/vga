

macro_rules! declare_register_type {
    ($name:tt) => {
        #[derive(Debug)]
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
    ($name:tt, $flags_type:tt) => {
        declare_register_type!($name);

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

macro_rules! simple_register_value {
    ($getter_name:tt, $setter_name:tt, $mask:expr, $($doc:expr),*) => {
        $(
            #[doc=$doc]
        )*
        pub fn $getter_name(&self) -> u8 {
            self.0 & $mask
        }

        $(
            #[doc=$doc]
        )*
        pub fn $setter_name(&mut self, value: u8) {
            crate::raw::remove_bits(&mut self.0, $mask);
            self.0 |= value & $mask;
        }
    };
}

macro_rules! register_boolean {
    ($getter_name:tt, $setter_name:tt, $mask:expr, $($doc:expr),* ) => {
        $(
            #[doc=$doc]
        )*
        pub fn $getter_name(&self) -> bool {
            self.0 & $mask == $mask
        }

        $(
            #[doc=$doc]
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
    ($getter_name:tt, $setter_name:tt, $name:ty, $($doc:expr),*) => {
        $(
            #[doc=$doc]
        )*
        pub fn $getter_name(&self) -> Result<$name, crate::raw::UnknownValue> {
            $name::from_register_value(self.0)
        }

        $(
            #[doc=$doc]
        )*
        pub fn $setter_name(&mut self, value: $name) {
            value.update_register_value(&mut self.0)
        }
    };
}

macro_rules! register_enum_with_unwrap {
    ($getter_name:tt, $setter_name:tt, $name:tt, $($doc:expr),*) => {
        $(
            #[doc=$doc]
        )*
        pub fn $getter_name(&self) -> $name {
            $name::from_register_value(self.0).unwrap()
        }

        $(
            #[doc=$doc]
        )*
        pub fn $setter_name(&mut self, value: $name) {
            value.update_register_value(&mut self.0)
        }
    };
}

macro_rules! declare_register_enum {
    (
        $(
            #[$enum_meta_item:meta]
        )*
        pub enum $name:ident {
            $(
                $(
                    #[$variant_meta_item:meta]
                )*
                $variant_name:ident = $value:expr
            ),+
        }
    ) => {
        #[repr(u8)]
        #[derive(Debug, TryFromPrimitive)]
        #[TryFromPrimitiveType="u8"]
        $(
            #[$enum_meta_item]
        )*
        pub enum $name {
            $(
                $(
                    #[$variant_meta_item]
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
