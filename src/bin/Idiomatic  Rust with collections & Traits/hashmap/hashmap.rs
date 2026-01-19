// * HASHMAP:
//              ? A hash-map is a collection type that consists of a key value pairs.
//              ? KEYS and VALUES should be of same type and known which type are type before compiling..
//              ? Hasmap lives on the heap data as it can grow
//                  ^let mut menu: HashMap<String, f64> = HashMap::new();


//              ? Keys should always be  different  but same type
//              ? values can be same value but same type

//                  ^ eg:
//                  ^ menu.insert(String::from("MILK"), 29.99);
//                  ^ menu.insert(String::from("RICE"), 29.99);
//                  ^ menu.insert(String::from("WATER"), 50.0);

// * from method :
//              ? from method is giving the data when declared with help of from insted of new .....
//              ? 


// * remove method:
//              ? The remove method uses key to delete the key value pair
//              ? removes gives the valuse as the option 
//              ? Option ? becs,their is a posibility that that key will not be present in the hasmap so it gives SOME varient as a value if present and NONE vareirnt if the value is not present




use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("MILK"), 29.99);
    menu.insert(String::from("RICE"), 29.99);
    menu.insert(String::from("WATER"), 50.0);

    println!("{menu:#?}");      // ^ {
//                                 ^     "WATER": 50.0,
//                                 ^     "MILK": 29.99,
//                                 ^     "RICE": 29.99,
//                                 ^ }

    from_method_hashmap()



}


fn from_method_hashmap() {
    let data = [("sam", 2), ("light", 4), ("goku", 9), ("naruto", 10)];
    let emp = HashMap::from(data);
    println!("{emp:#?}");  // ^ {
//                           ^     "sam": 2,
//                           ^     "goku": 9,
//                           ^     "naruto": 10,
//                           ^     "light": 4,
//                           ^ }

}

fn remove_method_hashmap(){
    let data = [("sam", 2), ("light", 4), ("goku", 9), ("naruto", 10)];
    let mut emp = HashMap::from(data);
    println!("{emp:#?}");  // ^ {
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