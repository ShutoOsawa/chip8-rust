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
            ram: [0; 4096],
            i_reg: 0x0,
            v_reg: [0;16]
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
        
        //get each digit
        let digits:[u16;4] = [ 
             (op & 0xF000) >> 12,
             (op & 0x0F00) >> 8,
             (op & 0x00F0) >> 4,
             op & 0x000F];
        
        println!("chip8 I: {:X}",self.i_reg);
        //execute
        match (digits[0],digits[1],digits[2],digits[3]) {
            (0xA, _, _, _) => Self::exec_annn(digits),
            (_, _, _, _) => Self::exec_invalid(digits),
        }
    }
    
    
    fn exec_annn(digits:[u16;4]){
        println!("ANNN op code received: {:X?}",digits);
        let num :u16 = digits[1]<<8 | digits[2]<<4 | digits[3];
        println!("NNN: {:X}",num);
        //chip8.I = num;
    }
    
    fn exec_invalid(digits:[u16;4]){
        println!("Invalid op code: {:X?}", digits);
    }
    
}

//prepare chip8 structure
struct Chip8 {
    pc: u16,
    ram: [u8; 4096],
    i_reg: u16,
    v_reg: [u8; 16]
}

