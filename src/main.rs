use std::{env, process};

use clip::clipboard;
use clip::file;

fn main() {
    // Get the filename from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Read the file contents
    let contents = match file::read_file(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Copy to clipboard
    if let Err(e) = clipboard::copy_to_clipboard(&contents) {
        eprintln!("{}", e);
        process::exit(1);
    }

    println!(
        "Successfully copied contents of '{}' to clipboard",
        filename
    );
}
