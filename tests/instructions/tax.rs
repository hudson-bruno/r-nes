use r_nes::cpu::{Cpu, Status};

#[test]
fn test_tax() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x01;
    cpu.memory[0] = 0xAA;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.x_index_register, 0x01);
}

#[test]
fn test_tax_status_zero() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x00;
    cpu.memory[0] = 0xAA;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
    assert_eq!(cpu.x_index_register, 0x00);
}

#[test]
fn test_tax_status_negative() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0xFF;
    cpu.memory[0] = 0xAA;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(cpu.x_index_register, 0xFF);
}
