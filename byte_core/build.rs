use serde::{Deserialize, Serialize};

fn main() {
    let opcodes: Vec<Option<Opcode>> = {
        let mut result = vec![None; 0xff];
        let opcodes: Vec<Opcode> = serde_json::from_str(include_str!("scripts/instructions.json"))
            .expect("failed to parse json file");

        opcodes
            .into_iter()
            .for_each(|opcode| result[opcode.code as usize] = Some(opcode));

        result
    };

    // serde doesn't support serializing/deserializing big arrays (yet)
    // so i'm just serializing a `Vec<Option<Opcode>>` of size 0xff
    // and modifiying the output of the serializer into something like:
    // ```rust
    // [Some(Opcode { .. }), None, ..]
    // ```
    // and then in the `byte_core::opcode` module, `OPCODE_MAP` is defined as:
    // ```rust
    // pub const OPCODE_MAP: [Option<Opcode>; 255] =
    //     include!(concat!(env!("OUT_DIR"), "/opcode_arr.rs"));
    // ```
    let string = uneval::to_string(opcodes)
        .expect("failed to serialize the opcodes")
        .replace("vec!", "")
        .replace(".into()", "")
        .replace(".into_iter().collect()", "");

    let path: std::path::PathBuf = [
        std::env::var("OUT_DIR")
            .expect("OUT_DIR not set, check if you're running this from the build script"),
        "opcode_arr.rs".to_string(),
    ]
    .iter()
    .collect();

    std::fs::write(path, string).expect("failed to write the opcode_arr.rs file");
}

// these definitions are directly mirrored
// from the `byte_core::opcode` module so that
// `byte_core` itself doesn't depend on serde.

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum TickModifier {
    Branch,
    PageCrossed,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Relative,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Opcode {
    pub code: u8,
    pub size: u8,
    pub tick: u8,
    pub name: &'static str,
    pub mode: AddressingMode,
    pub tick_modifier: Option<TickModifier>,
}
