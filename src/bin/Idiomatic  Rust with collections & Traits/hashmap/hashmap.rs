// * HASHMAP:
//              ? A hash-map is a collection type that consists of a key value pairs.
//              ? KEYS and VALUES should be of same type and known which type are type before compiling..
//              ? hashmap lives on the heap data as it can grow
//                  ^let mut menu: HashMap<String, f64> = HashMap::new();

//              ? Keys should always be  different  but same type
//              ? values can be same value but same type

//                  ^ eg:
//                  ^ menu.insert(String::from("MILK"), 29.99);
//                  ^ menu.insert(String::from("RICE"), 29.99);
//                  ^ menu.insert(String::from("WATER"), 50.0);

// * from method :
//              ? from method is giving the data when declared with help of from instead of new .....
//              ?

// * remove method:
//              ? The remove method uses key to delete the key value pair
//              ? removes gives the values as the option
//              ? Option ? because,their is a possibility that that key will not be present in the hashmap so it gives SOME variant as a value if present and NONE variant if the value is not present

// * get method : .get()
//              ? Get method get the reference of the keys and output the reference of the value  of the key....
//              ? if option is none is does not give an error it gives none as a option and value....

//      * copied method : .copied()
//                  ? Used to copy the value in the declared variable...if that data type implement the copy trait... as shown in the example

// * insert method: .insert(K,V)
//              ? it inserts the kay value and if the key is present the value is updated to new value

// * entry or_insert method :
//              ? entry is a method used to put a key in a hashmap if the kay is present it will drop

//              ? or_insert is a combine method used with the entry method which holds the value if a key is not present it will insert the value in that entry/ key pair ..

//              ? if a value is already present it will drop ....

use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("MILK"), 29.99);
    menu.insert(String::from("RICE"), 29.99);
    menu.insert(String::from("WATER"), 50.0);

    println!("{menu:#?}"); // ^ {
                           //                                 ^     "WATER": 50.0,
                           //                                 ^     "MILK": 29.99,
                           //                                 ^     "RICE": 29.99,
                           //                                 ^ }

    from_method_hashmap();
    remove_method_hashmap();
    get_method();
    insert_method();
    entry_orinsert_method();
}

fn from_method_hashmap() {
    let data = [("sam", 2), ("light", 4), ("goku", 9), ("naruto", 10)];
    let emp = HashMap::from(data);
    println!("{emp:#?}"); // ^ {
                          //                           ^     "sam": 2,
                          //                           ^     "goku": 9,
                          //                           ^     "naruto": 10,
                          //                           ^     "light": 4,
                          //                           ^ }
}

fn remove_method_hashmap() {
    let data = [("sam", 2), ("light", 4), ("goku", 9), ("naruto", 10)];
    let mut emp = HashMap::from(data);
    println!("{emp:#?}"); // ^ {
                          //                           ^     "sam": 2,
                          //                           ^     "goku": 9,
                          //                           ^     "naruto": 10,
                          //                           ^     "light": 4,
                          //                           ^ }

    let sam = emp.remove("sam");

    println!("{sam:?}"); // ^ Some(2)

    println!("{emp:#?}");
    //                  ^ {
    //                  ^     "naruto": 10,
    //                  ^     "goku": 9,
    //                  ^     "light": 4,
    //                  ^ }
}

fn get_method() {
    let data = [("sam", 2), ("light", 4), ("goku", 9), ("naruto", 10)];
    let emp = HashMap::from(data);
    // println!("{emp:#?}");

    let value1 = emp.get("d");
    println!("{value1:?}"); // ^ None

    let value1 = emp.get("d").copied().unwrap_or(0);
    println!("{value1:?}"); // ^ 0

    let data = [
        (String::from("sam"), String::from("2")),
        (String::from("light"), String::from("4")),
        (String::from("goku"), String::from("9")),
        (String::from("naruto"), String::from("10")),
    ];
    let emp = HashMap::from(data);
    println!("{emp:#?}");

    let value1 = emp.get("sam").cloned().unwrap_or("hello".to_string());
    println!("{value1:?}"); // ^ "2"
}

fn insert_method() {
    let mut hi = HashMap::new();
    hi.insert("hello", 1);
    println!("{:?}", hi.get("hello")); // ^ Some(1)
    hi.insert("hello", 2);
    println!("{:?}", hi.get("hello")); // ^ Some(2)
    hi.insert("hello", 3);
    println!("{:?}", hi.get("hello")); // ^ Some(3)
}

fn entry_orinsert_method() {
    let mut hi = HashMap::new();
    hi.entry("hello");
    println!("{:?}", hi); // ^ {}

    hi.entry("hello").or_insert(2);
    println!("{:?}", hi); // ^{"hello": 2}

    hi.entry("hello");
    println!("{:?}", hi); // ^ {"hello": 2}

    hi.entry("hello").or_insert(3);
    println!("{:?}", hi); // ^ {"hello": 2}
}
