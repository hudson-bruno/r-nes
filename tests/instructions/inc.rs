use r_nes::nes::Nes;

#[test]
fn test_inc_zero_page() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xE6, 0x03, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0x02);
}

#[test]
fn test_inc_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xF6, 0xFE, 0x00]);
    nes.bus.cpu_memory[0xFF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0xFF], 0x02);
}

#[test]
fn test_inc_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0xF6, 0xFF, 0x00, 0x01]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x03], 0x02);
}

#[test]
fn test_inc_absolute() {
    let mut nes = Nes::new();
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xEE, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0x02);
}

#[test]
fn test_inc_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0xFE, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0x01;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.bus.cpu_memory[0x07FF], 0x02);
}
