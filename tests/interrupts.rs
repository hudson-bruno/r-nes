use r_nes::cpu::{Cpu, Status, memory::stack::Stack};

#[test]
fn test_reset() {
    let mut cpu = Cpu::new();
    cpu.status_register.remove(Status::INTERRUPT);
    let old_stack_pointer = cpu.stack_pointer;

    cpu.reset();

    assert_eq!(cpu.program_counter, 0x0000);
    assert_eq!(cpu.stack_pointer, old_stack_pointer - 3);
    assert!(cpu.status_register.contains(Status::INTERRUPT));
}

#[test]
fn test_irq() {
    let mut cpu = Cpu::new();
    cpu.memory[0xFFFE..=0xFFFF].copy_from_slice(&[0xFF, 0x07]);
    cpu.program_counter = 0xFF;
    cpu.status_register.remove(Status::INTERRUPT);

    cpu.irq();

    let status_in_stack = Status::from_bits_retain(cpu.stack_pop());
    let program_counter_in_stack = cpu.stack_pop_address();

    assert_eq!(program_counter_in_stack, 0xFF);
    assert_eq!(cpu.status_register.union(Status::BREAK), status_in_stack);
    assert_eq!(cpu.program_counter, 0x07FF);
}

#[test]
fn test_irq_status_interrupt_disable() {
    let mut cpu = Cpu::new();
    cpu.memory[0xFFFE..=0xFFFF].copy_from_slice(&[0xFF, 0x07]);
    cpu.program_counter = 0xFF;
    cpu.status_register.insert(Status::INTERRUPT);

    cpu.irq();

    assert_eq!(cpu.program_counter, 0xFF);
}

#[test]
fn test_nmi() {
    let mut cpu = Cpu::new();
    cpu.memory[0xFFFA..=0xFFFB].copy_from_slice(&[0xFF, 0x07]);
    cpu.program_counter = 0xFF;

    cpu.nmi();

    let status_in_stack = Status::from_bits_retain(cpu.stack_pop());
    let program_counter_in_stack = cpu.stack_pop_address();

    assert_eq!(program_counter_in_stack, 0xFF);
    assert_eq!(cpu.status_register.union(Status::BREAK), status_in_stack);
    assert_eq!(cpu.program_counter, 0x07FF);
}
