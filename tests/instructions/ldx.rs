use r_nes::cpu::Cpu;

#[test]
fn test_ldx_immediate() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0xA2, 0x01, 0x00]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

#[test]
fn test_ldx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0xA6, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

#[test]
fn test_ldx_zero_page_y() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xB6, 0xFE, 0x00]);
    cpu.memory[0xFF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

#[test]
fn test_ldx_zero_page_y_overflow() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x04;
    cpu.memory[0..4].copy_from_slice(&[0xB6, 0xFF, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

#[test]
fn test_ldx_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0xAE, 0xFF, 0x07, 0x00]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

#[test]
fn test_ldx_absolute_y() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x01;
    cpu.memory[0..4].copy_from_slice(&[0xBE, 0xFE, 0x07, 0x00]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ldx_absolute_y_overflow() {
//     let mut cpu = Cpu::new();
//     cpu.y_index_register = 0x05;
//     cpu.memory[0..5].copy_from_slice(&[0xBE, 0xFF, 0x07, 0x00, 0x01]);
//
//     let result = cpu.clock();
//
//     assert!(result.is_none());
//     assert_eq!(cpu.x_index_register, 0x01);
// }
