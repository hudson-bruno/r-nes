use r_nes::cpu::{Cpu, ExitStatus};

#[test]
fn test_asl_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0x06, 0x03, 0x00, 0x80]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x03], 0x00);
}
