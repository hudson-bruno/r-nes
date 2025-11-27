use r_nes::{cartridge::Cartridge, cpu::Status, nes::Nes};

#[test]
fn test_iny() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0xC8]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.y_index_register = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x02);
}

#[test]
fn test_iny_status_zero() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0xC8]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.y_index_register = 0xFF;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
    assert_eq!(nes.cpu.y_index_register, 0x00);
}

#[test]
fn test_iny_status_negative() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0xC8]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.y_index_register = 0xFE;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.y_index_register, 0xFF);
}
