use r_nes::{cartridge::Cartridge, nes::Nes};

#[test]
fn test_ora_immediate() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x09, 0xBA]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x05, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0x0003] = 0xBA;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x15, 0xFE]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x00FF] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x15, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x0D, 0xFF, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x1D, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x1D, 0xFF, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x05;
    nes.bus.cpu_memory[0x0004] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x19, 0xFE, 0x07]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute_y_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x19, 0xFF, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x05;
    nes.bus.cpu_memory[0x0004] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_x() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x01, 0x02]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_x_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x01, 0xFF]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_y() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x11, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_y_overflow() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0x11, 0x03]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x06;
    nes.bus.cpu_memory[0x0003..=0x0004].copy_from_slice(&[0xFF, 0xFF]);
    nes.bus.cpu_memory[0x0005] = 0xBA;
    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}
