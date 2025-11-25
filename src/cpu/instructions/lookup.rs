use crate::cpu::{
    Cpu, ExitStatus, addressing_modes::AddressingModes, instructions::Instructions,
    operand::OperandLocation,
};

#[derive(Debug)]
pub struct Instruction {
    pub addressing_mode: fn(&mut Cpu) -> OperandLocation,
    pub operation: fn(&mut Cpu) -> Option<ExitStatus>,
}

pub const INSTRUCTIONS_LOOKUP: [Option<Instruction>; 256] = [
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::brk,
    }), // 0x00
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::ora,
    }), // 0x01
    None, // 0x02
    None, // 0x03
    None, // 0x04
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::ora,
    }), // 0x05
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::asl,
    }), // 0x06
    None, // 0x07
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::php,
    }), // 0x08
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::ora,
    }), // 0x09
    Some(Instruction {
        addressing_mode: Cpu::accumulator,
        operation: Cpu::asl,
    }), // 0x0A
    None, // 0x0B
    None, // 0x0C
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::ora,
    }), // 0x0D
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::asl,
    }), // 0x0E
    None, // 0x0F
    Some(Instruction {
        addressing_mode: Cpu::relative,
        operation: Cpu::bpl,
    }), // 0x10
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::ora,
    }), // 0x11
    None, // 0x12
    None, // 0x13
    None, // 0x14
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::ora,
    }), // 0x15
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::asl,
    }), // 0x16
    None, // 0x17
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::clc,
    }), // 0x18
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::ora,
    }), // 0x19
    None, // 0x1A
    None, // 0x1B
    None, // 0x1C
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::ora,
    }), // 0x1D
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::asl,
    }), // 0x1E
    None, // 0x1F
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::jsr,
    }), // 0x20
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::and,
    }), // 0x21
    None, // 0x22
    None, // 0x23
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::bit,
    }), // 0x24
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::and,
    }), // 0x25
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::rol,
    }), // 0x26
    None, // 0x27
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::plp,
    }), // 0x28
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::and,
    }), // 0x29
    Some(Instruction {
        addressing_mode: Cpu::accumulator,
        operation: Cpu::rol,
    }), // 0x2A
    None, // 0x2B
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::bit,
    }), // 0x2C
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::and,
    }), // 0x2D
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::rol,
    }), // 0x2E
    None, // 0x2F
    Some(Instruction {
        addressing_mode: Cpu::relative,
        operation: Cpu::bmi,
    }), // 0x30
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::and,
    }), // 0x31
    None, // 0x32
    None, // 0x33
    None, // 0x34
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::and,
    }), // 0x35
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::rol,
    }), // 0x36
    None, // 0x37
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::sec,
    }), // 0x38
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::and,
    }), // 0x39
    None, // 0x3A
    None, // 0x3B
    None, // 0x3C
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::and,
    }), // 0x3D
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::rol,
    }), // 0x3E
    None, // 0x3F
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::rti,
    }), // 0x40
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::eor,
    }), // 0x41
    None, // 0x42
    None, // 0x43
    None, // 0x44
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::eor,
    }), // 0x45
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::lsr,
    }), // 0x46
    None, // 0x47
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::pha,
    }), // 0x48
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::eor,
    }), // 0x49
    Some(Instruction {
        addressing_mode: Cpu::accumulator,
        operation: Cpu::lsr,
    }), // 0x4A
    None, // 0x4B
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::jmp,
    }), // 0x4C
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::eor,
    }), // 0x4D
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::lsr,
    }), // 0x4E
    None, // 0x4F
    Some(Instruction {
        addressing_mode: Cpu::relative,
        operation: Cpu::bvc,
    }), // 0x50
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::eor,
    }), // 0x51
    None, // 0x52
    None, // 0x53
    None, // 0x54
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::eor,
    }), // 0x55
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::lsr,
    }), // 0x56
    None, // 0x57
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::cli,
    }), // 0x58
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::eor,
    }), // 0x59
    None, // 0x5A
    None, // 0x5B
    None, // 0x5C
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::eor,
    }), // 0x5D
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::lsr,
    }), // 0x5E
    None, // 0x5F
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::rts,
    }), // 0x60
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::adc,
    }), // 0x61
    None, // 0x62
    None, // 0x63
    None, // 0x64
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::adc,
    }), // 0x65
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::ror,
    }), // 0x66
    None, // 0x67
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::pla,
    }), // 0x68
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::adc,
    }), // 0x69
    Some(Instruction {
        addressing_mode: Cpu::accumulator,
        operation: Cpu::ror,
    }), // 0x6A
    None, // 0x6B
    Some(Instruction {
        addressing_mode: Cpu::indirect,
        operation: Cpu::jmp,
    }), // 0x6C
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::adc,
    }), // 0x6D
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::ror,
    }), // 0x6E
    None, // 0x6F
    Some(Instruction {
        addressing_mode: Cpu::relative,
        operation: Cpu::bvs,
    }), // 0x70
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::adc,
    }), // 0x71
    None, // 0x72
    None, // 0x73
    None, // 0x74
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::adc,
    }), // 0x75
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::ror,
    }), // 0x76
    None, // 0x77
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::sei,
    }), // 0x78
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::adc,
    }), // 0x79
    None, // 0x7A
    None, // 0x7B
    None, // 0x7C
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::adc,
    }), // 0x7D
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::ror,
    }), // 0x7E
    None, // 0x7F
    None, // 0x80
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::sta,
    }), // 0x81
    None, // 0x82
    None, // 0x83
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::sty,
    }), // 0x84
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::sta,
    }), // 0x85
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::stx,
    }), // 0x86
    None, // 0x87
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::dey,
    }), // 0x88
    None, // 0x89
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::txa,
    }), // 0x8A
    None, // 0x8B
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::sty,
    }), // 0x8C
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::sta,
    }), // 0x8D
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::stx,
    }), // 0x8E
    None, // 0x8F
    Some(Instruction {
        addressing_mode: Cpu::relative,
        operation: Cpu::bcc,
    }), // 0x90
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::sta,
    }), // 0x91
    None, // 0x92
    None, // 0x93
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::sty,
    }), // 0x94
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::sta,
    }), // 0x95
    Some(Instruction {
        addressing_mode: Cpu::zero_page_y,
        operation: Cpu::stx,
    }), // 0x96
    None, // 0x97
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::tya,
    }), // 0x98
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::sta,
    }), // 0x99
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::txs,
    }), // 0x9A
    None, // 0x9B
    None, // 0x9C
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::sta,
    }), // 0x9D
    None, // 0x9E
    None, // 0x9F
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::ldy,
    }), // 0xA0
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::lda,
    }), // 0xA1
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::ldx,
    }), // 0xA2
    None, // 0xA3
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::ldy,
    }), // 0xA4
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::lda,
    }), // 0xA5
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::ldx,
    }), // 0xA6
    None, // 0xA7
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::tay,
    }), // 0xA8
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::lda,
    }), // 0xA9
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::tax,
    }), // 0xAA
    None, // 0xAB
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::ldy,
    }), // 0xAC
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::lda,
    }), // 0xAD
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::ldx,
    }), // 0xAE
    None, // 0xAF
    Some(Instruction {
        addressing_mode: Cpu::relative,
        operation: Cpu::bcs,
    }), // 0xB0
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::lda,
    }), // 0xB1
    None, // 0xB2
    None, // 0xB3
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::ldy,
    }), // 0xB4
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::lda,
    }), // 0xB5
    Some(Instruction {
        addressing_mode: Cpu::zero_page_y,
        operation: Cpu::ldx,
    }), // 0xB6
    None, // 0xB7
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::clv,
    }), // 0xB8
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::lda,
    }), // 0xB9
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::tsx,
    }), // 0xBA
    None, // 0xBB
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::ldy,
    }), // 0xBC
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::lda,
    }), // 0xBD
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::ldx,
    }), // 0xBE
    None, // 0xBF
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::cpy,
    }), // 0xC0
    Some(Instruction {
        addressing_mode: Cpu::indirect_x,
        operation: Cpu::cmp,
    }), // 0xC1
    None, // 0xC2
    None, // 0xC3
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::cpy,
    }), // 0xC4
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::cmp,
    }), // 0xC5
    Some(Instruction {
        addressing_mode: Cpu::zero_page,
        operation: Cpu::dec,
    }), // 0xC6
    None, // 0xC7
    Some(Instruction {
        addressing_mode: Cpu::implicit,
        operation: Cpu::iny,
    }), // 0xC8
    Some(Instruction {
        addressing_mode: Cpu::immediate,
        operation: Cpu::cmp,
    }), // 0xC9
    None, // 0xCA
    None, // 0xCB
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::cpy,
    }), // 0xCC
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::cmp,
    }), // 0xCD
    Some(Instruction {
        addressing_mode: Cpu::absolute,
        operation: Cpu::dec,
    }), // 0xCE
    None, // 0xCF
    None, // 0xD0
    Some(Instruction {
        addressing_mode: Cpu::indirect_y,
        operation: Cpu::cmp,
    }), // 0xD1
    None, // 0xD2
    None, // 0xD3
    None, // 0xD4
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::cmp,
    }), // 0xD5
    Some(Instruction {
        addressing_mode: Cpu::zero_page_x,
        operation: Cpu::dec,
    }), // 0xD6
    None, // 0xD7
    None, // 0xD8
    Some(Instruction {
        addressing_mode: Cpu::absolute_y,
        operation: Cpu::cmp,
    }), // 0xD9
    None, // 0xDA
    None, // 0xDB
    None, // 0xDC
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::cmp,
    }), // 0xDD
    Some(Instruction {
        addressing_mode: Cpu::absolute_x,
        operation: Cpu::dec,
    }), // 0xDE
    None, // 0xDF
    None, // 0xE0
    None, // 0xE1
    None, // 0xE2
    None, // 0xE3
    None, // 0xE4
    None, // 0xE5
    None, // 0xE6
    None, // 0xE7
    None, // 0xE8
    None, // 0xE9
    None, // 0xEA
    None, // 0xEB
    None, // 0xEC
    None, // 0xED
    None, // 0xEE
    None, // 0xEF
    None, // 0xF0
    None, // 0xF1
    None, // 0xF2
    None, // 0xF3
    None, // 0xF4
    None, // 0xF5
    None, // 0xF6
    None, // 0xF7
    None, // 0xF8
    None, // 0xF9
    None, // 0xFA
    None, // 0xFB
    None, // 0xFC
    None, // 0xFD
    None, // 0xFE
    None, // 0xFF
];
