mod common;

#[test]
fn opcode_0xf0_relative_beq() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0xf0, 0xfb,  // BCC rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.p.insert(common::cpu::Flags::ZERO);
    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BCS

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}