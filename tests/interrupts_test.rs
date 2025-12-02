use r_nes::{
    cartridge::Cartridge,
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_reset() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.status_register.remove(Status::INTERRUPT);
    let old_stack_pointer = nes.cpu.stack_pointer;

    nes.reset();

    assert_eq!(nes.cpu.program_counter, 0x8000);
    assert_eq!(nes.cpu.stack_pointer, old_stack_pointer.wrapping_sub(3));
    assert!(nes.cpu.status_register.contains(Status::INTERRUPT));
}

#[test]
fn test_irq() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x7FFE..=0x7FFF].copy_from_slice(&[0xFF, 0x87]); // IRQ vector to 0x87FF
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
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
    assert_eq!(nes.cpu.program_counter, 0x87FF);
}

#[test]
fn test_irq_status_interrupt_disable() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x7FFE..=0x7FFF].copy_from_slice(&[0xFF, 0x87]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.program_counter = 0xFF;
    nes.cpu.status_register.insert(Status::INTERRUPT);

    nes.irq();

    assert_eq!(nes.cpu.program_counter, 0xFF);
}

#[test]
fn test_nmi() {
    let mut cartridge = Cartridge::new();
    cartridge.program_memory[0x7FFA..=0x7FFB].copy_from_slice(&[0xFF, 0x87]);
    cartridge.program_memory[0x7FFC..=0x7FFD].copy_from_slice(&[0x00, 0x80]);

    let mut nes = Nes::new_with_cartridge(cartridge);
    nes.cpu.program_counter = 0xFF;

    nes.nmi();

    let status_in_stack = Status::from_bits_retain(nes.cpu.stack_pop(&mut nes.bus));
    let program_counter_in_stack = nes.cpu.stack_pop_address(&mut nes.bus);

    assert_eq!(program_counter_in_stack, 0xFF);
    assert_eq!(
        nes.cpu.status_register.difference(Status::BREAK),
        status_in_stack
    );
    assert_eq!(nes.cpu.program_counter, 0x87FF);
}
