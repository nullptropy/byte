use serde::{Deserialize, Serialize};

fn main() {
    let opcodes: Vec<Option<Opcode>> = {
        let mut result = vec![None; 0xff];
        let opcodes: Vec<Opcode> = serde_json::from_str(include_str!("misc/instructions.json"))
            .expect("failed to parse json file");

        opcodes
            .into_iter()
            .for_each(|opcode| result[opcode.code as usize] = Some(opcode));

        result
    };

    let mut identifier = String::new();
    let mut opcode_map = String::new();

    identifier.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString)]\n");
    identifier.push_str("pub enum Mnemonic {\n");
    opcode_map.push_str("[\n");

    for opcode in opcodes.iter() {
        match opcode {
            Some(op) => {
                let mnemonic = op.name.to_uppercase();
                let tick_modifier =
                    format!("{:?}", op.tick_modifier).replace("Some(", "Some(TickModifier::");

                if !identifier.contains(mnemonic.as_str()) {
                    identifier.push_str(format!("{},", mnemonic).as_str());
                }
                opcode_map.push_str(
                    format!(
                        "Some(Opcode {{
                            code: {},
                            size: {},
                            tick: {},
                            tick_modifier: {},
                            mnemonic: Mnemonic::{},
                            mode: AddressingMode::{:?}
                        }}),",
                        op.code, op.size, op.tick, tick_modifier, mnemonic, op.mode,
                    )
                    .as_str(),
                );
            }
            None => opcode_map.push_str("None,"),
        }
    }

    identifier.push('}');
    opcode_map.push(']');

    write_to_out_dir("mnemonics.rs", identifier.as_str());
    write_to_out_dir("opcode_arr.rs", opcode_map.as_str());
}

fn write_to_out_dir(filename: &str, content: &str) {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = std::path::PathBuf::from(out_dir).join(filename);
    std::fs::write(dest_path, content).expect("Could not write file");
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TickModifier {
    Branch,
    PageCrossed,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
