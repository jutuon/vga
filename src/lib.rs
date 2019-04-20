

//! Driver for VGA graphics.
//!
//! # Reference material
//! * <http://classiccomputers.info/down/IBM_PS2/documents/PS2_Hardware_Interface_Technical_Reference_May88.pdf>
//!     * PDF page 388
//! * <https://wiki.osdev.org/VGA_Hardware>
//! * [Intel® OpenSource HD Graphics Programmer’s Reference Manual (PRM) Volume 3 Part 1: VGA and Extended VGA Registers (Ivy Bridge)](https://01.org/sites/default/files/documentation/ivb_ihd_os_vol3_part1_0.pdf)

#![no_std]
#![forbid(missing_debug_implementations, unsafe_code)]

pub mod raw;
#[macro_use]
pub mod io;
pub mod driver;
