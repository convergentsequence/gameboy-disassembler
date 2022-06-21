use std::fmt::Display;

use crate::opcodes::{OPCODES, CB_PREFIXED_OPCODES};

#[derive(Clone)]
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

pub struct Instruction {
    opcode: Opcode,
    argument: u16,
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

pub fn classify_intruction(buffer: &mut Vec<u8>, current_byte: &usize) -> Option<(Instruction, usize)>{
    let mut opcode: Option<&Opcode> = None;
    for oc in OPCODES.iter() {
        if buffer[*current_byte] == oc.start_byte {
            opcode = Some(oc);
        }
    }
    match opcode {
        Some(oc) => {  
            if oc.start_byte == 0xCB {
                return Some((
                    Instruction{
                        opcode: CB_PREFIXED_OPCODES[buffer[*current_byte+1] as usize].clone(),
                        argument: 0,
                    },
                    1
                ));
            }
            
            let mut argument: u16 = 0;
            for i in 0..oc.argument_count {
                argument >>= 8;
                argument |= (buffer[*current_byte + i as usize + 1] as u16) << 8;
            }
            return Some((
                Instruction{
                    opcode: oc.clone(),
                    argument: argument,
                },
                oc.argument_count as usize
            ));
        }
        None => {}
    }

    None
}