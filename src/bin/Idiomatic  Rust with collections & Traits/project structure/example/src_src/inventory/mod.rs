/// this is a inventory module
pub mod products;

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "SAM Inventory";

/// & inline mod/MODULE in a mod/MODULE
/// mod products {
///     #[derive(Debug)]
///     pub enum ProductCategory {
///         Ladder,
///         Hammer,
///     }

///     #[derive(Debug)]
///     pub struct Item {
///         pub name: String,
///         pub category: ProductCategory,
///         pub quantity: u32,
///     }
/// }

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee");
}
