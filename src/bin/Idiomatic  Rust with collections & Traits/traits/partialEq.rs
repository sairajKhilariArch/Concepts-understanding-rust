/*
* PartialEq Trait:
?           The PartialEq trait establishes equality between two values.....


*/

/*

struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: String, destination: String, time: String) -> Self {
        Self {
            origin,
            destination,
            time,
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

fn main() {
    let a = Flight::new(
        String::from("hi"),
        String::from("hello"),
        String::from("9:00 A.M"),
    );
    let b = Flight::new(
        String::from("hi"),
        String::from("hello"),
        String::from("9:00 A.M"),
    );
    let c = Flight::new(
        String::from("hi"),
        String::from("how"),
        String::from("9:00 A.M"),
    );
    println!("{}", a == b);
}


*/
