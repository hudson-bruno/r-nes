use r_nes::{cartridge::Cartridge, nes::Nes};

#[test]
fn test_stx_zero_page() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x86, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0xFF;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x0003], 0xFF);
}

#[test]
fn test_stx_zero_page_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x96, 0xFE]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0xFF;
    nes.cpu.y_index_register = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x00FF], 0xFF);
}

#[test]
fn test_stx_zero_page_y_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x96, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0xFF;
    nes.cpu.y_index_register = 0x04;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x0003], 0xFF);
}

#[test]
fn test_stx_absolute() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x8E, 0xFF, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0xFF;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}
