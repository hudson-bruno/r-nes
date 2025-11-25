use r_nes::cpu::{Cpu, Status};

#[test]
fn test_cpy_immediate_status_carry() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x02;
    cpu.memory[0..2].copy_from_slice(&[0xC0, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cpy_immediate_status_zero() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0xC0, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cpy_immediate_status_negative() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0xFF;
    cpu.memory[0..2].copy_from_slice(&[0xC0, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cpy_zero_page_carry() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x02;
    cpu.memory[0..4].copy_from_slice(&[0xC4, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cpy_zero_page_zero() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x01;
    cpu.memory[0..4].copy_from_slice(&[0xC4, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cpy_zero_page_negative() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0xFF;
    cpu.memory[0..4].copy_from_slice(&[0xC4, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cpy_absolute_carry() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x02;
    cpu.memory[0..3].copy_from_slice(&[0xCC, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cpy_absolute_zero() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xCC, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cpy_absolute_negative() {
    let mut cpu = Cpu::new();
    cpu.y_index_register = 0xFF;
    cpu.memory[0..3].copy_from_slice(&[0xCC, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}
