use r_nes::cpu::Cpu;

#[test]
fn test_jmp_absolute() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0x4C, 0xFE, 0x07]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.program_counter, 0x07FE);
}

#[test]
fn test_jmp_indirect() {
    let mut cpu = Cpu::new();
    cpu.memory[0..3].copy_from_slice(&[0x6C, 0xFE, 0x07]);
    cpu.memory[0x07FE..=0x07FF].copy_from_slice(&[0x03, 0x00]);

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.program_counter, 0x0003);
}
