#![allow(unused_variables)]

mod classifier;
mod opcodes;

use std::{fs::{File, self}, io::Read};

use opcodes::OPCODES;

fn main() {
    let target: String = r"E:\gameboy\gameboy-roms\bootrom.gb".to_owned();

    let mut buffer: Vec<u8> = vec![];

    {
        let mut file = File::open(&target).expect("can't find file");
        let metadata = fs::metadata(&target).expect("can't read metadata");
        buffer.resize(metadata.len() as usize, 0);
        file.read(&mut buffer[..]).unwrap();
    }

    let mut current_byte = 0usize;
    while current_byte < buffer.len() {
        classifier::classify_intruction(&mut buffer, &mut current_byte);
    }
}
