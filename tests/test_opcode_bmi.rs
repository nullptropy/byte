mod common;

#[test]
fn opcode_0x30_relative_bmi() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0x30, 0xfb,  // BMI rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.p.insert(common::cpu::Flags::NEGATIVE);
    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BMI

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}