use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = Path::new(&args[1]);
    let file = File::open(&file_path).unwrap();
    let my_buf = BufReader::new(file);
    for byte_or_error in my_buf.bytes() {
        let byte = byte_or_error.unwrap();
        println!("{:x}", byte);
    }
}

