// fn main() {
//     println!("hello ");
// }

use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("MILK"), 29.99);
    menu.insert(String::from("RICE"), 29.99);
    menu.insert(String::from("WATER"), 50.0);

    println!("{menu:#?}");


    // ? now giving the data when declared with help of from insted of new 
    let data = [("sam", 2), ("light", 4), ("goku", 9), ("naruto", 10)];
    let mut emp = HashMap::from(data);
    println!("{emp:#?}");

    let sam = emp.remove("sam");

    println!("{sam:?}"); // ^ Some(2)

    println!("{emp:#?}");
//                  ^ {
//                  ^     "naruto": 10,
//                  ^     "goku": 9,
//                  ^     "light": 4,
//                  ^ }


}
