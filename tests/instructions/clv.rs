use r_nes::cpu::{Cpu, Status};

#[test]
fn test_clv() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::OVERFLOW);
    cpu.memory[0] = 0xB8;

    let result = cpu.clock();

    assert!(result.is_none());
    assert!(!cpu.status_register.contains(Status::OVERFLOW));
}
