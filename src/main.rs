use std::env::args;
use std::fs::{self, File};
use std::path::Path;

mod decode;
mod encode;

fn main() {
    let args: Vec<String> = args().collect();

    let operation = &args[1];
    let input_path = Path::new(&args[2]);
    let output_filename = Path::new(&args[3]);

    if args.len() != 4 {
        eprintln!("Usage: [d,e] file-to-encode/decode <encode-output-filename>");
        panic!("Wrong number of args")
    }
    let input_bytes = fs::read(&args[2]).expect("not a valid file input");
    let target_filename = Path::new(&args[3]);
    match args[1].as_str() {
        "e" => encode::encode(input_bytes, input_path, output_filename),
        "d" => decode::decode(input_path, target_filename),
        _ => {
            panic!("not a valid operation")
        }
    };
}
