use r_nes::{cartridge::Cartridge, cpu::memory::stack::Stack, nes::Nes};

#[test]
fn test_rts() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x60]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.stack_push_address(&mut nes.bus, 0x87FF);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x8800);
}
