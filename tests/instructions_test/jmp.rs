use r_nes::{cartridge::Cartridge, nes::Nes};

#[test]
fn test_jmp_absolute() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x4C, 0xFE, 0x87]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x87FE);
}

#[test]
fn test_jmp_indirect() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x6C, 0xFE, 0x87]);
    cartridge.program_memory[0x07FE..=0x07FF].copy_from_slice(&[0x03, 0x80]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x8003);
}
