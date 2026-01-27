// * open file

// * HOW TO OPEN FILE IN RUST:

//              ? directly
//              ~ let file = File::open("hi/story.txt");
//              ~ println!("{file:?}");

// ! fs::read_to_string method is different from the File:::read_to_string method

// * Asking the user for input: and reading the file

// ?            with the functional way

// ~ use std::fs::File;
// ~ use std::io::{stdin, Read};
// ~ use std::process;

// ~ fn user_input_file_name() -> String {
// ~     println!("please give me the name of the file you want to read:");
// ~     let mut input = String::new();
// ~     if let Err(error) = stdin().read_line(&mut input) {
// ~         eprintln!("input error: {error}");
// ~         process::exit(1);
// ~     }
// ~     input.trim().to_string()
// ~ }

// ~ fn with_match(path: &str) -> File {
// ~     let file = match File::open(path) {
// ~         Ok(f) => f,
// ~         Err(err) => {
// ~             eprintln!("error opening file: {err}");
// ~             process::exit(1);
// ~         }
// ~     };
// ~     println!("file opened successfully: {file:?}");
// ~     file
// ~ }

// ~ fn main() {
// ~     println!("hello ");
// ~     let filename = user_input_file_name();
// ~     let mut file = with_match(&filename);
// ~     let mut file_lines = String::new();
// ~     if let Err(err) = file.read_to_string(&mut file_lines) {
// ~         eprintln!("error reading file: {err}");
// ~         process::exit(1);
// ~     }
// ~     println!("{file_lines}");
// ~ }

// * in  a functional way but with minummem programming

use std::fs::File;
use std::io::{self, stdin, Read};

fn main() {
    match read_file() {
        Ok(content) => println!("File contents:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("please give me the name of the file you want to read:");
    let mut input = String::new();

    if let Err(error) = stdin().read_line(&mut input) {
        return Err(error);
    }

    let filename = input.trim();

    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(err) => return Err(err),
    };

    let mut file_lines = String::new();

    if let Err(err) = file.read_to_string(&mut file_lines) {
        return Err(err);
    }

    Ok(file_lines)
}

// * using the ? operator insted of match for return :


// ~
// ~ use std::fs::File;
// ~ use std::io::{self, stdin, Read};
// ~
// ~ fn main() {
// ~     match read_file() {
// ~         Ok(content) => println!("File contents:\n{}", content),
// ~         Err(e) => eprintln!("Error reading file: {}", e),
// ~     }
// ~ }
// ~
// ~ fn read_file() -> Result<String, io::Error> {
// ~     println!("Please give me the name of the file you want to read:");
// ~
// ~     let mut input = String::new();
// ~     // 1. usage of ? to propagate read_line errors
// ~     stdin().read_line(&mut input)?;
// ~
// ~     // 2. Trim the newline off the input
// ~     let filename = input.trim();
// ~
// ~     // 3. usage of ? to propagate file opening errors
// ~     let mut file = File::open(filename)?;
// ~
// ~     let mut file_lines = String::new();
// ~     // 4. usage of ? to propagate reading errors
// ~     file.read_to_string(&mut file_lines)?;
// ~
// ~     Ok(file_lines)
// ~ }


// * with fs::read_to_string method

use std::fs;
use std::io::{self, stdin};
fn main() {
    match read_file() {
        Ok(content) => println!("File contents:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please give me the name of the file you want to read:");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())
}



