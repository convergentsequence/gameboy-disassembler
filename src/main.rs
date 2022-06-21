mod classifier;
mod opcodes;

use std::{fs::{File, self}, io::Read, env};

use classifier::classify_intruction;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let target: String = 
        if args.len() > 1 
            {args[1].clone()} 
        else 
            {r"E:\gameboy\gameboy-roms\bootrom.gb".to_owned()};

    let mut buffer: Vec<u8> = vec![];

    {
        let mut file = File::open(&target).expect("can't find file");
        let metadata = fs::metadata(&target).expect("can't read metadata");
        buffer.resize(metadata.len() as usize, 0);
        file.read(&mut buffer[..]).unwrap();
    }

    let mut current_byte = 0usize;
    while current_byte < buffer.len() {
        if let Some((instruction, consumed_bytes)) = classify_intruction(&mut buffer, &current_byte) {
            // let mut hex = "".to_owned();
            // for i in current_byte..=current_byte+consumed_bytes {
            //     hex.push_str(&format!("{:02X} ", buffer[i]));
            // }

            println!("{}", instruction);
            current_byte += consumed_bytes;
        }
        current_byte += 1;
    }
}
