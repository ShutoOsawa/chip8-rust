use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main(){
    
    let mut chip8 = Chip8::new();
    chip8.load();
    chip8.run();
}

impl Chip8 {
    pub fn new() -> Self{
        Self {
            pc: 0x200,
            ram: [0; 4096]
        }
        
    }
    
    fn load(&mut self){
        //get arguments
        let args: Vec<String> = env::args().collect();
        //get the first argument
        let file_path = Path::new(&args[1]);
        let file = File::open(&file_path).unwrap();

        //load the file into a buffer
        let my_buf = BufReader::new(file);
        
        let mut current = 512;
        //read each byte from the file
        //and load into ram
        for byte_or_error in my_buf.bytes() {
            let byte = byte_or_error.unwrap();
            self.ram[current] = byte;
            current += 1;
        }

        println!("{:?}", &self.ram[500..520]);
        
    }
    
    fn run(&mut self) {
        println!("{:X}", self.ram[self.pc as usize] as u16)
    }
}

//prepare chip8 structure
struct Chip8 {
    pc: u16,
    ram: [u8; 4096]
}

