use r_nes::{
    cpu::Status,
    nes::Nes,
};

#[test]
fn test_clv() {
    let mut nes = Nes::new();
    nes.cpu.status_register.insert(Status::OVERFLOW);
    nes.bus.cpu_memory[0] = 0xB8;

    let result = nes.clock();

    assert!(result.is_none());
    assert!(!nes.cpu.status_register.contains(Status::OVERFLOW));
}
