/*
* Traits :

* Intro to Traits:
?            A trait is a  "A  Distinguishing  quality or characteristic".
?            Consider  a trait  like flight . flight  describes the  quality  of being  able to fly...
?            A trait is a contract that describes functionality that a type  should have...
?            We use the word implement to describe when a type honors a trait's requirement ....
?                    For the previous example , we can say that a candle , a computer screen  and the sun "implement" the illumination trait..
?           A trait definition declares the method(s) that a type implementing that trait must have .....
?                   Method Name
?                   Parameters with type
?                   Return value type

* Traits we have seen Before :
?           The display and debug trait requires a type to define methods for presenting itself as a string .....
?           The clone type requires a clone method to describe a clone method for creating  a duplicate of itself ....


* Implementation :
?           Once we have defined a trait we can implement it on structs and enums.  The type promises to honor the trait's contract....
?                   A type implementing the flight trait promises it can fly...

?           Multiple type can implement the same trait..
?                   A bird and a plane type both implement the flight trait....

?           A type can implement multiple traits.......
?                   A plane can implement both the FLIGHT and STORAGE traits.

* Trait Bound :
?           A trait Bound requires that a generic type implement a specific trait....

* Trait Object : 
?           A Trait object is an instances of the type that implements a perticulat trait whose methods will be accrssed a run time using a features called dinaminc dispatch .....

*/

// ! this is a example of the simple trait and how trait bound work

/*

use std::{fmt::Debug,collections, collections::HashMap};

trait Accommodations {
    // fn get_description(&self) -> String {
    //     format!("A wondeful place to stay")
    // }
    fn book(&mut self, name: &str, night: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: collections::HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accommodations for Hotel {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }

    // fn get_description(&self) -> String {
    //     format!("{} is the pinnacle of the luxury", self.name)
    // }
}

impl Discription for Hotel {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s Room", self.name)
    }
}

#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    fn new(name: &str) -> Self {
        Self {
            host: name.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodations for AirBnb {
    // fn get_description(&self) -> String {
    //     format!(" plz enjoy  {}'s appartment", self.host)
    // }

    fn book(&mut self, name: &str, night: u32) {
        self.guests.push((name.to_string(), night))
    }
}
impl Discription for AirBnb {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s appartment", self.host)
    }
}

// ? single trait impl in the fn
// fn book_for_one_night(entity: &mut impl Accommodations, name: &str) {
//     entity.book(name, 1);
// }

// ? multiple trait impl in the fn
// fn book_for_one_night(entity: &mut (impl Accommodations + Discription), name: &str) {
//     entity.book(name, 1);
// }

// ! Trait Bound :using generics

// fn book_for_one_night<T: Accommodations>(entity: &mut T, name: &str) {
//     entity.book(name, 1);
// }

// ? multiple trait impl in a fn with generics
fn book_for_one_night<T: Accommodations + Discription>(entity: &mut T, name: &str) {
    entity.book(name, 1);
}

// fn mix_and_match<T: Accommodations , U: Accommodations>(
//     first: &mut T,
//     second: &mut U,
//     guest: String,
// ) {
//     first.book(&guest, 1);
//     second.book(&guest, 1);
// }

// * where clause : usage in a generics with multiple traits

fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: String)
where
    T: Accommodations + Discription,
    U: Accommodations,
{
    first.book(&guest, 1);
    second.book(&guest, 1);
}

// * multiple trait bound:
trait Discription {
    fn get_description(&self) -> String {
        format!("A wondeful place to stay")
    }
}

fn choose_best_place_to_stay() -> impl Accommodations + Discription + Debug {
    Hotel::new("Taj hotel")
}

fn main() {
    let mut hotel1 = choose_best_place_to_stay();
    // let mut hotel1 = Hotel::new("Taj hotel");
    // println!("{}", hotel1.get_description());
    book_for_one_night(&mut hotel1, "sairaj");

    // hotel1.book("sairaj", 10);
    println!("{:?}", hotel1);

    let mut airbnb1 = AirBnb::new("Vrindavan");
    // println!("{}", airbnb1.get_description());
    book_for_one_night(&mut airbnb1, "sairaj");

    // airbnb1.book("sairaj", 10);
    println!("{:?}", airbnb1);

    mix_and_match(&mut hotel1, &mut airbnb1, "soham".to_string());
    println!("{:?}", hotel1);
    println!("{:?}", airbnb1);
}


 */

// ! this is a work of the trait with effective generics in the example :

/*


use std::{collections, collections::HashMap, fmt::Debug, fmt::Display};

trait Accommodations {
    fn book(&mut self, name: &str, night: u32);
}
trait Discription {
    fn get_description(&self) -> String {
        format!("A wondeful place to stay")
    }
}
// -----------------------------------------------------------
// ---------------------------------HOTEL--------------------
// -----------------------------------------------------------

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: collections::HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summerize(&self) -> String {
        format!(" {} : {} ", self.name, self.get_description())
    }
}

impl<T> Accommodations for Hotel<T> {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}

impl<T: Display> Discription for Hotel<T> {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s Room", self.name)
    }
}

// -----------------------------------------------------------
// ----------------------------MAIN---------------------------
// -----------------------------------------------------------

fn main() {
    let mut h1 = Hotel::new("sairajkkk");
    println!("{}", h1.summerize());

    let h2 = Hotel::new(String::from("sairajkk"));
    println!("{}", h2.summerize());
    // let h3 = Hotel::new(vec!["sairaj", "hi"]);
    // println!("{}",h3);
}

*/

// * dynamic dispatch trait object

/*
// -----------------------------------------------------------
// ----------------------------MAIN---------------------------
// -----------------------------------------------------------

fn main() {
    let mut h1 = Hotel::new("Ankit");
    let mut a1 = AirBnb::new("sairaj");

    let mut stays: Vec<&mut dyn Accommodations> = vec![&mut h1, &mut a1];

    stays[0].book("prasad", 5);
    stays[1].book("shadow", 10);

    println!("{:#?}", h1);
    println!("{:#?}", a1);
}

*/

// * 

/* */

