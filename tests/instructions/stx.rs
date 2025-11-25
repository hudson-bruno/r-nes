use r_nes::cpu::Cpu;

#[test]
fn test_stx_zero_page() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0xFF;
    cpu.memory[0..2].copy_from_slice(&[0x86, 0x03]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x03], 0xFF);
}

#[test]
fn test_stx_zero_page_y() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0xFF;
    cpu.y_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0x96, 0xFE]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0xFF], 0xFF);
}

#[test]
fn test_stx_zero_page_y_overflow() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0xFF;
    cpu.y_index_register = 0x04;
    cpu.memory[0..2].copy_from_slice(&[0x96, 0xFF]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x03], 0xFF);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0xFF;
    cpu.memory[0..3].copy_from_slice(&[0x8E, 0xFF, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0xFF);
}
