use std::fs::File;
use std::io::{Error, Read};

pub fn read_file(path: String) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut str = String::new();
    file.read_to_string(&mut str)?;

    Ok(str)
}
