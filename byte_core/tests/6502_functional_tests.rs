#![cfg_attr(rustfmt, rustfmt_skip)]

mod common;

#[test]
fn test_6502_functional_tests() {
    let mut cpu = common::init_cpu();

    cpu.reg.pc = 0x0400;
    cpu.load(
        include_bytes!("6502_functional_tests.bin"), 0x0000);

    let mut pc = [0xdead, 0xbeef];
    let mut ip = 0;

    loop {
        pc[ip % 2] = cpu.reg.pc;
        ip += 1;

        if cpu.reg.pc == 0x3469 { break; }
        if      pc[0] == pc[1]  { panic!("test failed: {:#X?}", cpu.reg) }

        cpu.step().unwrap();
    }
}
