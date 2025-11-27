use r_nes::{
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_php_implicit() {
    let mut nes = Nes::new();
    nes.cpu.status_register = Status::all();
    nes.cpu.status_register.remove(Status::BREAK);
    nes.bus.cpu_memory[0] = 0x08;

    let result = nes.clock();
    let status_from_php = nes.cpu.stack_pop(&mut nes.bus);

    assert!(result.is_none());
    assert_eq!(Status::all().bits(), status_from_php);
}
