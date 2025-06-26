# **sound-encode**

### A fast and minimal CLI tool written in Rust that encodes any file into a .wav audio file

#### Converts an input file into .wav format while keeping the original file extension and verifying integrity via checksum when decoded.

### Usage:

##### encoding: `sencode e <file-to-encode> <output_file>`

###### note: providing a .wav extension for the output file is purely optional

##### decoding: `sencode d <file-to-decode> <output_file>`

###### note: the original file extension is restored during decoding, so you don't need to specify it for the output file

---

### Building:

###### to build sound-encode:

###### run: `cargo build --release` in the root directory of the project

~~[](https://)~~

### Releases:

###### precompiled binaries for Windows x86_64 and x86_64 Linux can be found [here](https://github.com/valaxiar/sound-encode/releases/)
