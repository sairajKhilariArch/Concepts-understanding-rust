// fn main() {
//     println!("hello ");
// }

use Concepts_understanding_rust::lodging::{Accommodations, AirBnb, Description, Hotel };
use Concepts_understanding_rust::utils::{book_for_one_night, mix_and_match};

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

    book_for_one_night(&mut h1, "soham");
    mix_and_match(&mut h1, &mut a1, "yash".to_string());

    println!("{:#?}", h1);
    println!("{:#?}", a1);
}
