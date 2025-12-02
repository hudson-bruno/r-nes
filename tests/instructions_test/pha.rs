use r_nes::{cartridge::Cartridge, cpu::memory::stack::Stack, nes::Nes};

#[test]
fn test_pha_implicit() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x48]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.a_register = 0xFF;

    let result = nes.step();
    let a_register_from_pha = nes.cpu.stack_pop(&mut nes.bus);

    assert!(result.is_none());
    assert_eq!(a_register_from_pha, 0xFF);
}
