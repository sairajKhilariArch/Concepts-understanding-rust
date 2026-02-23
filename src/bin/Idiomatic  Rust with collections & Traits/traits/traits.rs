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
?           A Trait object is an instances of the type that implements a particular trait whose methods will be accessed a run time using a features called dynamic dispatch .....

* Constant :
?           Constant is a name for a fixed , immutable value.....
* Associated constant :
?           An associated constant is a constant declare within the trait ..

* Getter Method :
?           A getter method is type of method that retrieves apace of data .it  'gets' a piece of state.....

* Setter Method :
?           A setter method is a method that writes a piece of data ..
?           its 'sets' a piece of state....

* Supertrait :
?           A supertrait is a trait from which another trait inherits functionality....
?           The parent is called the super trait and
?           The child is called a sub trait...

* Associated type:
?           An Associated type is a placeholder for a type that is required within a trait......


*/

// ! this is a example of the simple trait and how trait bound work

/*

use std::{fmt::Debug,collections, collections::HashMap};

trait Accommodations {
    // fn get_description(&self) -> String {
    //     format!("A wonderful place to stay")
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

impl description for Hotel {
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
    //     format!(" plz enjoy  {}'s apartment", self.host)
    // }

    fn book(&mut self, name: &str, night: u32) {
        self.guests.push((name.to_string(), night))
    }
}
impl description for AirBnb {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s apartment", self.host)
    }
}

// ? single trait impl in the fn
// fn book_for_one_night(entity: &mut impl Accommodations, name: &str) {
//     entity.book(name, 1);
// }

// ? multiple trait impl in the fn
// fn book_for_one_night(entity: &mut (impl Accommodations + description), name: &str) {
//     entity.book(name, 1);
// }

// ! Trait Bound :using generics

// fn book_for_one_night<T: Accommodations>(entity: &mut T, name: &str) {
//     entity.book(name, 1);
// }

// ? multiple trait impl in a fn with generics
fn book_for_one_night<T: Accommodations + description>(entity: &mut T, name: &str) {
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
    T: Accommodations + description,
    U: Accommodations,
{
    first.book(&guest, 1);
    second.book(&guest, 1);
}

// * multiple trait bound:
trait description {
    fn get_description(&self) -> String {
        format!("A wonderful place to stay")
    }
}

fn choose_best_place_to_stay() -> impl Accommodations + description + Debug {
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
trait description {
    fn get_description(&self) -> String {
        format!("A wonderful place to stay")
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
    fn summarize(&self) -> String {
        format!(" {} : {} ", self.name, self.get_description())
    }
}

impl<T> Accommodations for Hotel<T> {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}

impl<T: Display> description for Hotel<T> {
    fn get_description(&self) -> String {
        format!(" plz enjoy  {}'s Room", self.name)
    }
}

// -----------------------------------------------------------
// ----------------------------MAIN---------------------------
// -----------------------------------------------------------

fn main() {
    let mut h1 = Hotel::new("sairajkkk");
    println!("{}", h1.summarize());

    let h2 = Hotel::new(String::from("sairajkk"));
    println!("{}", h2.summarize());
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

// * Associated constant

/*
trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
struct income {
    amount: f64,
}

impl Taxable for income {
    fn tax_bill(&self) -> f64 {
        &self.amount * Self::TAX_RATE
    }
}

struct bonus {
    b_amount: f64,
}

impl Taxable for bonus {
    const TAX_RATE: f64 = 0.5;

    fn tax_bill(&self) -> f64 {
        &self.b_amount * Self::TAX_RATE
    }
}

fn main() {
    let wer = income { amount: 10000.00 };
    println!("{:.2}", wer.tax_bill());

    let qwe = bonus { b_amount: 10000.00 };
    println!("{:.2}", qwe.tax_bill());
}

*/

// * Getter Method :

/*

trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64;

    fn tax_bill(&self) -> f64 {
        &self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct income {
    amount: f64,
}

impl Taxable for income {
    fn amount(&self) -> f64 {
        self.amount
    }
}

struct bonus {
    b_amount: f64,
}

impl Taxable for bonus {
    const TAX_RATE: f64 = 0.5;

    fn amount(&self) -> f64 {
        self.b_amount
    }
}

fn main() {
    let wer = income { amount: 10000.00 };
    println!("{:.2}", wer.tax_bill());

    let qwe = bonus { b_amount: 10000.00 };
    println!("{:.2}", qwe.tax_bill());
}

*/

// * Setter Method :

/*

// -----------------------------------------------------------
// ----------------------------taxable---------------------------
// -----------------------------------------------------------

trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&mut self) -> f64; //GETTER METHOD

    fn set_amount(&mut self, amount: f64); // setter method

    fn tax_bill(&mut self) -> f64 {
        &self.amount() * Self::TAX_RATE
    }

    fn double_amount(&mut self) {
        let hi = self.amount() * 2.00;
        self.set_amount(hi);
    }
}

// -----------------------------------------------------------
// ----------------------------income---------------------------
// -----------------------------------------------------------

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    fn amount(&mut self) -> f64 {
        self.amount
    }
    fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }
}

// -----------------------------------------------------------
// ----------------------------bonus---------------------------
// -----------------------------------------------------------

#[derive(Debug)]
struct Bonus {
    b_amount: f64,
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;

    fn amount(&mut self) -> f64 {
        self.b_amount
    }

    fn set_amount(&mut self, amount: f64) {
        self.b_amount = amount;
    }
}


// -----------------------------------------------------------
// ----------------------------MAIN---------------------------
// -----------------------------------------------------------


fn main() {
    let mut wer = Income { amount: 10000.00 };
    println!("{:.2}", wer.tax_bill());

    println!("{:?}", wer.amount());
    println!("{:?}", wer.set_amount(20000.0));
    println!("{:?}", wer.double_amount());
    println!("{wer:?}");

    let mut qwe = Bonus { b_amount: 10000.00 };
    println!("{:.2}", qwe.tax_bill());

    println!("{:?}", qwe.amount());
    println!("{:?}", qwe.set_amount(12345.0));
    println!("{:?}", qwe.double_amount());
    println!("{qwe:?}");
}

*/

// * SuperTrait

/*
// -----------------------------------------------------------------
// ----------------------------Investment---------------------------
// -----------------------------------------------------------------

trait Investment {
    fn amount(&mut self) -> f64; //GETTER METHOD

    fn set_amount(&mut self, amount: f64); // setter method

    fn double_amount(&mut self) {
        let hi = self.amount() * 2.00;
        self.set_amount(hi);
    }
}

// --------------------------------------------------------------
// ----------------------------taxable---------------------------
// --------------------------------------------------------------

trait Taxable: Investment {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&mut self) -> f64 {
        &self.amount() * Self::TAX_RATE
    }
}

// -------------------------------------------------------------
// ----------------------------income---------------------------
// -------------------------------------------------------------

#[derive(Debug)]
struct Income {
    amount: f64,
}
impl Investment for Income {
    fn amount(&mut self) -> f64 {
        self.amount
    }
    fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }
}
impl Taxable for Income {}

// ------------------------------------------------------------
// ----------------------------bonus---------------------------
// ------------------------------------------------------------

#[derive(Debug)]
struct Bonus {
    b_amount: f64,
}
impl Investment for Bonus {
    fn amount(&mut self) -> f64 {
        self.b_amount
    }

    fn set_amount(&mut self, amount: f64) {
        self.b_amount = amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
}
// ------------------------------------------------------------
// ----------------------------QualityTime---------------------
// ------------------------------------------------------------
#[derive(Debug)]
struct QualityTime {
    minutes: f64,
}
impl Investment for QualityTime {
    fn amount(&mut self) -> f64 {
        self.minutes
    }

    fn set_amount(&mut self, minutes: f64) {
        self.minutes = minutes;
    }
}
// -----------------------------------------------------------
// ----------------------------MAIN---------------------------
// -----------------------------------------------------------

fn main() {
    let mut wer = Income { amount: 10000.00 };
    println!("{:.2}", wer.tax_bill());

    println!("{:?}", wer.amount());
    println!("{:?}", wer.set_amount(20000.0));
    println!("{:?}", wer.double_amount());
    println!("{wer:?}");

    let mut qwe = Bonus { b_amount: 10000.00 };
    println!("{:.2}", qwe.tax_bill());

    println!("{:?}", qwe.amount());
    println!("{:?}", qwe.set_amount(12345.0));
    println!("{:?}", qwe.double_amount());
    println!("{qwe:?}");

    let mut qaz = QualityTime { minutes: 180.00 };

    println!("{:?}", qaz.amount());
    println!("{:?}", qaz.set_amount(12345.0));
    println!("{:?}", qaz.double_amount());
    println!("{qaz:?}");
}

*/

// * generics in traits

/*

// -----------------------------------------------------------------
// ----------------------------Investment---------------------------
// -----------------------------------------------------------------

trait Investment<T> {
    fn amount(&mut self) -> T; //GETTER METHOD

    fn double_amount(&mut self) {}
}
// --------------------------------------------------------------
// ----------------------------taxable---------------------------
// --------------------------------------------------------------

trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&mut self) -> f64 {
        &self.amount() * Self::TAX_RATE
    }
}

// -------------------------------------------------------------
// ----------------------------income---------------------------
// -------------------------------------------------------------

#[derive(Debug)]
struct Income {
    amount: f64,
}
impl Investment<f64> for Income {
    fn amount(&mut self) -> f64 {
        self.amount
    }
    fn double_amount(&mut self) {
        self.amount *= 2.0
    }
}
impl Taxable for Income {}

// ------------------------------------------------------------
// ----------------------------bonus---------------------------
// ------------------------------------------------------------

#[derive(Debug)]
struct Bonus {
    b_amount: f64,
}
impl Investment<f64> for Bonus {
    fn amount(&mut self) -> f64 {
        self.b_amount
    }

    fn double_amount(&mut self) {
        self.b_amount *= 2.0
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
}
// ------------------------------------------------------------
// ----------------------------QualityTime---------------------
// ------------------------------------------------------------
#[derive(Debug)]
struct QualityTime {
    minutes: u64,
}
impl Investment<u64> for QualityTime {
    fn amount(&mut self) -> u64 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes *= 2
    }
}
// -----------------------------------------------------------
// ----------------------------MAIN---------------------------
// -----------------------------------------------------------

fn main() {
    let mut wer = Income { amount: 10000.00 };
    println!("{:.2}", wer.tax_bill());
    println!("{:?}", wer.amount());
    println!("{:?}", wer.double_amount());
    println!("{wer:?}");

    let mut qwe = Bonus { b_amount: 10000.00 };
    println!("{:.2}", qwe.tax_bill());
    println!("{:?}", qwe.amount());
    println!("{:?}", qwe.double_amount());
    println!("{qwe:?}");

    let mut qaz = QualityTime { minutes: 180 };
    println!("{:?}", qaz.amount());
    println!("{:?}", qaz.double_amount());
    println!("{qaz:?}");
}
*/

// *

// * Associated type

/*
use std::ops::Add;

#[derive(Debug, Clone)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }
}

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
    let one = Lunch { cost: 20.99 };
    let two = Lunch { cost: 10.99 };

    println!("{:.2}", one.clone() + two.clone());
    println!("{:.2}", add_two_numbers(one.cost, two.cost));
}

*/

//*  Traits Full Example */
/*

use std::{
    clone,
    fmt::{Debug, Display, Formatter},
};

trait Drinkable {
    fn consume(self: &mut Self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("COFFEE ")
            .field("Kind", &self.kind)
            .field("Milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(self: &mut Self) {
        self.ounces = 0
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(self: &mut Self) {
        self.percentage = 0
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    let mut latte = Coffee::new("latte", Milk::Oat, 32);
    println!("{:?}", latte);

    latte.consume();

    println!("{:?}", latte);

    let cappuccino = Coffee::new(String::from("cappuccino"), Milk::Almond, 50);

    let hi = cappuccino.get_data();

    println!("{}", hi);

    let pepsi = Soda::new(100, 66.66, String::from("Cherry"));

    println!("{}", pepsi);

    let mut coke = pepsi.clone();

    println!("{}",coke == pepsi);

    coke.consume();

    println!("{:?}",coke);
}

*/
