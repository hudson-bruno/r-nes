use r_nes::cpu::Cpu;

#[test]
fn test_txs() {
    let mut cpu = Cpu::new();
    cpu.x_index_register = 0x01;
    cpu.memory[0] = 0x9A;

    let result = cpu.clock();

    assert!(result.is_none());
    assert_eq!(cpu.stack_pointer, 0x01);
}
