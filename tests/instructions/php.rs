use r_nes::cpu::{Cpu, ExitStatus, Status, memory::stack::Stack};

#[test]
fn test_php_implicit() {
    let mut cpu = Cpu::new();
    cpu.status_register = Status::all();
    cpu.status_register.remove(Status::BREAK);
    cpu.memory[0] = 0x08;

    let result = cpu.run();
    let status_from_php = cpu.stack_pop();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(Status::all().bits(), status_from_php);
}
