use r_nes::cpu::{Cpu, Status};

#[test]
fn test_cld() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::DECIMAL);
    cpu.memory[0] = 0xD8;
    let result = cpu.clock();

    assert!(result.is_none());
    assert!(!cpu.status_register.contains(Status::DECIMAL));
}
