use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main(){
    let mut chip8 = Chip8::new();
    chip8.load_rom();
    loop {
        chip8.run();
    }
}

impl Chip8{
    fn new() -> Self{
        Self {
            pc: 0x200,
            ram: [0; 4096]
        }
    }
    
    fn load_rom(&mut self){
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
        //fetch
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc+1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;

        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;
        
        //execute
        match (digit1,digit2,digit3,digit4) {
            (0xA, _, _, _) => Self::exec_ANNN(op),
            (_, _, _, _) => Self::exec_invalid(op),
        }
    }
    
    fn exec_ANNN(op:u16){
        println!("ANNN op code received: {:X}",op);
    }
    
    fn exec_invalid(op:u16){
        println!("Invalid op code: {:X}", op);
    }
    
}

//prepare chip8 structure
struct Chip8 {
    pc: u16,
    ram: [u8; 4096]
}

