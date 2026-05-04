use crate::lodging::{Accommodations, Description};

// -----------------------------------------------------------
// ---------------------------------FUNCTIONS-----------------
// -----------------------------------------------------------

pub fn book_for_one_night<T: Accommodations + Description>(entity: &mut T, name: &str) {
    entity.book(name, 1);
}

pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: String)
where
    T: Accommodations + Description,
    U: Accommodations,
{
    first.book(&guest, 1);
    second.book(&guest, 1);
}



