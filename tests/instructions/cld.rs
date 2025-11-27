use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_cld() {
    let mut nes = Nes::new();
    nes.cpu.status_register.insert(Status::DECIMAL);
    nes.bus.cpu_memory[0] = 0xD8;
    let result = nes.clock();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::DECIMAL));
}
