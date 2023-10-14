mod opcode_types;

pub mod opcode {
    pub use super::opcode_types::*;

    pub const OPCODE_MAP: [Option<Opcode>; 255] =
        include!(concat!(env!("OUT_DIR"), "/opcode_arr.rs"));

    // TODO: make this into a comp time hash map of some sort
    pub fn get_opcode(mnemonic: Mnemonic, mode: AddressingMode) -> Option<&'static Opcode> {
        OPCODE_MAP
            .iter()
            .flatten()
            .find(|opcode| opcode.mnemonic == mnemonic && opcode.mode == mode)
    }
}
