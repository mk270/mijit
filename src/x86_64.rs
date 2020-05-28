/*
pub use std::fmt::{Debug};

pub enum Error {
    OutOfMemory,
    Other(&'static dyn std::error::Error),
}

impl std::error::Error for Error {
}

/** A placeholder. I'm not sure what will implement this, but whatever it is,
 * we should probably move all the methods there and delete this trait. */
pub trait Buffer {
    fn write1(&mut self) -> Result;
}
*/

pub struct Register(usize);

pub const A: Register = Register(0);
pub const D: Register = Register(1);
pub const C: Register = Register(2);
pub const B: Register = Register(3);

pub const ALLOCATABLE_REGISTERS: [Register; 4] = [A, D, C, B];
