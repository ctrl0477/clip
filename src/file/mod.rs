use std::fs;
use std::io;
use std::path::Path;

/// Reads the contents of a file and returns it as a String.
///
/// # Arguments
///
/// * `path` - The path to the file to read
///
/// # Returns
///
/// * `Result<String, io::Error>` - The file contents or an error
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
