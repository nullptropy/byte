mod opcode_types;

pub mod opcode {
    pub use super::opcode_types::*;
    pub const OPCODE_MAP: [Option<Opcode>; 255] =
        include!(concat!(env!("OUT_DIR"), "/opcode_arr.rs"));

    pub fn get_opcode(name: &str) -> Option<&Opcode> {
        OPCODE_MAP
            .iter()
            .flatten()
            .find(|opcode| opcode.name.eq_ignore_ascii_case(name))
    }
}
