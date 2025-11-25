use r_nes::cpu::{Cpu, Status};

#[test]
fn test_cmp_immediate_status_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.memory[0..2].copy_from_slice(&[0xC9, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_immediate_status_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0xC9, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_immediate_status_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0..2].copy_from_slice(&[0xC9, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_zero_page_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.memory[0..4].copy_from_slice(&[0xC5, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_zero_page_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.memory[0..4].copy_from_slice(&[0xC5, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_zero_page_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0..4].copy_from_slice(&[0xC5, 0x03, 0x00, 0x01]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_zero_page_x_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0xD5, 0xFE]);
    cpu.memory[0xFF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_zero_page_x_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0xD5, 0xFE]);
    cpu.memory[0xFF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_zero_page_x_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x01;
    cpu.memory[0..2].copy_from_slice(&[0xD5, 0xFE]);
    cpu.memory[0xFF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_absolute_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.memory[0..3].copy_from_slice(&[0xCD, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_absolute_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xCD, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_absolute_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0..3].copy_from_slice(&[0xCD, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_absolute_x_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xDD, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_absolute_x_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xDD, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_absolute_x_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xDD, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_absolute_y_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.y_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xD9, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_absolute_y_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.y_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xD9, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_absolute_y_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.y_index_register = 0x01;
    cpu.memory[0..3].copy_from_slice(&[0xD9, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_indirect_x_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.x_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0xC1, 0x02, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_indirect_x_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.x_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0xC1, 0x02, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_indirect_x_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.x_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0xC1, 0x02, 0x00, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}

#[test]
fn test_cmp_indirect_y_carry() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x02;
    cpu.y_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0xD1, 0x03, 0x00, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::CARRY));
}

#[test]
fn test_cmp_indirect_y_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.y_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0xD1, 0x03, 0x00, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
}

#[test]
fn test_cmp_indirect_y_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.y_index_register = 0x01;
    cpu.memory[0..5].copy_from_slice(&[0xD1, 0x03, 0x00, 0xFE, 0x07]);
    cpu.memory[0x07FF] = 0x01;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
}
