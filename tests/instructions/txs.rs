use r_nes::{cartridge::Cartridge, nes::Nes};

#[test]
fn test_txs() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x9A]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.stack_pointer, 0x01);
}
