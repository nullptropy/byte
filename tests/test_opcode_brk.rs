mod common;

#[test]
fn opcode_0x00_implied_brk() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}