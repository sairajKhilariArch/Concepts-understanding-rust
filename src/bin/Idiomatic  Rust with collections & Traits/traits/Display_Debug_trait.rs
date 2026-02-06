/*
* Display trait :
?           Display Trait is a trait which is to display a text as an output with the help of {} operator .....
?           Display trait is use as convention on the struct for printing the struct how we like ......

?           it is used for the last use readability as they are not the rust developer they will not know if we print the struct with the derive debug formats
?            to use a string print like a normal and code reusability we will implement the display trait directly to the struct with impl
?           as followed in the example........../

* Debug Trait :
?           It is same as the display but for the custimized version for the developer reading...



*/

// *  Display trait for STRUCT

/*


use std::fmt::{Display, Formatter, Result};

// -----------STRUCT----------------------------------------------------
// ----------------------------------APPLE------------------------------
// ---------------------------------------------------------------------

struct Apple {
    kind: String,
    price: i64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ,{}", self.kind, self.price)
    }
}

// ----------------------------------------------------------------------
// ----------------------------------MAIN--------------------------------
// ----------------------------------------------------------------------

fn main() {
    println!("hello ");

    let a1 = Apple {
        kind: "kashmiri".to_string(),
        price: 180,
    };

    println!("{a1}")
}




*/

// *  Display trait for ENUM

/*

use std::fmt::{Display, Formatter, Result};

// ----------ENUM-----------------------------------------------------
// ----------------------------------APPLE-KIND-------------------------
// ---------------------------------------------------------------------


enum AppleKind {
    Japnese,
    Kashmiri,
    Canadien,
}

impl Display for AppleKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleKind::Canadien => write!(f, "Canadien"),
            AppleKind::Japnese => write!(f, "Japnese"),
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

// ----------------------------------------------------------------------
// ----------------------------------MAIN--------------------------------
// ----------------------------------------------------------------------

fn main() {
    println!("hello ");

    let a1 = Apple {
        kind: AppleKind::Japnese,
        price: 180,
    };

    println!("{a1}")
}

*/

// * Debug trait for STRUCT and ENUM 

/*

use std::fmt::{Debug, Display, Formatter, Result};

// ----------ENUM-----------------------------------------------------
// ----------------------------------APPLE-KIND-------------------------
// ---------------------------------------------------------------------

enum AppleKind {
    Japnese,
    Kashmiri,
    Canadien,
}
impl Display for AppleKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleKind::Canadien => write!(f, "Canadien"),
            AppleKind::Japnese => write!(f, "Japnese"),
            AppleKind::Kashmiri => write!(f, "Kashmiri"),
        }
    }
}

impl Debug for AppleKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleKind::Canadien => write!(f, "AppleKind::Canadien"),
            AppleKind::Japnese => write!(f, "AppleKind::Japnese"),
            AppleKind::Kashmiri => write!(f, "AppleKind::Kashmiri"),
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
impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Apple ::: {:?} FOR {}", self.kind, self.price)
    }
}

// ----------------------------------------------------------------------
// ----------------------------------MAIN--------------------------------
// ----------------------------------------------------------------------

fn main() {
    println!("hello ");

    let a1 = Apple {
        kind: AppleKind::Japnese,
        price: 180,
    };

    println!("{a1:?}")
}

*/



