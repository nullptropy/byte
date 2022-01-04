mod common;

#[test]
fn opcode_0x90_relative_bcc_1() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0x90, 0xfb,  // BCC rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BCC

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}

#[test]
fn opcode_0x90_relative_bcc_2() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0x90, 0x05,  // BCC rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BCC

    assert_eq!(cpu.reg.pc, 0x8006); // 0x8000 + 1 + 5
}