use hound::WavReader;
use std::fs::File;
use std::io::Write;

pub fn decode(path: &str) {
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
