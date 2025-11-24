use r_nes::cpu::{Cpu, ExitStatus};

#[test]
fn test_asl_accumulator() {
    let mut cpu = Cpu::new();
    cpu.a_register = 0x80;
    cpu.memory[0] = 0x0A;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.a_register, 0x00);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = Cpu::new();
    cpu.memory[0..4].copy_from_slice(&[0x06, 0x03, 0x00, 0x80]);
    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x03], 0x00);
}

#[test]
fn test_asl_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0x0E, 0xFF, 0x07]);
    cpu.memory[0x07FF] = 0x80;

    let result = cpu.run();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.memory[0x07FF], 0x00);
}
