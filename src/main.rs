use std::env::args;
use std::fs::{self};
use std::path::Path;

mod util;


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 4 || args.len() > 5 {
        eprintln!("Usage: {} <e|d> <input-file> <output-file> [c (optional compression)]", args[0]);
        std::process::exit(1);
    }

    let mode = args[1].as_str();
    let input_path = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);
    let compression = args.get(4).map(|s| s == "c").unwrap_or(false);

    let input_bytes = fs::read(input_path).expect("failed to read input file");

    match mode {
        "e" => util::encode(input_bytes, input_path, output_path, compression),
        "d" => util::decode(input_path, output_path, compression),
        _ => {
            eprintln!("invalid: use 'e' for encode or 'd' for decode.");
            std::process::exit(1);
        }
    }
}
