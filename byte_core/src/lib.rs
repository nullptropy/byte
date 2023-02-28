#![allow(clippy::useless_conversion)]

pub mod bus;
pub mod cpu;
pub mod opcode;

#[derive(Debug)]
pub enum Error {
    UnrecognizedOpcode(u8),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnrecognizedOpcode(code) => {
                write!(f, "Unrecognized Opcode: {code:#04X}")
            }
        }
    }
}
