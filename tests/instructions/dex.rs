use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_dex() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x02;
    nes.bus.cpu_memory[0] = 0xCA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.x_index_register, 0x01);
}

#[test]
fn test_dex_status_zero() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0] = 0xCA;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::ZERO));
    assert_eq!(nes.cpu.x_index_register, 0x00);
}

#[test]
fn test_dex_status_negative() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x00;
    nes.bus.cpu_memory[0] = 0xCA;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(nes.cpu.status_register.contains(Status::NEGATIVE));
    assert_eq!(nes.cpu.x_index_register, 0xFF);
}
