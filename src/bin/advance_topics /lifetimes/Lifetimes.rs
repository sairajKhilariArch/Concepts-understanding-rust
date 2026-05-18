/*
* Lifetimes :
?           The lifetime of a value refers to how long it is valid within the code....

?           A values lifetime is the time during which it exists at a particular memory address....

!           A lifetime is the region of code that a value is alive , which means functional , valid ,useful....
?           A lifetime is the time during which a value exists at a particular memory address....
?           lifetimes exist so that the borrow checker can identify dangling references...
?           A dangling references is a reference to data that does not exist...

* Scopes and Lifetimes:
?           A scope is a region of code belonging to a block (a section between a pair of opening and closing curly braces).
?           A value's lifetime is often connected to its scope ..
?           When an owner reaches the end of its scope,it cleans up the associated data . the value's lifetime ends.....


* References and lifetimes :
?           The referent is the data/value that a reference borrows...
?           A references lifetime ends at the last place in the code where it is used (non-lexical scope).
?           A references's lifetime must be contained within the referent's the source of its data...
?           The reference's lifetime must be contained within the referent's lifetime to avoid a dangling reference....

* Borrow Checker :
?           The Borrow checker is the part of the rust compiler that validates that all borrows (i.e.,references) are  valid.....
?           The Borrow checker treats the end of a reference's lifetime as the last place  it is  used; a reference has non-lexical  scope......


* Lexical :
?           Lexical means lasting until the end of the block .....

* NonLexical :
?           NonLexical means not lasting until the end of the block....

* Dangling References :
?           A Dangling Reference is a reference to data that no-longer exists....


* 'static :
?           the 'static is used with the reference that we can guarantee  that the value will be present till the program ends
?           such as : let a  = 23 here 23 will be present till the program end....

*/

// * Lifetime:
/*

fn select_first_second(one: &[String]) -> &[String] {
    &one[..2]
}

fn main() {
    let names = vec![
        String::from("orange"),
        String::from("mango"),
        String::from("apple"),
    ];
! here you can see if the drop method is use prior to printing the second to terminal the code will not run cause names lifetime will end and its reference will become a dangling reference.....
    let second = select_first_second(&names);
    println!("{:?}", second);
    drop(names);
}
*/

// * 'static
/*
fn say_hello() -> &'static str {
    "Hello"
}

const AGE: i32 = 25;

fn what_is_age() -> &'static i32 {
    &AGE
}

fn main() {
    println!("{}", say_hello()); // ^ Hello
    println!("{}", what_is_age()); // ^ 25
}

*/

// * practice session
/*
use std::vec;

// fn main() {
//     println!("hello ");
// }
fn double_the_length<T>(vec_any_type: &Vec<T>) -> usize {
    let hi = vec_any_type.len() * 2;
    hi
}

fn last_two<T>(vec_any_type: &[T]) -> &[T] {
    let last_two_item = vec_any_type.len() - 2;
    &vec_any_type[last_two_item..]
}

fn first_five<'a>(text: &'a str, announcement: &'a str) -> &'a str {
    println!("{}", announcement);
    &text[..5]
}

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &'a str) -> &'a str {
    if first.contains(target) {
        &first
    } else {
        &second
    }
}

fn main() {
    let hi = vec![1, 2, 3];
    let hi1 = double_the_length(&hi);
    println!("{hi1}");

    let hii = vec![1, 2, 3];
    let hii1 = last_two(&hii);
    println!("{hii1:?}");

    let hiii = first_five("sairaj", "how are you");
    println!("{}", hiii);

    let hiiii = find_string_that_has_content("first", "second", "st");
    println!("{}", hiiii);

}

*/
