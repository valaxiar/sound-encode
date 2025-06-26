use hound::WavReader;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn decode(path: &Path, output_filename: &Path) {
    let reader = WavReader::open(path).expect("can't open wav file");
    let mut bytes_raw = Vec::new();

    for sample in reader.into_samples::<i16>() {
        let sample = sample.expect("failed to read sample");
        let bytes = sample.to_le_bytes();
        bytes_raw.extend_from_slice(&bytes);
    }
    let ext_len = bytes_raw[0] as usize;
    let ext_start = 1;
    let ext_end = ext_start + ext_len;
    let checksum_start = ext_end;
    let checksum_end = checksum_start + 4;
    let length_start = checksum_end;
    let length_end = length_start + 4;
    let data_start = length_end;

    let data_length = u32::from_le_bytes(
        bytes_raw[length_start..length_end]
            .try_into()
            .expect("bad data length"),
    ) as usize;

    let extension = std::str::from_utf8(&bytes_raw[1..ext_end])
        .expect("utf8 for extension invalid")
        .trim_matches(char::from(0))
        .trim();
    let mut output_filename_final = PathBuf::from(output_filename);
    output_filename_final.set_extension(extension);

    let stored_checksum = u32::from_le_bytes(
        bytes_raw[checksum_start..checksum_end]
            .try_into()
            .expect("couldn't decode checksum"),
    );
    let file_data = &bytes_raw[data_start..data_start + data_length];
    let checksum = crc32fast::hash(file_data);

    if checksum == stored_checksum {
        println!("Checksum matches!");
        println!(
            "File written to: {}",
            output_filename_final.to_string_lossy()
        );
        let mut out_file =
            File::create(output_filename_final).expect("failed to create output file");
        out_file
            .write_all(file_data)
            .expect("failed to write output file");
    } else {
        eprintln!("Found checksum: {checksum}");
        eprintln!("Stored checksum: {stored_checksum}");
        panic!("Checksums don't match. possibly corrupted")
    }
}
