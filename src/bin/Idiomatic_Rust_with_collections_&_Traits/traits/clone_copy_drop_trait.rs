/*
* Drop :
?           Drop trait is a called when an heap allocated  type of data is freed on
?           When a Heap allocated data is been Removed from the program the Drop Trait is been invoked on every time ......

?           The Drop trait and its drop  method define clean-up execution logic when a heap type is deallocated.....
?           Rust will invoke the drop method when the type is deallocated...
?           Our code cannot manually call the drop method..

* Clone :
?           The Clone Trait models the ability to create a duplicate of an instance ....
?           can use the derive clone as trait if the all fields of the struct implement clone trait.....

?           The Clone trait allows a type to create a duplicate of itself with an explicit call to the clone method ...

*Copy :
?           Copy trait is a child node of the Clone trait so,
?           if you say derive clone then also u can use the copy trait

?           The Copy trait implicitly creates a copy of a type in certain situations (assigning to variable , passing function argument , adding element to array , etc)......
?           The copy trait is a subtrait of the Clone . if a type implements copy , it must implement Clone....


*/

// * Drop trait :
/*

use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::ops::Drop;
// ----------ENUM-----------------------------------------------------
// ----------------------------------APPLE-KIND-------------------------
// ---------------------------------------------------------------------

enum AppleKind {
    Japanese,
    Kashmiri,
    Canadian,
}
impl Display for AppleKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleKind::Canadian => write!(f, "Canadian"),
            AppleKind::Japanese => write!(f, "Japanese"),
            AppleKind::Kashmiri => write!(f, "Kashmiri"),
        }
    }
}

// -----------STRUCT----------------------------------------------------
// ----------------------------------APPLE------------------------------
// ---------------------------------------------------------------------

struct Apple {
    kind: AppleKind,
    price: i64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ,{}", self.kind, self.price)
    }
}

// ? Drop trait implementation on a apple struct
impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("hello.txt") {
            Ok(_) => println!(
                "{} Apple with Rs.{} price has been  removed from program",
                self.kind, self.price
            ),
            Err(rr) => println!("{}", rr),
        }
    }
}

// ----------------------------------------------------------------------
// ----------------------------------MAIN--------------------------------
// ----------------------------------------------------------------------

fn main() {
    println!("hello ");

    let a1 = Apple {
        kind: AppleKind::Japanese,
        price: 180,
    };

    println!("{a1}")
}


*/

// * Clone trait :

/*

use std::clone::Clone;
#[derive(Clone)]
struct Appointment {
    doctor_name: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor_name: String, start_time: String, end_time: String) -> Self {
        Self {
            doctor_name,
            start_time,
            end_time,
        }
    }
}

// impl Clone for Appointment {
//     fn clone(&self) -> Self {
//         Self {
//             doctor_name: self.doctor_name.clone(),
//             start_time: self.start_time.clone(),
//             end_time: self.end_time.clone(),
//         }
//     }
// }

fn main() {
    let first_appointment = Appointment::new(
        String::from("Dr.Makhija"),
        String::from("9:00 A.M"),
        String::from("10:00 A.M"),
    );

    let second_appointment = first_appointment.clone();

    println!(
        "{}'s appointing time start from {} to {}",
        second_appointment.doctor_name, second_appointment.start_time, second_appointment.end_time
    );
}


*/

// * Copy trait :

/*

#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Copy for Duration {}

fn main() {
    let first_hour = Duration::new(1, 0, 0);

    let another_hour = first_hour;

    println!("{another_hour:?}");
}
*/
