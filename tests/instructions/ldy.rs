use r_nes::cpu::Cpu;

#[test]
fn test_ldy_immediate() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0xA0, 0x01, 0x00]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0xA4, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xB4, 0xFE, 0x00]);
    cpu.memory[0xFF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x04;
    cpu.memory[0..4].copy_from_slice(&[0xB4, 0xFF, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0xAC, 0xFF, 0x07, 0x00]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..4].copy_from_slice(&[0xBC, 0xFE, 0x07, 0x00]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.y_index_register, 0x01);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ldy_absolute_x_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.x_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0xBC, 0xFF, 0x07, 0x00, 0x01]);
//
//     let result = cpu.clock();
//
//     assert!(result.is_none());
//     assert_eq!(cpu.a_register, 0x01);
// }
