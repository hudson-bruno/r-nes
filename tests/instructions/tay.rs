use r_nes::{
    cpu::Status,
    nes::Nes,
};

#[test]
fn test_tay() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x01;
    nes.bus.cpu_memory[0] = 0xA8;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.y_index_register, 0x01);
}

#[test]
fn test_tay_status_zero() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0x00;
    nes.bus.cpu_memory[0] = 0xA8;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
    assert_eq!(nes.cpu.y_index_register, 0x00);
}

#[test]
fn test_tay_status_negative() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xFF;
    nes.bus.cpu_memory[0] = 0xA8;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.y_index_register, 0xFF);
}
