use md5;
use std::env;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let file_path = &args[1];
        calc_hash(file_path)
    }else {
        let file_path = &args[0];
        println!("Please provide path to a file for md5 hash calculation. \nAs no path provided - self file is used\n{}", file_path)
    }
    
}

fn calc_hash(file_path: &str) {
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

    let digest  = context.compute();
    println!("{:x}\t{}", digest, file_path);
}