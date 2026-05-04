// * string :
//              ? String is a heap allocated data

// * format :
//              ? format is a macro which works same as println macro but it does not print the statement but returns it ....
//              ? which implement the display trait....
//              ? returns back heap allocated string....

// *  methods:

// * Trim:   .trim()
//              ? Trim is a method of string which is used to remove white space from the string .....
//              ? it removes the space from only front and end of the string ....
//              ^ eg:     let hii = String::from("  ss       sairaj khilari   d          ");
//              ^         println!("{}",hii.trim()); // ss       sairaj khilari   d
//              ? As you can see from the example trim only removes the ending and front white spacing not in between space.....

// & trim_start():
//              ? it only removes the starting /front spacing ....

// & trim_end():
//              ? It only removes the ending space.....

// * to_uppercase():
//              ? It makes the string to uppercase.....

// * to_lowercase():
//              ? It makes the string to lowercase....

// * replace():
//              ? It is use to replace the char or series of char in the given char or series of char

// * split()  : split("")
//              ? It is use to split the string and take that split
//              ? A split fist perimeter is from which will take and see if their is a same char present if present then it will split the string from that char....
//              ? to collect the data all data their is method name collect
//       & collect:
//                     ? It collects the data if any provided from
//              ? no we can store this data in the vac or any other
//               ^ eg:    let hiiii : Vec<&str> =hii.split(" ").collect();
//               ^        println!("{:?}",hiiii); // ^ ["s@ir@j", "", "", "", "", "khil@ri"]
//              ? As you can see it also took the spacing which was present after the split split spacing......

fn main() {
    println!("hello ");

    let mut hii = String::from("    sairaj     khilari    ");

    hii = hii.trim_end().to_string();
    println!("{}", hii); //^     sairaj     khilari

    hii = hii.trim_start().to_string();
    println!("{}", hii); //^ sairaj     khilari

    hii = hii.to_uppercase();
    println!("{}", hii); //^ sairaj     khilari

    let hello = &hii[0..10];
    println!("{}", hello); //^     sairaj

    hii = hii.to_uppercase();
    println!("{}", hii); //^SAIRAJ     KHILARI

    hii = hii.to_lowercase();
    println!("{}", hii); //^sairaj     khilari

    hii = hii.replace('a', "@");
    println!("{}", hii); // ^s@ir@j     khil@ri

    let hiiii: Vec<&str> = hii.split(" ").collect();
    println!("{:?}", hiiii); // ^ ["s@ir@j", "", "", "", "", "khil@ri"]
}
