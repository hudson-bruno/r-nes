use r_nes::nes::Nes;

#[test]
fn test_txs() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0] = 0x9A;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.stack_pointer, 0x01);
}
