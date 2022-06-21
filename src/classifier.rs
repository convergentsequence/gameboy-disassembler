#![allow(dead_code)]

use std::fmt::Display;

pub struct Opcode  {
    pub name: &'static str,
    pub start_byte: u8,
    pub argument_count: u8,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f ,"op: {}, sb: 0x{:02X}, ac: {}", self.name, self.start_byte, self.argument_count)
    }
}

#[macro_export]
macro_rules! opcode {
    ($name:expr, $start_byte:expr, $arg_len:expr) => {
        Opcode {
            name: $name,
            start_byte: $start_byte,
            argument_count: $arg_len,
        }
    };
    ($name:expr, $start_byte:expr) => {
        Opcode {
            name: $name,
            start_byte: $start_byte,
            argument_count: 0,
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