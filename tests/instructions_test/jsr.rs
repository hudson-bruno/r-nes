use r_nes::{cartridge::Cartridge, cpu::memory::stack::Stack, nes::Nes};

#[test]
fn test_jsr() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0002].copy_from_slice(&[0x20, 0xFF, 0x87]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    let result = nes.step();
    let pc_before_jsr = nes.cpu.stack_pop_address(&mut nes.bus);

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x87FF);
    assert_eq!(pc_before_jsr, 0x8003);
}
