use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use hound::{WavReader, WavSpec, WavWriter, SampleFormat};
use snap::raw::{Encoder, Decoder};

pub const WAV_SPEC: WavSpec = WavSpec {
    channels: 1,
    sample_rate: 176_400,
    bits_per_sample: 16,
    sample_format: SampleFormat::Int,
};


pub fn encode(mut input: Vec<u8>, input_path: &Path, output_filename: &Path, compression: bool) {
    let checksum = crc32fast::hash(&input);

    if compression {
        input = compress(input);
    }

    let output_path = if output_filename
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(true, |ext| ext != "wav")
    {
        let mut p = output_filename.to_path_buf();
        p.set_extension("wav");
        p
    } else {
        output_filename.to_path_buf()
    };

    let extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");

    let extension_bytes = extension.as_bytes();

    let mut writer = WavWriter::create(&output_path, WAV_SPEC).unwrap();

    let mut buf = Vec::with_capacity(1 + extension_bytes.len() + 4 + 4 + 1 + input.len());
    buf.push(extension_bytes.len() as u8);
    buf.extend_from_slice(extension_bytes);
    buf.extend_from_slice(&checksum.to_le_bytes());
    buf.extend_from_slice(&(input.len() as u32).to_le_bytes());
    buf.push(compression as u8); // 0 or 1
    buf.extend_from_slice(&input);

    if buf.len() % 2 != 0 {
        buf.push(0);
    }

    for chunk in buf.chunks(2) {
        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().expect("couldn't finalize wav file");
    println!("File written to {}", output_path.display());
}

pub fn decode(path: &Path, output_filename: &Path, _ignored_flag: bool) {
    let reader = WavReader::open(path).expect("can't open wav file");
    let mut bytes_raw = Vec::new();

    for sample in reader.into_samples::<i16>() {
        let sample = sample.expect("failed to read sample");
        bytes_raw.extend_from_slice(&sample.to_le_bytes());
    }

    let ext_len = bytes_raw[0] as usize;
    let ext_end = 1 + ext_len;
    let checksum_end = ext_end + 4;
    let length_end = checksum_end + 4;
    let flag_end = length_end + 1;
    let data_start = flag_end;

    let extension = std::str::from_utf8(&bytes_raw[1..ext_end])
        .expect("invalid UTF-8 in extension")
        .trim_matches(char::from(0))
        .trim();

    let stored_checksum = u32::from_le_bytes(bytes_raw[ext_end..checksum_end].try_into().unwrap());
    let data_len = u32::from_le_bytes(bytes_raw[checksum_end..length_end].try_into().unwrap()) as usize;
    let compression_flag = bytes_raw[length_end] != 0;

    let data_slice = &bytes_raw[data_start..data_start + data_len];

    let file_data = if compression_flag {
        println!("decompressing data...");
        decompress(data_slice.to_vec())
    } else {
        data_slice.to_vec()
    };

    let actual_checksum = crc32fast::hash(&file_data);

    if actual_checksum != stored_checksum {
        eprintln!("Checksum mismatch!");
        eprintln!(" Stored: {stored_checksum}");
        eprintln!("Actual: {actual_checksum}");
        panic!("File may be corrupted");
    } else {
        println!("Checksum OK");
    }

    let mut output_file = PathBuf::from(output_filename);
    output_file.set_extension(extension);
    println!("File written to: {}", output_file.display());

    let mut file = File::create(output_file).expect("failed to create output file");
    file.write_all(&file_data).expect("failed to write data");
}

pub fn compress(data: Vec<u8>) -> Vec<u8> {
    Encoder::new().compress_vec(&data).expect("couldn't compress")
}

pub fn decompress(data: Vec<u8>) -> Vec<u8> {
    Decoder::new().decompress_vec(&data).expect("couldn't decompress")
}
