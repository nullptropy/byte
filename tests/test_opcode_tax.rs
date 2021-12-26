mod common;

#[test]
fn opcode_0xaa_implied_tax() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}