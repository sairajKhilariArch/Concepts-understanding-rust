// * FUNCTIONS :
//                 ? function is a sequence and series of code

//                 * parameter :
//                             ? a PARAMETER is a name  for an expeceted input to a function .....
//                             ? can have 0 to many parameter in a single function

//                 * argument :
//                             ? An arguement is the concreate value passed in for a parameter  when the function  is invoked......
//                             ? depends on how many parameter are their in the function

//                 * Return Value type:
//                             ? OUtput of the function
//                             ? after declaring the function parameter
//                             ?        '-> datatype'
//                             ? before decleraing the function  body

//                 * return value:
//                             ? with out explicitly telling the output or saying RETURN you can declair the output statement

//                             ? 1.last statemnet of function should not contain ; semicolen
//                             ? just like shown in the following example
//                             ? eg.   place
//                             ? this place it will return the var place  value as return

//                             ?it does not print you have to store that return in somthing before using like in return_type_fn shown it is stored in the helloo var then that var is used ....

//                 * UNIT :
//                             ? A unit is an emply tuple ,a  tuple without values
//                             eg:  let hello = ();

//                             ? it is a default return type of a function ...
//                             ? there will always be a retun type declared if not declared then a empty unit()'empty tuple' will be the return type by the compiler.....

fn main() {
    open_store("pune"); // ^ pune is a argument for fn open_store
    no_of_store(50);
    store_details("pune", 50);
    let helloo = return_type_fn("hello_hi");
    println!("{helloo}");
}

fn open_store(place: &str) {
    //^ here open_store function is taking the 1 parameter which is string
    println!("store is open now ...");
    println!("in the {place:}")
}

fn no_of_store(number: i32) {
    //^ here no_of_store function is taking the 1 parameter which is integer
    println!("store is open now ...");
    println!("their are total {number:} store open all over")
}

fn store_details(place: &str, number: i32) {
    println!("my store is in {place:}, and their are total {number:} of stores at {place:}") //^ here stores_details function is taking the 2 parameter which is string and integer........
}

fn return_type_fn(place: &str) -> &str {
    place
}

// * function with out name  :
// ? {}
// ? this are the curley braces which are like a name less function
// ? one they are used then they are deleated by the default

// ? after declaring the code the block you have to save it in a veriable so it goes like this

//? eg :
//^ let multiplier = 3
//^ hello  = {
//^     let value = 5+4;
//^     value * multiplier
//^ };
//^ println!("{hello}")
