#![allow(unused_variables)]

mod classifier;
mod opcodes;

use opcodes::OPCODES;

fn main() {
    let target: String = r"E:\gameboy\gameboy-roms\bootrom.gb".to_owned();

    println!("All instructions");
    println!("----------------------");
    for opcode in OPCODES.iter() {
        println!("{}", opcode);
    }
}
