use r_nes::cpu::Cpu;

#[test]
fn test_dec_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0xC6, 0x03, 0x00, 0x02]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x03], 0x01);
}

#[test]
fn test_dec_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xD6, 0xFE, 0x00]);
    cpu.memory[0xFF] = 0x02;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0xFF], 0x01);
}

#[test]
fn test_dec_zero_page_x_overflow() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x04;
    cpu.memory[0..4].copy_from_slice(&[0xD6, 0xFF, 0x00, 0x02]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x03], 0x01);
}

#[test]
fn test_dec_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0xCE, 0xFF, 0x07, 0x00]);
    cpu.memory[0x07FF] = 0x02;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0x01);
}

#[test]
fn test_dec_absolute_x() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0..4].copy_from_slice(&[0xDE, 0xFE, 0x07, 0x00]);
    cpu.memory[0x07FF] = 0x02;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.memory[0x07FF], 0x01);
}
