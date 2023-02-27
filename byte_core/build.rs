#[path = "src/opcode.rs"]
mod opcode;

fn main() {
    let opcodes: Vec<Option<opcode::Opcode>> = {
        let mut result = vec![None; 0xff];
        let opcodes: Vec<opcode::Opcode> =
            serde_json::from_str(include_str!("scripts/instructions.json"))
                .expect("failed to parse json file");

        opcodes
            .into_iter()
            .for_each(|opcode| result[opcode.code as usize] = Some(opcode));

        result
    };

    uneval::to_out_dir(opcodes, "opcode_vec.rs")
        .expect("failed to construct the opcode_vec.rs file");
}
