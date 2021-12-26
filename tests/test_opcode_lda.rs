mod common;

#[test]
fn opcode_0xa9_immediate_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xa5_zeropage_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xb5_zeropagex_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xad_absolute_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xbd_absolutex_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xb9_absolutey_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xa1_indirectx_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}

#[test]
fn opcode_0xb1_indirecty_lda() {
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}