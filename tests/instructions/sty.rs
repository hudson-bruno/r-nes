use r_nes::nes::Nes;

#[test]
fn test_sty_zero_page() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x84, 0x03]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0xFF);
}

#[test]
fn test_sty_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x94, 0xFE]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0xFF], 0xFF);
}

#[test]
fn test_sty_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x94, 0xFF]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0xFF);
}

#[test]
fn test_sty_absolute() {
    let mut nes = Nes::new();
    nes.cpu.y_index_register = 0xFF;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x8C, 0xFF, 0x07]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0xFF);
}
