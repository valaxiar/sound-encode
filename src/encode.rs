use hound::{self, WavWriter, WavReader};


pub fn encode (input: Vec<u8>) {
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

