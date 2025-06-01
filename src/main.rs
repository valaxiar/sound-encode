use std::io::{Read, Write};
use std::process::Output;
use std::{env::args, path::PathBuf};
use std::fs::{self, File};
use hound::{self, WavWriter, WavReader};


fn main() {
    let args: Vec<String> = args().collect();
    let input_bytes = fs::read(&args[2]).expect("not a valid file input");
    let operation = match args[1].as_str() {
        "e" => encode(input_bytes),
        "d" => decode(&args[2]),
        _ => {panic!("not a valid operation")}
    };
}

fn encode (input: Vec<u8>) {
let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};
let mut writer = WavWriter::create("encoded-data.wav", spec).unwrap();


for chunk in input.chunks(2) {
        let sample = if chunk.len() == 2 {
            i16::from_le_bytes([chunk[0], chunk[1]])
        } else {
            i16::from_le_bytes([chunk[0], 0])
        };
        writer.write_sample(sample).unwrap();
    }

}

fn decode(path: &str) {
let reader = WavReader::open(path).expect("no");
let mut output = Vec::new();
    for sample in reader.into_samples::<i16>() {
    let bytes = sample.expect("bad sample").to_le_bytes();
    output.push(bytes[0]);
    output.push(bytes[1]);

}

let mut out_file = File::create("out.bin").expect("can't make file");
out_file.write_all(&output).expect("can't write file")
}

