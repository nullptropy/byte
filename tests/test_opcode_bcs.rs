mod common;

#[test]
fn opcode_0xb0_relative_bcs() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0xb0, 0xfb,  // BCC rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.p.insert(common::cpu::Flags::CARRY);
    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BCS

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}