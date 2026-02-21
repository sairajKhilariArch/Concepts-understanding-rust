/*
* PartialEq Trait:
?           The PartialEq trait establishes equality between two values.....

* PartialOrd trait: ( Partial order trait)
?           THe PartialOrd trait indicates that a type can be ordered/sorted.

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

//* PartialOrd on trait
/*
use std::cmp::Ordering;
struct Job {
    title: String,
    salary: u128,
}

impl Job {
    fn new(title: String, salary: u128) -> Self {
        Self { title, salary }
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //?
        self.salary.partial_cmp(&other.salary)

        // ? Match Approach
        // match self.salary.partial_cmp(&other.salary){
        //     Some(Ordering::Equal) => Some(Ordering::Equal),
        //     Some(Ordering::Greater) => Some(Ordering::Greater),
        //     Some(Ordering::Less) => Some(Ordering::Less),
        //     None => None
        // }

        // ? if else approach
        //     if self.salary == other.salary {
        //         Some(Ordering::Equal)
        //     } else if self.salary < other.salary {
        //         Some(Ordering::Less)
        //     } else if self.salary > other.salary {
        //         Some(Ordering::Greater)
        //     } else {
        //         None
        //     }
    }
}

fn main() {
    let first_job = Job::new(String::from("DEV"), 100000);
    let second_job = Job::new(String::from("HR"), 80000);
    let third_job = Job::new(String::from("DESIGNER"), 100000);

    println!("{}", first_job == third_job); // ^ true
    println!("{}", first_job < third_job); // ^ false
    println!("{}", first_job > third_job); // ^ false
    println!("{}", first_job <= third_job); // ^ true
    println!("{}", first_job >= third_job); // ^ true

    println!("{}", first_job == second_job); // ^ false
    println!("{}", first_job > second_job); // ^ true
    println!("{}", first_job < second_job); // ^ false
    println!("{}", first_job >= second_job); // ^ true
    println!("{}", first_job <= second_job); // ^ false
}

*/
