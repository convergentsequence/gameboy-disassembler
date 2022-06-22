use std::fmt::Display;

use crate::opcodes::{OPCODES, CB_PREFIXED_OPCODES};

#[derive(Clone)]
pub struct Opcode  {
    pub name: &'static str,
    pub start_byte: u8,
    pub argument_count: u8,
    pub cb_prefixed: bool,
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
            cb_prefixed: false
        }
    };
}

#[macro_export]
macro_rules! cb_opcode {
    ($name:expr, $start_byte:expr, $arg_len:expr) => {
        Opcode {
            name: $name,
            start_byte: $start_byte,
            argument_count: $arg_len,
            cb_prefixed: true
        }
    };
}

pub struct Instruction {
    pub opcode: Opcode,
    pub argument: u16,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode.argument_count {
            1 => {
                let mut replaced = self.opcode.name.replace("u8", &format!("${:02x}", self.argument));
                replaced = replaced.replace("i8", &format!("${:02x}", self.argument));
                return write!(f, "{}", replaced);
            },
            2 => {
                let mut replaced = self.opcode.name.replace("u16", &format!("${:04x}", self.argument));
                replaced = replaced.replace("i16", &format!("${:04x}", self.argument));
                return write!(f, "{}", replaced);
            },
            _ => write!(f, "{}", self.opcode.name),
        }
        
    }
}

pub fn classify(buffer: &Vec<u8>, current_byte: &usize) -> (Instruction, usize){
    let opcode = &OPCODES[buffer[*current_byte] as usize];

    if opcode.start_byte == 0xCB {
        return (
            Instruction{
                opcode: CB_PREFIXED_OPCODES[buffer[*current_byte+1] as usize].clone(),
                argument: 0,
            },
            1
        );
    }

    let mut argument: u16 = 0;
    for i in (1..=opcode.argument_count).rev() {
        argument <<= 8;
        argument |= buffer[*current_byte + i as usize] as u16;        
    }

    return (
        Instruction{
            opcode: opcode.clone(),
            argument: argument,
        },
        opcode.argument_count as usize
    );
}