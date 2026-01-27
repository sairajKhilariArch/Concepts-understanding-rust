// * opening file and writing file into 

use std::fs;
use std::io::{self, stdin};

fn main() {
    let hi = practice();

    let hiii = match hi {
        Ok(hi) => hi,
        Err(hi) => hi.to_string(),
    };

    println!("{}", hiii);
}

fn practice() -> Result<String, io::Error> {
    println!("What file would you like to write to ? ");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name)?;

    println!("What would you like to write to the file ? ");
    let mut file_to_write_into = String::new();
    stdin().read_line(&mut file_to_write_into)?;

    fs::write(&file_name.trim(), file_to_write_into.trim())?;

    Ok(file_name)
}
