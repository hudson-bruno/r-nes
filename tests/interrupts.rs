use r_nes::{
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_reset() {
    let mut nes = Nes::new();
    nes.cpu.status_register.remove(Status::INTERRUPT);
    let old_stack_pointer = nes.cpu.stack_pointer;

    nes.reset();

    assert_eq!(nes.cpu.program_counter, 0x0000);
    assert_eq!(nes.cpu.stack_pointer, old_stack_pointer - 3);
    assert!(nes.cpu.status_register.contains(Status::INTERRUPT));
}

#[test]
fn test_irq() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0xFFFE..=0xFFFF].copy_from_slice(&[0xFF, 0x07]);
    nes.cpu.program_counter = 0xFF;
    nes.cpu.status_register.remove(Status::INTERRUPT);

    nes.irq();

    let status_in_stack = Status::from_bits_retain(nes.cpu.stack_pop(&mut nes.bus));
    let program_counter_in_stack = nes.cpu.stack_pop_address(&mut nes.bus);

    assert_eq!(program_counter_in_stack, 0xFF);
    assert_eq!(
        nes.cpu.status_register.difference(Status::BREAK),
        status_in_stack
    );
    assert_eq!(nes.cpu.program_counter, 0x07FF);
}

#[test]
fn test_irq_status_interrupt_disable() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0xFFFE..=0xFFFF].copy_from_slice(&[0xFF, 0x07]);
    nes.cpu.program_counter = 0xFF;
    nes.cpu.status_register.insert(Status::INTERRUPT);

    nes.irq();

    assert_eq!(nes.cpu.program_counter, 0xFF);
}

#[test]
fn test_nmi() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0xFFFA..=0xFFFB].copy_from_slice(&[0xFF, 0x07]);
    nes.cpu.program_counter = 0xFF;

    nes.nmi();

    let status_in_stack = Status::from_bits_retain(nes.cpu.stack_pop(&mut nes.bus));
    let program_counter_in_stack = nes.cpu.stack_pop_address(&mut nes.bus);

    assert_eq!(program_counter_in_stack, 0xFF);
    assert_eq!(
        nes.cpu.status_register.difference(Status::BREAK),
        status_in_stack
    );
    assert_eq!(nes.cpu.program_counter, 0x07FF);
}
