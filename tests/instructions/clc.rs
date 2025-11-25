use r_nes::cpu::{Cpu, Status};

#[test]
fn test_clc() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::CARRY);
    cpu.memory[0] = 0x18;
    let result = cpu.clock();

    assert!(result.is_none());
    assert!(!cpu.status_register.contains(Status::CARRY));
}
