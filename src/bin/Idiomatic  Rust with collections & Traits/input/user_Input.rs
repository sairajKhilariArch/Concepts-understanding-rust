// * User Input:
//              ? 



use std::io;

fn main() {
    println!("hello ");

    let mut name = String::new();
    println!("input?");
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("hello b{:?}  xcc", name.trim()),
        Err(message) => println!("there was an error: {message}"),
    };
    let gu: i32 = name.parse().unwrap();
    println!("{}", gu);

    println!("hello ");

    println!("hello ");

    println!("hello ");

    println!("hello ");

    println!("hello ");
}