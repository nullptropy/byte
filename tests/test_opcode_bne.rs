mod common;

#[test]
fn opcode_0xd0_relative_bne() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0xd0, 0xfb,  // BNE rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BNE

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}