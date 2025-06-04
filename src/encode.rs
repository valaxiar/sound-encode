use std::{clone, path::{Path, PathBuf}};
use hound::WavWriter;

pub fn encode(input: Vec<u8>, input_path: &Path, output_filename: &Path) {
    let output_path = if output_filename.extension().map_or(true, |ext| ext != "wav") {
        let mut p = output_filename.to_path_buf();
        p.set_extension("wav");
        p
    } else {
        output_filename.to_path_buf()
    };

    let checksum = crc32fast::hash(&input);
    println!("Checksum: {}", checksum);

    let extension = input_path.extension().and_then(|e| e.to_str()).unwrap_or("bin");
    let extension_bytes = extension.as_bytes();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 352800,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(&output_path, spec).unwrap();

    let mut buf = Vec::with_capacity(1 + extension_bytes.len() + 4 + input.len());
    buf.push(extension_bytes.len() as u8);
    buf.extend_from_slice(extension_bytes);
    buf.extend_from_slice(&checksum.to_le_bytes());
    buf.extend_from_slice(&input);


    for chunk in buf.chunks(2) {
        let sample = if chunk.len() == 2 {
            i16::from_le_bytes([chunk[0], chunk[1]])
        } else {
            i16::from_le_bytes([chunk[0], 0])
        };
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().expect("couldn't finalize wav file");
    println!("File written to {}", &output_path.to_string_lossy());
}


