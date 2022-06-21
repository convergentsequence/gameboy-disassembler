#![allow(dead_code)]

use std::fmt::Display;

pub struct Opcode  {
    pub name: &'static str,
    pub start_byte: u8,
    pub arg_len: u8,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f ,"op: {} \t| sb: 0x{:02X} \t| ac: {}", self.name, self.start_byte, self.arg_len)
    }
}

#[macro_export]
macro_rules! opcode {
    ($name:expr, $start_byte:expr, $arg_len:expr) => {
        Opcode {
            name: $name,
            start_byte: $start_byte,
            arg_len: $arg_len,
        }
    };
    ($name:expr, $start_byte:expr) => {
        Opcode {
            name: $name,
            start_byte: $start_byte,
            arg_len: 0,
        }
    };
}

enum InstructionArgument {
    NoArg,
    OneByte(u8),
    TwoByte(u16),
}

struct Instruction {
    opcode: Opcode,
    argument: InstructionArgument,
}