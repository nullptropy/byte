mod common;

#[test]
fn opcode_0x70_relative_bvs() {
    let mut cpu = common::init_cpu();

    cpu.load(&[
        0x70, 0xfb,  // BVS rel(-5)
        0x00         // BRK
    ], 0x8000);

    cpu.reg.p.insert(common::cpu::Flags::OVERFLOW);
    cpu.reg.pc = 0x8000;
    cpu.step(); // only execute BVS

    assert_eq!(cpu.reg.pc, 0x7ffc); // 0x8000 + 1 - 5
}