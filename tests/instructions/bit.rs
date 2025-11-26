use r_nes::cpu::{Cpu, Status};

#[test]
fn test_bit_zero_page_status_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0b0011_1111;
    cpu.memory[0..4].copy_from_slice(&[0x24, 0x03, 0x00, 0b0000_0000]);
    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_bit_zero_page_status_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0b0011_1111;
    cpu.memory[0..4].copy_from_slice(&[0x24, 0x03, 0x00, 0b0100_0001]);
    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::OVERFLOW));
}

#[test]
fn test_bit_zero_page_status_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0b0011_1111;
    cpu.memory[0..4].copy_from_slice(&[0x24, 0x03, 0x00, 0b1000_0001]);
    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_bit_absolute_status_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0b0011_1111;
    cpu.memory[0..3].copy_from_slice(&[0x2C, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0b0000_0000;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_bit_absolute_status_overflow() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0b0011_1111;
    cpu.memory[0..3].copy_from_slice(&[0x2C, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0b0100_0001;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::OVERFLOW));
}

#[test]
fn test_bit_absolute_status_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0b0011_1111;
    cpu.memory[0..3].copy_from_slice(&[0x2C, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0b1000_0001;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}
