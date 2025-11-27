use r_nes::nes::Nes;

#[test]
fn test_ora_immediate() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x09, 0xBA, 0x00]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x05, 0x03, 0x00, 0xBA]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x15, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x15, 0xFF, 0x00, 0xBA]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x0D, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x1D, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ora_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xAB;
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x1D, 0xFF, 0x07, 0x00, 0xBA]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
// }

#[test]
fn test_ora_absolute_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x19, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ora_absolute_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xAB;
//     nes.cpu.y_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0xB9, 0xFF, 0x07, 0x00, 0xBA]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
// }

#[test]
fn test_ora_indirect_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x01, 0x02, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x01, 0xFF, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

#[test]
fn test_ora_indirect_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x11, 0x03, 0x00, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_ora_indirect_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xAB;
//     nes.cpu.y_index_register = 0x06;
//     nes.bus.cpu_memory[0..6].copy_from_slice(&[0x11, 0x03, 0x00, 0xFF, 0x07, 0xBA]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0xAB | 0xBA);
// }
