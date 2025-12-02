use r_nes::{cartridge::Cartridge, cpu::Status, nes::Nes};

#[test]
fn test_sed() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0xF8]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::DECIMAL));
}
