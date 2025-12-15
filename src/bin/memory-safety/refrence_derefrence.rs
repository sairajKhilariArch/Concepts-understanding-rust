// * References:
//                 ? the memory allocator returns a reference , which is an address..
//                 ? the reference points to the memory address of the data
//                 ? think of a parking  lot giving  you a reference (spot"H25") when they park your car..
//                 ? we can store a reference  in a variable in  a rust program . references have a fixed  size, so rust stores them on the stack...

//                 ? create a references with borrow operator the & symbol..
//                 ? it works with Heap data..


// * Dereference operator :
//                 ? the dereference operator (*) follows a reference to the original value ..
//                 ? in common operations like printing out a values or invoking a method , rust will automatically dereferences implement the copy trait . rust  will create a copy of the reference and the original one will remains valid .






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


fn add_string(mut add_str:String )-> String{
    add_str.push_str("world!!");
    add_str
}