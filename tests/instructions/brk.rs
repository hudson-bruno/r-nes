use r_nes::{
    cpu::{ExitStatus, Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_brk() {
    let mut nes = Nes::new();
    nes.cpu.program_counter = 0x07FD;
    let status_before_brk = nes.cpu.status_register;

    let result = nes.clock();

    let status_from_brk = nes.cpu.stack_pop(&mut nes.bus);
    let error_program_counter = nes.cpu.stack_pop_address(&mut nes.bus);

    assert_eq!(result, Some(ExitStatus::Brk));
    assert_eq!(nes.cpu.program_counter, 0x0000);
    assert_eq!(error_program_counter, 0x07FF);
    assert_eq!(
        status_before_brk.union(Status::BREAK).bits(),
        status_from_brk
    );
}
