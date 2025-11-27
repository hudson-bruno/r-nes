use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_beq() {
    let mut nes = Nes::new();
    nes.cpu.status_register.insert(Status::ZERO);
    nes.bus.cpu_memory[0..2].copy_from_slice(&[0xF0, 0x7F]);

    let result = nes.clock();

    assert!(result.is_none());
    assert_eq!(nes.cpu.program_counter, 0x81);
}
