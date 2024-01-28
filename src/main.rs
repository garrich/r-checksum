use md5;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    println!("Hello, world!");
    func("c:\\Users\\garrich\\Downloads\\windirstat1_1_2_setup.exe")
}

fn func(file_path: &str) {
    let f = File::open(file_path).unwrap();
    // Find the length of the file
    let len = f.metadata().unwrap().len();
    // Decide on a reasonable buffer size (1MB in this case, fastest will depend on hardware)
    let buf_len = len.min(1_000_000) as usize;
    let mut buf = BufReader::with_capacity(buf_len, f);
    let mut context = md5::Context::new();
    loop {
        // Get a chunk of the file
        let part = buf.fill_buf().unwrap();
        // If that chunk was empty, the reader has reached EOF
        if part.is_empty() {
            break;
        }
        // Add chunk to the md5
        context.consume(part);
        // Tell the buffer that the chunk is consumed
        let part_len = part.len();
        buf.consume(part_len);
    }
    let digest = context.compute();
    println!("{:x}\t{}", digest, file_path);
}