use r_nes::cpu::{Cpu, Status, memory::stack::Stack};

#[test]
fn test_pla() {
    let mut cpu = Cpu::new();
    cpu.memory[0] = 0x68;
    cpu.stack_push(0xFF);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.a_register, 0xFF);
}

#[test]
fn test_pla_status_zero() {
    let mut cpu = Cpu::new();
    cpu.memory[0] = 0x68;
    cpu.stack_push(0x00);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::ZERO));
    assert_eq!(cpu.a_register, 0x00);
}

#[test]
fn test_pla_status_negative() {
    let mut cpu = Cpu::new();
    cpu.memory[0] = 0x68;
    cpu.stack_push(0x80);

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(cpu.a_register, 0x80);
}
