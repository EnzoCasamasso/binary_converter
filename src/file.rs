use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
