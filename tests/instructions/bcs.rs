use r_nes::cpu::{Cpu, Status};

#[test]
fn test_bcs() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::CARRY);
    cpu.memory[0..2].copy_from_slice(&[0xB0, 0x7F]);
    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.program_counter, 0x81);
}
