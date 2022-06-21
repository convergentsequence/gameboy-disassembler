#![allow(dead_code)]

use crate::{classifier::Opcode, opcode};

#[allow(dead_code)]
pub const OPCODES: &[Opcode] = &[
    opcode!("NOP", 0x00),
    opcode!("LC BC, u16", 0x01, 2),
    opcode!("LC (BC), A", 0x02),
    opcode!("INC BC" ,0x03, 1),
    opcode!("INC B", 0x04),
    opcode!("DEC B", 0x05),
    opcode!("LD B, u8", 0x06, 1),
    opcode!("RLCA", 0x07),
    opcode!("LD (u16), SP", 0x08, 2),
];