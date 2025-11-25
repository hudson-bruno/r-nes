use r_nes::cpu::{Cpu, Status};

#[test]
fn test_bvs() {
    let mut cpu = Cpu::new();
    cpu.status_register.insert(Status::OVERFLOW);
    cpu.memory[0..2].copy_from_slice(&[0x70, 0x7F]);
    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.program_counter, 0x81);
}
