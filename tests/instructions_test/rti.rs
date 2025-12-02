use r_nes::{
    cartridge::Cartridge,
    cpu::{memory::stack::Stack, Status},
    nes::Nes,
};

#[test]
fn test_rti() {
    let mut status_in_stack = Status::all();
    status_in_stack.remove(Status::BREAK | Status::UNUSED);

    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x0000..=0x0000].copy_from_slice(&[0x40]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.stack_push_address(&mut nes.bus, 0x87FF);
    nes.cpu.stack_push(&mut nes.bus, status_in_stack.bits());

    let result = nes.clock();

    let mut expected_status = Status::all();
    expected_status.remove(Status::BREAK);

    assert!(result.is_none());
    assert_eq!(nes.cpu.status_register, expected_status);
    assert_eq!(nes.cpu.program_counter, 0x87FF);
}
