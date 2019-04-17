

//! Driver for VGA graphics.
//!
//! # Reference material
//! * <http://classiccomputers.info/down/IBM_PS2/documents/PS2_Hardware_Interface_Technical_Reference_May88.pdf>
//!     * PDF page 388
//! * <https://wiki.osdev.org/VGA_Hardware>

#![no_std]
#![forbid(missing_debug_implementations, unsafe_code)]

pub mod raw;
#[macro_use]
pub mod io;
pub mod driver;
