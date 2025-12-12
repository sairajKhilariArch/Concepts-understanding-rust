fn main() {
    let  a = String::from("world");
    let b = &a; 
    println!("hello {}",*b) // ^ hello hello
}
