/// this is a product module

use fake::Dummy;


/// enum product category
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}



/// hi struct item 
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}




/// hello item impl
impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}
