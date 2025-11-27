use r_nes::{
    cpu::{Status, memory::stack::Stack},
    nes::Nes,
};

#[test]
fn test_pla() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0] = 0x68;
    nes.cpu.stack_push(&mut nes.bus, 0xFF);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xFF);
}

#[test]
fn test_pla_status_zero() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0] = 0x68;
    nes.cpu.stack_push(&mut nes.bus, 0x00);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
    assert_eq!(nes.cpu.a_register, 0x00);
}

#[test]
fn test_pla_status_negative() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0] = 0x68;
    nes.cpu.stack_push(&mut nes.bus, 0x80);

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.a_register, 0x80);
}
