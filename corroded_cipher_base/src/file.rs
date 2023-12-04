use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

// This function reads binary data from a file and returns a `Result` containing the data.
pub fn read_binary_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    // Open the file for reading.
    let mut file = File::open(path)?;

    // Create a buffer to hold the file's contents.
    let mut buffer = Vec::new();

    // Read the file's contents into the buffer.
    file.read_to_end(&mut buffer)?;

    // Return the buffer.
    Ok(buffer)
}

// This function writes binary data to a new file.
pub fn write_binary_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    // Create the directory and its parent directories if they don't exist.
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }

    // Open the file for writing. It will create the file if it does not exist,
    // or truncate it if it does.
    let mut file = File::create(path)?;

    // Write the binary data to the file.
    file.write_all(data)?;

    // Return an empty tuple wrapped in a `Result` to indicate success.
    Ok(())
}
