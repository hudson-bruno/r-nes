use r_nes::nes::Nes;

#[test]
fn test_nop() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0] = 0xEA;

    let result = nes.clock();

    assert!(result.is_none());
}
