// * UNREACHABLE!() :
//                     ? this a macro where a if you want to satisy the need of the compiler...
//                     ? it just  states that if this is used that statement will never be executed but to satisfy the compiler error filling the statement this macro is used .....

fn main() {
    let number = 99;

    match number {
        x if x % 2 == 0 => println!("even"),
        x if x % 2 != 0 => println!("odd"),
        _ => unreachable!(),
    };
}
