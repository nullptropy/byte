mod common;

#[test]
fn opcode_0x50_relative_bvc() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0x50, 0xfb,  // BVC rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BVC

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}