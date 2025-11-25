use r_nes::cpu::{Cpu, Status, memory::stack::Stack};

#[test]
fn test_plp() {
    let mut status_in_stack = Status::all();
    status_in_stack.remove(Status::BREAK | Status::UNUSED);

    let mut cpu = Cpu::new();
    cpu.memory[0] = 0x28;
    cpu.stack_push(status_in_stack.bits());

    let result = cpu.clock();

    let mut expected_status = Status::all();
    expected_status.remove(Status::BREAK);

    assert!(result.is_none());
    assert_eq!(cpu.status_register, expected_status);
}
