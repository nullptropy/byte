mod opcode_types;

pub mod opcode {
    pub use super::opcode_types::*;
    pub const OPCODE_MAP: [Option<Opcode>; 255] =
        include!(concat!(env!("OUT_DIR"), "/opcode_arr.rs"));
}
