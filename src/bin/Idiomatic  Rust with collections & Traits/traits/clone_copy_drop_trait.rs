/*
* Drop :
?           Drop trait is a called when an heap allocated  type of data is freed on
?           When a Heap allocated data is been Removed from the program the Drop Trait is been invoked on every time ......



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


