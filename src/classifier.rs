#![allow(dead_code)]

use std::fmt::Display;

use crate::opcodes::OPCODES;

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
}

enum InstructionArgument {
    NoArg,
    OneByte(u8),
    TwoByte(u16),
}

pub struct Instruction {
    opcode: Opcode,
    argument: InstructionArgument,
}

pub fn classify_intruction(buffer: &mut Vec<u8>, current_byte: &mut usize){
    let mut opcode: Option<&Opcode> = None;
    for oc in OPCODES.iter() {
        if buffer[*current_byte] == oc.start_byte {
            opcode = Some(oc);
        }
    }
    match opcode {
        Some(oc) => {
            *current_byte += 1 + oc.argument_count as usize;
            if oc.start_byte == 0xCB {
                *current_byte += 1;
            }
            println!("{}", oc.name);
        }
        None => {
            *current_byte += 1;
            println!("Unknown opcode, skipping");
        }
    }
}