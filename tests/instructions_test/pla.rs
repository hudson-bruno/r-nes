use r_nes::{
    cartridge::Cartridge,
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_pla() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x68]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.stack_push(&mut nes.bus, 0xFF);

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xFF);
}

#[test]
fn test_pla_status_zero() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x68]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.stack_push(&mut nes.bus, 0x00);

    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
    assert_eq!(nes.cpu.a_register, 0x00);
}

#[test]
fn test_pla_status_negative() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x68]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.stack_push(&mut nes.bus, 0x80);

    let result = nes.step();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.a_register, 0x80);
}
