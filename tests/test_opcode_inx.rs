mod common;

#[test]
fn opcode_0xe8_implied_inx() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}