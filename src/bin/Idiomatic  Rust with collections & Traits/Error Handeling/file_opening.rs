// * open file

// * HOW TO OPEN FILE IN RUST:
//              ?




use std::fs::File;
use std::process;

fn main() {
    let file = File::open("hi/story.txt");
    println!("{file:?}");

    with_match()
}

fn with_match() {
    let file = match File::open("hi/fff.txt") {
        Ok(s) => s,
        Err(er) => {
            eprint!("this is the error {er:?}");
            process::exit(1)
        }
    };
    println!("{file:?}")
}
