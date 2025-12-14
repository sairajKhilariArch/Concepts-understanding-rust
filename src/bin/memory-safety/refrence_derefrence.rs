// * References:
//                 ? the memory allocator returns a reference , which is an address..
//                 ? the reference points to the memory address of the data
//                 ? think of a parking  lot giving  you a reference (spot"H25") when they park your car..
//                 ? we can store a reference  in a variable in  a rust program . references have a fixed  size, so rust stores them on the stack...

//                 ? create a refrences with borrow operator the & symbol..
//                 ? it works with Heap data..


// * Dereference operator :
//                 ? the dereference operator (*) follows a reference to the original value ..
//                 ? in common operations like printing out a values or invokeing a method , rust will automatically derefrences implenment the copy trait . rust  will create a copy of the refrence and the original one will remains valid .






fn main() {
    let  a = String::from("world");
    let b = &a; 
    println!("hello {}",*b) // ^ hello hello
}
