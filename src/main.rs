use std::env::args;
use std::fs::{self, File};

mod encode;
mod decode;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage [d,e] filename")
    }
    let input_bytes = fs::read(&args[2]).expect("not a valid file input");
    let operation = match args[1].as_str() {
        "e" => encode::encode(input_bytes),
        "d" => decode::decode(&args[2]),
        _ => {panic!("not a valid operation")}
    };
}
