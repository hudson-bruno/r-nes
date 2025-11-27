use r_nes::nes::Nes;

#[test]
fn test_bvc() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x50, 0x7F]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x81);
}
