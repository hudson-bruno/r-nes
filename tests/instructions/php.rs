use r_nes::{
    cartridge::Cartridge,
    cpu::{memory::stack::Stack, Status},
    nes::Nes,
};

#[test]
fn test_php_implicit() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x08]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.status_register = Status::all();
    nes.cpu.status_register.remove(Status::BREAK);

    let result = nes.clock();
    let status_from_php = nes.cpu.stack_pop(&mut nes.bus);

    assert!(result.is_none());
    assert_eq!(Status::all().bits(), status_from_php);
}
