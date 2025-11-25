use r_nes::cpu::Cpu;

#[test]
fn test_nop() {
    let mut cpu = Cpu::new();
    cpu.memory[0] = 0xEA;

    let result = cpu.clock();

    assert!(result.is_none());
}
