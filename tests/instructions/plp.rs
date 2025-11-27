use r_nes::{
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_plp() {
    let mut status_in_stack = Status::all();
    status_in_stack.remove(Status::BREAK | Status::UNUSED);

    let mut nes = Nes::new();
    nes.bus.cpu_memory[0] = 0x28;
    nes.cpu.stack_push(&mut nes.bus, status_in_stack.bits());

    let result = nes.clock();

    let mut expected_status = Status::all();
    expected_status.remove(Status::BREAK);

    assert!(result.is_none());
    assert_eq!(nes.cpu.status_register, expected_status);
}
