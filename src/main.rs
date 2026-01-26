// fn main() {
//     println!("hello ");
// }
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




