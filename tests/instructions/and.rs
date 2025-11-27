use r_nes::nes::Nes;

#[test]
fn test_and_immediate() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x29, 0xBA]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_zero_page() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x25, 0x03, 0x00, 0xBA]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x35, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x35, 0xFF, 0x00, 0xBA]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_absolute() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x2D, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x3D, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_absolute_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x39, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_indirect_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x21, 0x02, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_indirect_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x21, 0xFF, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}

#[test]
fn test_and_indirect_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x31, 0x03, 0x00, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB & 0xBA);
}
