/// hi this is a documentation..

// mod inventory;
// mod orders;

// use std::{
//     fmt,
//     io::{self, stdin, stdout},
// };

// use std::collections::*;

use fake::{Fake, Faker};

use my_Pro_ware_house::*;

// use inventory::MANAGER;
// use inventory::products::{Item, ProductCategory}; //multiple at a time
// use orders::MANAGER as Order_MANAGER; // with a convenient name

fn main() {
    // println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our inventory is {}", MANAGER);
    // println!("The manager of our order is {}", orders::MANAGER);
    println!("The manager of our order is {}", Order_MANAGER);
    inventory::talk_to_manager();
    println!(
        "We have total {} square meter of space in inventory...",
        inventory::FLOOR_SPACE
    );

    // let hammer101 = inventory::products::ProductCategory::Hammer;
    let hammer101 = ProductCategory::Hammer;
    println!("this product category is {:?}", hammer101);
    // let hammer2 = inventory::products::Item {
    let hammer2 = Item {
        name: String::from("Hammer one style"),
        category: hammer101,
        quantity: 100,
    };
    println!("{:#?}", hammer2);

    let ladder101 = ProductCategory::Ladder;
    println!("ladder101 is in {:?} product category...", ladder101);
    let ladder2 = Item::new(String::from("Ladder for one style"), ladder101, 100);
    println!(
        "
    name of the product is {0}
    category of the product is {1:?}
    quantity ordered is {2}
    ",
        ladder2.name, ladder2.category, ladder2.quantity
    );

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);
}






