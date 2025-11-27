use r_nes::nes::Nes;

#[test]
fn test_eor_immediate() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x49, 0xBA]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_zero_page() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x45, 0x03, 0x00, 0xBA]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_zero_page_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0x55, 0xFE]);
    nes.bus.cpu_memory[0xFF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_zero_page_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..4].copy_from_slice(&[0x55, 0xFF, 0x00, 0xBA]);
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_absolute() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x4D, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_absolute_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x5D, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_eor_absolute_x_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xAB;
//     nes.cpu.x_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x5D, 0xFF, 0x07, 0x00, 0xBA]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
// }

#[test]
fn test_eor_absolute_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..3].copy_from_slice(&[0x59, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_eor_absolute_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xAB;
//     nes.cpu.y_index_register = 0x05;
//     nes.bus.cpu_memory[0..5].copy_from_slice(&[0x59, 0xFF, 0x07, 0x00, 0xBA]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
// }

#[test]
fn test_eor_indirect_x() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x41, 0x02, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_indirect_x_overflow() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.x_index_register = 0x04;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x41, 0xFF, 0x00, 0xFF, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

#[test]
fn test_eor_indirect_y() {
    let mut nes = Nes::new();
    nes.cpu.a_register = 0xAB;
    nes.cpu.y_index_register = 0x01;
    nes.bus.cpu_memory[0..5].copy_from_slice(&[0x51, 0x03, 0x00, 0xFE, 0x07]);
    nes.bus.cpu_memory[0x07FF] = 0xBA;
    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
}

// At the moment the cpu does not have access to
// the full memory range, failing to test the overflow
// case
// #[test]
// fn test_eor_indirect_y_overflow() {
//     let mut nes = Nes::new();
//     nes.cpu.a_register = 0xAB;
//     nes.cpu.y_index_register = 0x06;
//     nes.bus.cpu_memory[0..6].copy_from_slice(&[0x51, 0x03, 0x00, 0xFF, 0x07, 0xBA]);
//     let result = nes.clock();
//
//     assert!(result.is_none());
//     assert_eq!(nes.cpu.a_register, 0xAB ^ 0xBA);
// }
