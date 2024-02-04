# MD5 Hash Calculator

## Description

This Rust program calculates the MD5 hash for one or multiple files, providing an efficient solution. It supports both individual files and directories, allowing users to easily generate MD5 hashes for various use cases.

## Features

- **Adaptive Buffering:** Adjusts the buffer size dynamically based on file length to optimize memory usage.
- **Directory Support:** Calculates MD5 hashes for all files in a given directory.
- **Error Handling:** Robust error handling for various file operations, ensuring a smooth user experience.

## Prerequisites

- Rust: Ensure you have Rust and Cargo installed on your system. You can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your_username/md5_hash_calculator.git
   
2. **Change into the project directory:**
    ```bash
    cd r-checksum

## Usage
To calculate the MD5 hash of a file or all files in a directory, run the following command:

    ```bash
    cargo run --release -- /path/to/your/file_or_directory

Replace **/path/to/your/file_or_directory** with the actual path to the file or directory you want to process.

