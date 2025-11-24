use r_nes::cpu::{Cpu, ExitStatus, Status, memory::stack::Stack};

#[test]
fn test_brk() {
    let mut cpu = Cpu::new();
    cpu.program_counter = 0x07FD;
    let status_before_brk = cpu.status_register;

    let result = cpu.run();
    let status_from_brk = cpu.stack_pop();
    let error_program_counter = cpu.stack_pop_address();

    assert_eq!(result, ExitStatus::Brk);
    assert_eq!(cpu.program_counter, 0x0000);
    assert_eq!(error_program_counter, 0x07FF);
    assert_eq!(
        status_before_brk.union(Status::BREAK).bits(),
        status_from_brk
    );
}
