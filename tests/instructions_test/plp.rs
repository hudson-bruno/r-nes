use r_nes::{
    cartridge::Cartridge,
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_plp() {
    let mut status_in_stack = Status::all();
    status_in_stack.remove(Status::BREAK | Status::UNUSED);

    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x28]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.stack_push(&mut nes.bus, status_in_stack.bits());

    let result = nes.step();

    let mut expected_status = Status::all();
    expected_status.remove(Status::BREAK);

    assert!(result.is_none());
    assert_eq!(nes.cpu.status_register, expected_status);
}
