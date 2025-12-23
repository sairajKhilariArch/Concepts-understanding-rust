// * Enums :
//              ? An  Enum is a type  that represents a set of possible  values .each possible value is called a variant.

//              ? Enum means Enumerate

//              ?







#[derive(Debug)]

enum CardType{
    Harts,
    Spade,
    Diamond,
    Clubs,
}
#[derive(Debug)]
enum CardNumber{
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Debug)]
struct Card {
    number: CardNumber,
    suit: CardType,
}
fn main()  {

    let first_card = CardType::Harts;

    let mut second_card = CardType::Clubs;

    second_card = CardType::Diamond;

    let third_card = CardNumber::Num(5);
    // println!("{:?}",third_card);

    let fourth_card = Card{
        number:CardNumber::Num(2),
        suit:CardType::Clubs
    };

    println!("{:#?}",fourth_card);


}