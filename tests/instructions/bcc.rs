use r_nes::nes::Nes;

#[test]
fn test_bcc() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x90, 0x7F]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x81);
}
