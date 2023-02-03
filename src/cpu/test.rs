use super::*;

/* Few more Lda test cases */
#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}

/* Tax test cases */

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10)
}

#[test]
fn test_tax_status_zero() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_tax_status_negative() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xFF, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 0xFF);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0b1000_0000);
}

/* Tay test cases */

#[test]
fn test_0xaa_tay_move_a_to_y() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x0A, 0xa8, 0x00]);

    assert_eq!(cpu.register_y, 10)
}

#[test]
fn test_tay_status_zero() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0xa8, 0x00]);

    assert_eq!(cpu.register_y, 0);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_tay_status_negative() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xFF, 0xa8, 0x00]);

    assert_eq!(cpu.register_y, 0xFF);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0b1000_0000);
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

/* Inx test cases */

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}

/* Lda test cases */

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_b5() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xb5, 0x0f, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_ad() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_bd() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xbd, 0xff, 0x0f, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_b9() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xb9, 0xff, 0x0f, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_a1() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x10, 0xCCCC);
    cpu.mem_write_u16(0xCCCC, 0x0a);

    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa1, 0x0f, 0x00]);

    assert_eq!(cpu.register_a, 0x0a);
}

#[test]
fn test_lda_b1() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x10, 0xCCCB);
    cpu.mem_write_u16(0xCCCC, 0x0a);

    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xb1, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x0a);
}

/* Sta test cases */

#[test]
fn test_sta_85() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x0a, 0x85, 0xFF, 0x00]);

    let value = cpu.mem_read(0xFF);

    assert_eq!(value, 0x0a);
}

#[test]
fn test_sta_95() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0x0a, 0x95, 0xFE, 0x00]);

    let value = cpu.mem_read(0xFF);

    assert_eq!(value, 0x0a);
}

#[test]
fn test_sta_8d() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x0a, 0x8D, 0xFF, 0x10, 0x00]);

    let value = cpu.mem_read(0x10FF);

    assert_eq!(value, 0x0a);
}

#[test]
fn test_sta_9d() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0x0a, 0x9D, 0xFE, 0x10, 0x00]);

    let value = cpu.mem_read_u16(0x10FF);

    assert_eq!(value, 0x0a);
}

#[test]
fn test_sta_99() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xa9, 0x0a, 0x99, 0xFE, 0x10, 0x00]);

    let value = cpu.mem_read_u16(0x10FF);

    assert_eq!(value, 0x0a);
}

#[test]
fn test_sta_81() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x10, 0xCCCC);
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0x0a, 0x81, 0x0F, 0x00]);

    let value = cpu.mem_read_u16(0xCCCC);

    assert_eq!(value, 0x0a);
}

#[test]
fn test_sta_91() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x10, 0xCCCB);
    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xa9, 0x0a, 0x91, 0x10, 0x00]);

    let value = cpu.mem_read_u16(0xCCCC);

    assert_eq!(value, 0x0a);
}

/* And command test cases */

#[test]
fn test_and_29() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0b0000_0110, 0x29, 0b0000_0100, 0x00]);

    assert_eq!(cpu.register_a, 4);
}

#[test]
fn test_and_25() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0000_0100);
    cpu.load_and_run(vec![0xa9, 0b0000_0110, 0x25, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 4);
}

#[test]
fn test_and_35() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0000_0100);
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0b0000_0110, 0x35, 0x0f, 0x00]);

    assert_eq!(cpu.register_a, 4);
}

#[test]
fn test_and_2d() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0b0000_0100);
    cpu.load_and_run(vec![
        0xa9,
        0x01,
        0xaa,
        0xa9,
        0b0000_0110,
        0x2d,
        0x00,
        0x10,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 4);
}

#[test]
fn test_and_39() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0b0000_0100);
    cpu.load_and_run(vec![
        0xa9,
        0x01,
        0xa8,
        0xa9,
        0b0000_0110,
        0x2d,
        0x00,
        0x10,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 4);
}

#[test]
fn test_and_21() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x10, 0xCCCC);
    cpu.mem_write_u16(0xCCCC, 0b0000_0100);
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0b0000_0110, 0x21, 0x0f, 0x00]);

    assert_eq!(cpu.register_a, 4);
}

#[test]
fn test_and_31() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x10, 0xCCCB);
    cpu.mem_write_u16(0xCCCC, 0b0000_0100);
    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xa9, 0b0000_0110, 0x31, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 4);
}

/* ASl test cases */

#[test]
fn test_asl_0a() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xFF, 0x0A, 0x00]);

    assert_eq!(cpu.register_a, 0xFE);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_asl_06() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);
    cpu.load_and_run(vec![0x06, 0x10, 0x00]);

    let value = cpu.mem_read(0x10);

    assert_eq!(value, 0);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_asl_16() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x7f);
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0x16, 0x0f, 0x00]);

    let value = cpu.mem_read(0x10);

    assert_eq!(value, 0xfe);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_asl_0e() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0x01);
    cpu.load_and_run(vec![0x0e, 0x00, 0x10, 0x00]);

    let value = cpu.mem_read(0x1000);

    assert_eq!(value, 0x02);
}

#[test]
fn test_asl_1e() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0x01);
    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0x1e, 0xff, 0x0f, 0x00]);

    let value = cpu.mem_read(0x1000);

    assert_eq!(value, 0x02);
}

/* Branch command test cases */

#[test]
fn tets_branch_true() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;

    cpu.mem_write(cpu.program_counter, 0x81);

    cpu.branch(true);

    assert_eq!(cpu.program_counter, 0x8000 - 0x7F + 1);
}

#[test]
fn test_branch_false() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;

    cpu.mem_write(cpu.program_counter, 0x81);

    cpu.branch(false);

    assert_eq!(cpu.program_counter, 0x8000);
}

#[test]
fn test_bcc_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0x90, 0x0a, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bcc_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x70);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0x90, 0x09, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bcs_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0xb0, 0x09, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bcs_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x70);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0xb0, 0x09, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_beq_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0xf0, 0x09, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_beq_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x70);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0xf0, 0x09, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8005);
}

/* Bit test cases */

#[test]
fn test_bit_24_zero() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0101_0101);
    cpu.load_and_run(vec![0xa9, 0b1010_1010, 0x24, 0x10, 0x00]);

    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_bit_24_overflow() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0101_0101);
    cpu.load_and_run(vec![0xa9, 0b0111_1111, 0x24, 0x10, 0x00]);

    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_bit_24_negative() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b1101_0101);
    cpu.load_and_run(vec![0xa9, 0b1011_1111, 0x24, 0x10, 0x00]);

    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_bit_2c_zero() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0b0101_0101);
    cpu.load_and_run(vec![0xa9, 0b1010_1010, 0x2c, 0x00, 0x10, 0x00]);

    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_bit_2c_overflow() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0b0101_0101);
    cpu.load_and_run(vec![0xa9, 0b0111_1111, 0x2c, 0x00, 0x10, 0x00]);

    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_bit_2c_negative() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0b1101_0101);
    cpu.load_and_run(vec![0xa9, 0b1011_1111, 0x2c, 0x00, 0x10, 0x00]);

    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
}

/* Branch command test cases continued*/

#[test]
fn test_bmi_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x70);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0x30, 0x09, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bmi_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0x30, 0x09, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bne_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x70);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0xd0, 0x09, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bne_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0xd0, 0x09, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bpl_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x10);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0x10, 0x09, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bpl_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x70);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0x06, 0x10, 0x10, 0x09, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bvc_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0101_0101);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0xa9, 0b0011_1111, 0x24, 0x10, 0x50, 0x07, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bvc_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0101_0101);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0xa9, 0b0111_1111, 0x24, 0x10, 0x50, 0x07, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8007);
}

#[test]
fn test_bvs_true() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0101_0101);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0xa9, 0b0111_1111, 0x24, 0x10, 0x70, 0x07, 0x00]);

    assert_eq!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8010);
}

#[test]
fn test_bvs_false() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0b0101_0101);
    cpu.mem_write(0x800d, 0xa9);
    cpu.mem_write(0x800e, 0xff);
    cpu.mem_write_u16(0x800f, 0x00);
    cpu.load_and_run(vec![0xa9, 0b0011_1111, 0x24, 0x10, 0x70, 0x07, 0x00]);

    assert_ne!(cpu.register_a, 0xff);
    assert_eq!(cpu.program_counter, 0x8007);
}

/* Clear Commands test cases */

#[test]
fn test_clear_carry() {
    let mut cpu = CPU::new();
    cpu.set_carry_flag();

    assert!(cpu.status.contains(CpuFlags::CARRY));
    cpu.load(vec![0x18, 0x00]);
    cpu.register_a = 0;
    cpu.register_x = 0;
    cpu.register_y = 0;
    cpu.program_counter = cpu.mem_read_u16(0xFFFC);
    cpu.run();
    assert!(!cpu.status.contains(CpuFlags::CARRY));
}

#[test]
fn test_clear_decimal() {
    let mut cpu = CPU::new();
    cpu.set_decimal_mode();

    assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
    cpu.load(vec![0xD8, 0x00]);
    cpu.register_a = 0;
    cpu.register_x = 0;
    cpu.register_y = 0;
    cpu.program_counter = cpu.mem_read_u16(0xFFFC);
    cpu.run();
    assert!(!cpu.status.contains(CpuFlags::DECIMAL_MODE));
}

#[test]
fn test_clear_interrupt() {
    let mut cpu = CPU::new();
    cpu.set_interupt_disable();

    assert!(cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
    cpu.load(vec![0x58, 0x00]);
    cpu.register_a = 0;
    cpu.register_x = 0;
    cpu.register_y = 0;
    cpu.program_counter = cpu.mem_read_u16(0xFFFC);
    cpu.run();
    assert!(!cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_clear_overflow() {
    let mut cpu = CPU::new();
    cpu.set_overflow_flag();

    assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    cpu.load(vec![0xB8, 0x00]);
    cpu.register_a = 0;
    cpu.register_x = 0;
    cpu.register_y = 0;
    cpu.program_counter = cpu.mem_read_u16(0xFFFC);
    cpu.run();
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
}

/* LDX Test Cases */

#[test]
fn test_ldx_a2() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x10, 0x00]);

    assert_eq!(cpu.register_x, 0x10);
}

#[test]
fn test_ldx_a6() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa6, 0x10, 0x00]);

    assert_eq!(cpu.register_x, 0x55);
}

#[test]
fn test_ldx_b6() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xb6, 0x0f, 0x00]);

    assert_eq!(cpu.register_x, 0x55);
}

#[test]
fn test_ldx_ae() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xae, 0x00, 0x10, 0x00]);

    assert_eq!(cpu.register_x, 0x55);
}

#[test]
fn test_ldx_be() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xbe, 0xff, 0x0f, 0x00]);

    assert_eq!(cpu.register_x, 0x55);
}

/* LDY Test Cases */
#[test]
fn test_ldy_a0() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x10, 0x00]);

    assert_eq!(cpu.register_y, 0x10);
}

#[test]
fn test_ldy_a4() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa4, 0x10, 0x00]);

    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn test_ldy_b4() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xb4, 0x0f, 0x00]);

    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn test_ldy_ac() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xac, 0x00, 0x10, 0x00]);

    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn test_ldy_bc() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0x1000, 0x55);

    cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xbc, 0xff, 0x0f, 0x00]);

    assert_eq!(cpu.register_y, 0x55);
}

/* Compare function validation */
#[test]
fn test_compare_carry() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x10;
    cpu.mem_write(0x10, 0x10);

    cpu.compare(&AddressingMode::Immediate, 0x11);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_compare_zero() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x10;
    cpu.mem_write(0x10, 0x10);

    cpu.compare(&AddressingMode::Immediate, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_compare_negative() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x10;
    cpu.mem_write(0x10, 0x10);

    cpu.compare(&AddressingMode::Immediate, 0x09);

    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
}

/* Cmp command tests */
#[test]
fn test_cmp_c9() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_c5() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xc5, 0xFF, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_d5() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xa2, 0x01, 0xd5, 0xFE, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_cd() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF00, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xcd, 0x00, 0xFF, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_dd() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF00, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xa2, 0x01, 0xdd, 0xFF, 0xFE, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_d9() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF00, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xa0, 0x01, 0xd9, 0xFF, 0xFE, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_c1() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0xFE, 0xCCCC);
    cpu.mem_write(0xCCCC, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xa2, 0x01, 0xc1, 0xFD, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cmp_d1() {
    let mut cpu = CPU::new();
    cpu.mem_write_u16(0xFE, 0xCCCB);
    cpu.mem_write(0xCCCC, 0x10);
    cpu.load_and_run(vec![0xa9, 0x10, 0xa0, 0x01, 0xd1, 0xFE, 0x00]);

    assert_eq!(cpu.register_a, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

/* CPX command tests */
#[test]
fn test_cpx_e0() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x10, 0xe0, 0x10, 0x00]);

    assert_eq!(cpu.register_x, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cpx_e4() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF, 0x10);
    cpu.load_and_run(vec![0xa2, 0x10, 0xe4, 0xFF, 0x00]);

    assert_eq!(cpu.register_x, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cpx_ec() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF00, 0x10);
    cpu.load_and_run(vec![0xa2, 0x10, 0xec, 0x00, 0xFF, 0x00]);

    assert_eq!(cpu.register_x, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

/* CPY command tests */
#[test]
fn test_cpy_c0() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x10, 0xc0, 0x10, 0x00]);

    assert_eq!(cpu.register_y, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cpy_c4() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF, 0x10);
    cpu.load_and_run(vec![0xa0, 0x10, 0xc4, 0xFF, 0x00]);

    assert_eq!(cpu.register_y, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

#[test]
fn test_cpy_cc() {
    let mut cpu = CPU::new();
    cpu.mem_write(0xFF00, 0x10);
    cpu.load_and_run(vec![0xa0, 0x10, 0xcc, 0x00, 0xFF, 0x00]);

    assert_eq!(cpu.register_y, 0x10);

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
}

/* DEC command tests */

#[test]
fn test_dec_c6_overflow() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x00);
    cpu.load_and_run(vec![0xc6, 0x10, 0xc6, 0x10, 0x00]);

    let data = cpu.mem_read(0x10);

    assert_eq!(data, 0xFE);
}

#[test]
fn test_dec_c6_flags_neg() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0xc6, 0x10, 0x00]);

    let data = cpu.mem_read(0x10);

    assert_eq!(data, 0xFE);

    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_dec_c6_flags_zero() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x01);
    cpu.load_and_run(vec![0xc6, 0x10, 0x00]);

    let data = cpu.mem_read(0x10);

    assert_eq!(data, 0x00);

    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_dec_d6() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0xFF);
    cpu.load_and_run(vec![0xa2, 0x01, 0xd6, 0x0F, 0x00]);

    let data = cpu.mem_read(0x10);

    assert_eq!(data, 0xFE);
}

#[test]
fn test_dec_ce() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0xFF);
    cpu.load_and_run(vec![0xce, 0x00, 0x10, 0x00]);

    let data = cpu.mem_read(0x1000);

    assert_eq!(data, 0xFE);
}

#[test]
fn test_dec_de() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x1000, 0xFF);
    cpu.load_and_run(vec![0xa2, 0x01, 0xde, 0xFF, 0x0F, 0x00]);

    let data = cpu.mem_read(0x1000);

    assert_eq!(data, 0xFE);
}

/* DEX command tests */
#[test]
fn test_dex_ca_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0xca, 0xca, 0x00]);

    assert_eq!(cpu.register_x, 0xFE);
}

#[test]
fn test_dex_ca_flags_neg() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0xFF, 0xca, 0x00]);

    assert_eq!(cpu.register_x, 0xFE);

    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_dex_ca_flags_zero() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x01, 0xca, 0x00]);

    assert_eq!(cpu.register_x, 0x00);

    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

/* DEY command tests */
#[test]
fn test_dey_88_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x00, 0x88, 0x88, 0x00]);

    assert_eq!(cpu.register_y, 0xFE);
}

#[test]
fn test_dey_88_flags_neg() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0xFF, 0x88, 0x00]);

    assert_eq!(cpu.register_y, 0xFE);

    assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(!cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_dey_88_flags_zero() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x01, 0x88, 0x00]);

    assert_eq!(cpu.register_y, 0x00);

    assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

//    #[test]
//    fn test_lda_b9(){
//         let mut cpu =  CPU::new();
//         cpu.mem_write_u16(0x1000, 0x55)
//    }
