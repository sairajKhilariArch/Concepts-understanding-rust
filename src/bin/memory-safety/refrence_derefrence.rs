fn main() {
    let  a = String::from("world");
    let b = &a; 
    println!("hello {}",*b) // ^ hello hello
}






fn main() {
    let mut a = String::from("hello ");
    println!("{}",a);
    a = add_string( a);
    println!("{}",a);
    println!("hello ")
}


fn add_string(mut addstr:String )-> String{
    addstr.push_str("world!!");
    addstr
}