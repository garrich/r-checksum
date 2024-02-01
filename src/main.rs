use md5;
use std::{env, io};
use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader},
};

// Decide on a reasonable buffer size (100MB in this case, fastest will depend on hardware)
const BUFF_SIZE: u64 = 100_000_000;

struct Command {
    file_path: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Command;
    match parse_input(&args) {
        Ok(parsed_command) => {
            command = parsed_command;
            println!("Calculating for total: \t{:x}", command.file_path.len());
            calc_hash(&command.file_path);
        }
        Err(err) => eprintln!("Error parsing user input: {}", err),
    }    
}

fn calc_hash(file_path_arr: &Vec<String>) {
    for file in file_path_arr.iter() {
        calc_hash_of_file(file)
    }
}
/// Calculates the MD5 hash of a file and prints the result.
///
/// # Arguments
///
/// * `file_path` - The path to the file.
///
/// # Example
///
/// ```
/// let file_path = "/path/to/your/file";
/// calc_hash_of_file(file_path);
/// ```
fn calc_hash_of_file(file_path: &str) {
    // Attempt to open the file
    match File::open(file_path) {
        Ok(f) => {
            // Attempt to retrieve file metadata
            match f.metadata() {
                Ok(metadata) => {
                    let len = metadata.len();
                    let buf_len = len.min(BUFF_SIZE) as usize;
                    let mut buf = BufReader::with_capacity(buf_len, f);
                    let mut context = md5::Context::new();

                    // Read file in chunks and compute MD5 hash
                    loop {
                        match buf.fill_buf() {
                            Ok(part) => {
                                if part.is_empty() {
                                    break;
                                }
                                context.consume(part);
                                let part_len = part.len();
                                buf.consume(part_len);
                            }
                            Err(err) => {
                                eprintln!("Error reading file: {}", err);
                                return;
                            }
                        }
                    }

                    // Compute and print the MD5 hash
                    let digest = context.compute();
                    println!("{:x}\t{}", digest, file_path);
                }
                Err(err) => eprintln!("Error getting file metadata: {}", err),
            }
        }
        Err(err) => eprintln!("Error opening file: {}", err),
    }
}

fn parse_input(user_input: &Vec<String>) -> Result<Command, io::Error> {
    let mut file_path_arr: Vec<String> = Vec::new();
    let initial_file: &String;

    if user_input.len() > 1 {
        initial_file = &user_input[1];

        if let Ok(metadata) = fs::metadata(initial_file) {
            let is_directory = metadata.is_dir();
            println!("Is directory: {}", is_directory);

            if is_directory {
                match list_files_in_directory(initial_file) {
                    Ok(files) => {
                        file_path_arr = files;
                    }
                    Err(err) => eprintln!("Error listing files: {}", err),
                }
            } else {
                file_path_arr.push(initial_file.clone());
            }
        } else {
            eprintln!("Error getting metadata for file: {}", initial_file);
        }
    } else {
        initial_file = &user_input[0];
        file_path_arr.push(initial_file.clone());
        println!("Please provide a path to a file for md5 hash calculation.\nAs no path provided, the self file is used:\n{}", initial_file);
    }

    Ok(
        Command {
        file_path: file_path_arr,
    })
}

fn list_files_in_directory(directory_path: &str) -> Result<Vec<String>, io::Error> {
    let entries = fs::read_dir(directory_path)?;

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.to_string_lossy().into_owned();
            files.push(file_name);
        }
    }

    Ok(files)
}
