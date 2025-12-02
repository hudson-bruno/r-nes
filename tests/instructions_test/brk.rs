use r_nes::{
    cartridge::Cartridge,
    cpu::{ExitStatus, Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_brk() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x07FD..=0x07FD].copy_from_slice(&[0x00]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0xFD, 0x87]);
    cartridge.program_memory[0x7FFE..=0x7FFF].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    let status_before_brk = nes.cpu.status_register;

    let result = nes.step();

    let status_from_brk = nes.cpu.stack_pop(&mut nes.bus);
    let error_program_counter = nes.cpu.stack_pop_address(&mut nes.bus);

    assert_eq!(result, Some(ExitStatus::Brk));
    assert_eq!(nes.cpu.program_counter, 0x8000);
    assert_eq!(error_program_counter, 0x87FD + 2);
    assert_eq!(
        status_before_brk.union(Status::BREAK).bits(),
        status_from_brk
    );
}
