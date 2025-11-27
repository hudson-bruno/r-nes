use r_nes::nes::Nes;

#[test]
fn test_jmp_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x4C, 0xFE, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x07FE);
}

#[test]
fn test_jmp_indirect() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x6C, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FE..=0x07FF].copy_from_slice(&[0x03, 0x00]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x0003);
}
