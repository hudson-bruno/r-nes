use r_nes::{cpu::Status, nes::Nes};

#[test]
fn test_clc() {
    let mut nes = Nes::new();
    nes.cpu.status_register.insert(Status::CARRY);
    nes.bus.cpu_memory[0] = 0x18;
    let result = nes.clock();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::CARRY));
}
