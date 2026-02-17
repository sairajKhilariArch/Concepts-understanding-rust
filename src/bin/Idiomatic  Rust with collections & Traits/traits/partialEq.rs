/*
* PartialEq Trait:
?           The PartialEq trait establishes equality between two values.....


*/

// * Partialeq on trait
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

// * partialeq on Enum
/*

enum Musician {
    Gitter(String),
    Piano(String),
}
impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Gitter(name) => match other {
                Musician::Gitter(other_name) => name == other_name,
                Musician::Piano(_) => false,
            },
            Self::Piano(name) => match other {
                Musician::Gitter(_) => false,
                Musician::Piano(other_name) => name == other_name,
            },
        }
    }
}
fn main() {
    let a = Musician::Gitter(String::from("a"));
    let b = Musician::Piano(String::from("b"));
    let c = Musician::Piano(String::from("b"));
    let d = Musician::Gitter(String::from("d"));
    println!("{}", a == b); // ^ false
    println!("{}", b == c); // ^ true
    println!("{}", c == d); // ^ false
    println!("{}", d == a); // ^ false
}

*/
