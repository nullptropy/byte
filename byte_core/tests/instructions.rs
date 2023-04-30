#![cfg_attr(rustfmt, rustfmt_skip)]

mod common;

use common::cpu::Flags;
use common::execute_nsteps;

#[test]
fn opcode_0x29_immediate_and() {
    // AND #$80
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0b1111_1111, &[0x29, 0x80], 0x8000, 1);

    assert!(cpu.reg.a == 0x80);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0x0a_accumulator_asl() {
    // ASL A
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0b1010_1010, &[0x0a, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0b0101_0100);
    assert!(cpu.reg.p.contains(Flags::CARRY));
    assert!(!cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0x06_zeropage_asl() {
    // ASL $aa
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write(0xaa, 0b1010_1010), &[0x06, 0xaa, 0x00], 0x8000, 1);

    assert_eq!(cpu.bus.read(0xaa), 0b0101_0100);
    assert!(cpu.reg.p.contains(Flags::CARRY));
    assert!(!cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0x24_zeropage_bit() {
    let cpu = execute_nsteps(|cpu| {
        cpu.reg.a = 0b1100_0000;
        cpu.bus.write(0x00aa, 0b1111_1111);
    },
    &[0x24, 0xaa], 0x8000, 1);

    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
    assert!(cpu.reg.p.contains(Flags::OVERFLOW));
    assert!(!cpu.reg.p.contains(Flags::ZERO));
}

#[test]
fn opcode_0x90_relative_bcc_1() {
    // BCC rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0x90, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0x90_relative_bcc_2() {
    // BCC rel(5)
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0x90, 0x05, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x8007); // 0x8000 + 5
}

#[test]
fn opcode_0xb0_relative_bcs() {
    // BCS rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.p.insert(Flags::CARRY), &[0xb0, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0xf0_relative_beq() {
    // BEQ rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.p.insert(Flags::ZERO), &[0xf0, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0x30_relative_bmi() {
    // BMI rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.p.insert(Flags::NEGATIVE), &[0x30, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0xd0_relative_bne() {
    // BNE rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0xd0, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0x10_relative_bpl() {
    // BPL rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0x10, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0x50_relative_bvc() {
    // BVC rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0x50, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0x70_relative_bvs() {
    // BVS rel(-5)
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.p.insert(Flags::OVERFLOW), &[0x70, 0xfb, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0x7ffd); // 0x8000 - 5
}

#[test]
fn opcode_0x18_implied_clc() {
    // CLC
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.set_flag(Flags::CARRY, true), &[0x18, 0x00], 0x8000, 1);

    assert!(!cpu.reg.p.contains(Flags::CARRY));
}

#[test]
fn opcode_0xd8_implied_cld() {
    // CLD
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.set_flag(Flags::DECIMAL, true), &[0xd8, 0x00], 0x8000, 1);

    assert!(!cpu.reg.p.contains(Flags::DECIMAL));
}

#[test]
fn opcode_0x58_implied_cli() {
    // CLI
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.set_flag(Flags::INTERRUPT, true), &[0x58, 0x00], 0x8000, 1);

    assert!(!cpu.reg.p.contains(Flags::INTERRUPT));
}

#[test]
fn opcode_0xb8_implied_clv() {
    // CLV
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.set_flag(Flags::OVERFLOW, false), &[0xb8], 0x8000, 1);

    assert!(!cpu.reg.p.contains(Flags::OVERFLOW));
}

#[test]
fn opcode_0xc9_immediate_cmp() {
    // CMP #$aa
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0xaa, &[0xc9, 0xaa, 0x00], 0x8000, 1);

    assert!(cpu.reg.p.contains(Flags::CARRY));
    assert!(cpu.reg.p.contains(Flags::ZERO));
}

#[test]
fn opcode_0xc5_zeropage_cmp() {
    // CMP $4a
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.a = 0xca;
            cpu.bus.write(0x4a, 0x4a);
        },
        &[0xc5, 0x4a, 0x00], 0x8000, 1);

    assert!(cpu.reg.p.contains(Flags::NEGATIVE))
}

#[test]
fn opcode_0xc6_zeropage_dec() {
    // DEC $aa
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write(0xaa, 0xff), &[0xc6, 0xaa, 0x00], 0x8000, 1);

    assert_eq!(cpu.bus.read(0x00aa), 0xfe);
}

#[test]
fn opcode_0xca_implied_dex() {
    // DEX
    // DEX
    // DEX
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.x = 0x3, &[0xca, 0xca, 0xca, 0x00], 0x8000, 3);

    assert_eq!(cpu.reg.x, 0x0);
    assert!(cpu.reg.p.contains(Flags::ZERO));
}

#[test]
fn opcode_0x88_implied_dey() {
    // DEY
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.y = 0xff, &[0x88, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.y, 0xfe);
}

#[test]
fn opcode_0x49_immediate_eor() {
    // CMP #$ff
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0b00111100, &[0x49, 0xff, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0b11000011);
}

#[test]
fn opcode_0xe6_zeropage_inc() {
    // INC $aa
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write(0xaa, 0xfe), &[0xe6, 0xaa, 0x00], 0x8000, 1);

    assert_eq!(cpu.bus.read(0x00aa), 0xff);
}

#[test]
fn opcode_0xe8_implied_inx() {
    // INX
    // INX
    // INX
    // INX
    // INX
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0x00], 0x8000, 5);

    assert_eq!(cpu.reg.x, 0x5);
}

#[test]
fn opcode_0xc8_implied_iny() {
    // INY
    // INY
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0xc8, 0xc8, 0x00], 0x8000, 2);

    assert_eq!(cpu.reg.y, 0x2);
}

#[test]
fn opcode_0x4c_absolute_jmp() {
    // JMP $dead ; absolute jump
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0x4c, 0xad, 0xde, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0xdead);
}

#[test]
fn opcode_0x6c_indirect_jmp() {
    // JMP $dead ; indirect jump
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write_u16(0xdead, 0xbeef), &[0x6c, 0xad, 0xde, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0xbeef);
}

#[test]
fn opcode_0x6c_indirect_jmp_bug() {
    // JMP $ccff ; indirect jump
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.bus.write(0xcc00, 0xde);
            cpu.bus.write_u16(0xccff, 0xdead);
        },
        &[0x6c, 0xff, 0xcc], 0x8000, 1);

    assert_eq!(cpu.reg.pc, 0xdead);
}

#[test]
fn opcode_0xa9_immediate_lda() {
    // LDA #$ff
    // BRK
    let cpu = execute_nsteps(
        |_| {}, &[0xa9, 0xff, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xa5_zeropage_lda() {
    // LDA $ee
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write(0xee, 0xff), &[0xa5, 0xee, 0x00], 0x8000, 1);
    
    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xb5_zeropagex_lda() {
    // LDA $eb,x
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.x = 0x3;
            cpu.bus.write(0xee, 0xff);
        },
        &[0xb5, 0xeb], 0x800, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xad_absolute_lda() {
    // LDA $faaf
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write(0xfaaf, 0xff), &[0xad, 0xaf, 0xfa, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xbd_absolutex_lda() {
    // LDA $faac,x
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.x = 0x3;
            cpu.bus.write(0xfaaf, 0xff);
        },
        &[0xbd, 0xac, 0xfa, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xb9_absolutey_lda() {
    // LDA $faac,y
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.y = 0x3;
            cpu.bus.write(0xfaaf, 0xff);
        },
        &[0xb9, 0xac, 0xfa, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xa1_indirectx_lda() {
    // LDA ($05,x)
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.x = 0x4;
            cpu.bus.write(0xfaaf, 0xff);
            cpu.bus.write_u16(0x09, 0xfaaf);
        },
        &[0xa1, 0x05, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xb1_indirecty_lda() {
    // LDA ($10),y
    // BRK
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.y = 0x4;
            cpu.bus.write(0xff04, 0xff);
            cpu.bus.write_u16(0x10, 0xff00);
        },
        &[0xb1, 0x10, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
    assert!(cpu.reg.p.contains(Flags::NEGATIVE));
}

#[test]
fn opcode_0xa2_immediate_ldx() {
    let cpu = execute_nsteps(
        |_| {}, &[0xa2, 0xff, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.x, 0xff);
}

#[test]
fn opcode_0xa0_immediate_ldy() {
    let cpu = execute_nsteps(
        |_| {}, &[0xa0, 0xff, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.y, 0xff);
}

#[test]
fn opcode_0x4a_accumulator_lsr() {
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0x81, &[0x4a, 0x00], 0x8000, 1);

    assert!(cpu.reg.a == 0x40);
    assert!(cpu.reg.p.contains(Flags::CARRY));
}

#[test]
fn opcode_0x46_zeropage_lsr() {
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write(0xde, 0xad), &[0x46, 0xde, 0x00], 0x8000, 1);

    assert!(cpu.bus.read(0xde) == 0xadu8.wrapping_shr(1));
    assert!(cpu.reg.p.contains(Flags::CARRY));
}

#[test]
fn opcode_0x09_immediate_ora() {
    let cpu = execute_nsteps(
        |_| {}, &[0x09, 0xff], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
}

#[test]
fn opcode_0x48_implied_pha() {
    let mut cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0xff, &[0x48, 0x00], 0x8000, 1);

    assert_eq!(cpu.stack_pull(), 0xff);
}

#[test]
fn opcode_0x08_implied_php() {
    let mut cpu = execute_nsteps(
        |cpu| cpu.reg.p.insert(Flags::CARRY), &[0x08, 0x00], 0x8000, 1);

    assert!(Flags::from_bits(cpu.stack_pull()).unwrap().contains(Flags::CARRY));
}

#[test]
fn opcode_0x68_implied_pla() {
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.sp = 0xff;
            cpu.stack_push(0xff);
        }, &[0x68, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0xff);
}

#[test]
fn opcode_0x28_implied_plp() {
    let cpu = execute_nsteps(|cpu| {
        cpu.reg.sp = 0xff;
        cpu.stack_push(0b1111_0111);
    }, &[0x28, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.p.bits(), 0b1111_0111);
}

#[test]
fn opcode_0x2a_accumulator_rol() {
    let cpu = execute_nsteps(|cpu| {
        cpu.set_flag(Flags::CARRY, true);
        cpu.reg.a = 0x88;
    },
    &[0x2a, 0x00], 0x8000, 1);

    assert!(cpu.reg.a == 0x11);
    assert!(cpu.reg.p.contains(Flags::CARRY));
}

#[test]
fn opcode_0x6a_accumulator_ror() {
    let cpu = execute_nsteps(
        |cpu| {
            cpu.set_flag(Flags::CARRY, true);
            cpu.reg.a = 0x81;
        },
        &[0x6a, 0x00], 0x8000, 1);

    assert!(cpu.reg.a == 0xc0);
    assert!(cpu.reg.p.contains(Flags::CARRY));
}

#[test]
fn opcode_0x40_implied_rti() {
    // BRK      ; 0xfffe: 0x8003
    // NOP
    // NOP
    // LDA #$ff ; 0x8003
    // RTI
    let cpu = execute_nsteps(
        |cpu| cpu.bus.write_u16(0xfffe, 0x8003),
        &[0x00, 0xea, 0xea, 0xa9, 0xff, 0x40], 0x8000, 3);

    assert_eq!(cpu.reg.pc, 0x8002);
}

#[test]
fn opcode_0x60_implied_rts() {
    //      [ 0x80 ]: ff \
    //      [ 0x01 ]: fe  => stack state
    //      [ 0x00 ]: fd /
    //
    // RTS
    // NOP
    let cpu = execute_nsteps(
        |cpu| cpu.stack_push_u16(0x8001), &[0x60, 0xea], 0x8000, 1);
    assert_eq!(cpu.reg.pc, 0x8002);
}

#[test]
fn opcode_0x20_absolute_jsr() {
    // JSR $80003
    // NOP       ; 0x8003
    let cpu = execute_nsteps(
        |cpu| {
            cpu.reg.sp = 0xff;  // initialize the stack pointer
            cpu.bus.write_u16(0x0001, 0x8003);
        }, &[0x20, 0x03, 0x80, 0xea], 0x8000, 1);

    assert_eq!(cpu.reg.sp, 0x00fd);
    assert_eq!(cpu.reg.pc, 0x8003);
}

#[test]
fn opcode_0x85_zeropage_sta() {
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0xad, &[0x85, 0xde, 0x00], 0x8000, 1);

    assert_eq!(cpu.bus.read(0xde), 0xad);
}

#[test]
fn opcode_0x86_zeropage_stx() {
    let cpu = execute_nsteps(
        |cpu| cpu.reg.x = 0xef, &[0x86, 0xbe, 0x00], 0x8000, 1);

    assert_eq!(cpu.bus.read(0xbe), 0xef);
}

#[test]
fn opcode_0x84_zeropage_sty() {
    let cpu = execute_nsteps(
        |cpu| cpu.reg.y = 0xed, &[0x84, 0xfe, 0x00], 0x8000, 1);

    assert_eq!(cpu.bus.read(0xfe), 0xed);
}

#[test]
fn opcode_0xaa_implied_tax() {
    // TAX
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0x5, &[0xaa, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.x, 0x5);
}

#[test]
fn opcode_0xa8_implied_tay() {
    // TAY
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.a = 0x5, &[0xa8, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.y, 5);
}

#[test]
fn opcode_0x8a_implied_txa() {
    // TXA
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.x = 0x5, &[0x8a, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0x5);
}

#[test]
fn opcode_0x98_implied_tya() {
    // TYA
    // BRK
    let cpu = execute_nsteps(
        |cpu| cpu.reg.y = 0x5, &[0x98, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.a, 0x5);
}

#[test]
fn opcode_0xba_implied_tsx() {
    let cpu = execute_nsteps(
        |cpu| cpu.reg.sp = 0xde, &[0xba, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.x, 0xde);
}

#[test]
fn opcode_0x9a_implied_txs() {
    let cpu = execute_nsteps(
        |cpu| cpu.reg.x = 0xad, &[0x9a, 0x00], 0x8000, 1);

    assert_eq!(cpu.reg.sp, 0xad);
}
