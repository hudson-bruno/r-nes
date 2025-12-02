use r_nes::{cartridge::Cartridge, nes::Nes};

#[test]
fn test_ldy_immediate() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xA0, 0x01]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xA4, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.bus.cpu_memory[0x0003] = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xB4, 0xFE]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x00FF] = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_zero_page_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xB4, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003] = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0xAC, 0xFF, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0xBC, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_ldy_absolute_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0xBC, 0xFF, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.x_index_register = 0x05;
    nes.bus.cpu_memory[0x0004] = 0x01;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}
