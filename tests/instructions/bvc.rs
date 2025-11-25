use r_nes::cpu::Cpu;

#[test]
fn test_bvc() {
    let mut cpu = Cpu::new();
    cpu.memory[0..2].copy_from_slice(&[0x50, 0x7F]);
    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.program_counter, 0x81);
}
