// * Enums :
//              ? An  Enum is a type  that represents a set of possible  values .each possible value is called a variant.

//              ? Enum means Enumerate

//              ? Use the enum keyword followed by a name in PascalCase and Curly braces

//              ? separate  variants with  a comma  and a line break...

//              ? By default. custom enums do not implement the copy trait. ownership rules apply...



// * Tuple Variant :
//              ? A variant can store associated data..
//              ? A Tuple Variant stores one or more pieces of data based on  order/position...

//              ? In the variant definition, provide parentheses and the sequential type of the data....
//              ? In the enum instance, provide parentheses and the sequential values for the declared types....




// * Struct variant :
//              ? A struct variant  Stores associated data fields rather than by position.  each piece of data has an associated name....
//              ? struct as a variant for the enum type

//              ? A STRUCT variant stores one or more pieces of data by field name ...
//              ? in the variant definition , provide curly braces and a block ..inside the block ,declared the fields with their associated types ....

//              ? for the enum instance ,provide curly braces and the concrete values for the fields .....


// * Methods:
//              ? Like Struct ,enums can define methods ..
//              ? Use the impl keyword following by the enum name and a block /...
//              ? Methods can take ownership of the enum or borrow it through a reference....
//              ? Methods can define additional parameters after self..Pass those arguments in during invocation.....







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
        Password:String::from("1234567890"),
    };

    let customer01 = PaymentMethodType::Cred(customer01_cre);

    println!("{:#?}",customer01);

}