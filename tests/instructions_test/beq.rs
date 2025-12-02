use r_nes::{cartridge::Cartridge, cpu::Status, nes::Nes};

#[test]
fn test_beq() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0001].copy_from_slice(&[0xF0, 0x7F]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.status_register.insert(Status::ZERO);

    let result = nes.step();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x8002 + 0x7F);
}
