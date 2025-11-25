use r_nes::cpu::{Cpu, Status};

#[test]
fn test_sei() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::INTERRUPT);
    cpu.memory[0] = 0x78;
    let result = cpu.clock();

    assert!(result.is_none());
    assert!(cpu.status_register.contains(Status::INTERRUPT));
}
