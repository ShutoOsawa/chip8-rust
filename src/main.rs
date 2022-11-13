use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    //get arguments
    let args: Vec<String> = env::args().collect();
    //get the first argument
    let file_path = Path::new(&args[1]);
    let file = File::open(&file_path).unwrap();

    //load the file into a buffer
    let my_buf = BufReader::new(file);

    let mut chip8: Chip8 = Chip8 {
        ram: [0; 4096]
    };

    let mut current = 200;
    //read each byte from the file
    //and load into ram
    for byte_or_error in my_buf.bytes() {
        let byte = byte_or_error.unwrap();
        println!("{:X}", byte);
        chip8.ram[current] = byte;
        current += 1;
    }

    println!("{:?}", &chip8.ram[190..210]);
}

//prepare chip8 structure
struct Chip8 {
    ram: [u8; 4096]
}

