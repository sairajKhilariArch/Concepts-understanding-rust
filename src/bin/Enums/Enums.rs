// * Enums :
//              ? An  Enum is a type  that represents a set of possible  values .each possible value is called a variant.

//              ? Enum means Enumerate

//              ?




// * Struct variant :
//              ? A struct variant  Stores associated data fields rather than by position.  each piece of data has an associated name....
//              ? struct as a variant for the enum type





#[derive(Debug)]
enum PaymentMethodType{
    CreditCard(credentials),
    DebitCard(credentials),
    PayPal(credentials),
    GooglePay(credentials),
    Cred(credentials),

}
#[derive(Debug)]
struct credentials{
    Name: String,
    AccountNo: u64,
    Email:String,
    Password:String,
}


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

    // println!("{:#?}",fourth_card);

    let customer01_cre = credentials{
        Name: String::from("sairaj"),
        AccountNo:098765432123,
        Email:String::from("cus01@mail.com"),
        Password:String::from("qwertyuiop"),
    };

    let customer01 = PaymentMethodType::Cred(customer01_cre);

    println!("{:#?}",customer01);

}