pub mod inventory;
pub mod orders;

// use fake::{Fake, Faker};

pub use inventory::MANAGER;
pub use inventory::products::{Item, ProductCategory}; //multiple at a time
pub use orders::MANAGER as Order_MANAGER; // with a convenient name
