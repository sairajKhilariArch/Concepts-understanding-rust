
// * match in enum :
//              ? The match keyword is often paired with enums ...
//              ? The match keyword validates that eve enum variant has an associated code block....
//              ? A match pattern can extract the associated data from a tuple variant or a struct variant .....
//              ? A match pattern can match against an exact pieces of associated data....


// * The if let Construct :
//              ? The if let constant executes a block of code if there is a match against a specific enum variant ....
//              ? The if let construct declares variables for the associated data .the block has access to the variables....
//              ? Declare the hardcoded enum variant on the dynamic values on the right-hand side....


//              ^ this is how you show the variant in the enum :
//                  ! | Type           | Syntax                 | Purpose                                   |
//                  ? | -------------- | ---------------------- | ----------------------------------------- |
//                  & | Unit-like      | Free                   | No associated data                        |
//                  & | Tuple variant  | Basic(f64, u32)        | Anonymous positional fields (like tuples) |
//                  & | Struct variant | Premium { tier: Tier } | Named fields (like structs)               |



#[derive(Debug)]
enum Marvel{
    Avengers,
    Fantastic,
    Defenders,
    Shield,
    Hydra,
}

impl Marvel{
    // ? but  this is a marvel enum function only who has the enum variant  he can use this method
    fn in_which_team(name:&Marvel) ->&str{
        match name{
            Marvel::Avengers => "you are in avengers team",
            Marvel::Fantastic =>"you are in the Fantastic 4 team",
            Marvel::Defenders => "you are in the Defenders",
            Marvel::Shield => "you are in the Shield",
            Marvel::Hydra => "you are in the Hydra",
        }
    }

    fn in_avengers_team(self){
        if let Marvel::Avengers = self {
            println!("welcome  to avengers team")
        }else {
            println!("sorry,you are not in avengers team..you are in{:?}",self)
        }
    }

}
// ? this is a separate function and any one can use it
fn in_which_team(name:&Marvel) ->&str{
    match name{
        Marvel::Avengers => "you are in avengers team",
        Marvel::Fantastic =>"you are in the Fantastic 4 team",
        Marvel::Defenders => "you are in the Defenders",
        Marvel::Shield => "you are in the Shield",
        Marvel::Hydra => "you are in the Hydra",
    }
}





#[derive(Debug)]
enum Marvel{
    Avengers,
    Fantastic,
    Defenders,
    Shield,
    Hydra,
}

impl Marvel{
    // ? but  this is a marvel enum function only who has the enum variant  he can use this method
    fn in_which_team(name:&Marvel) ->&str{
        match name{
            Marvel::Avengers => "you are in avengers team",
            Marvel::Fantastic =>"you are in the Fantastic 4 team",
            Marvel::Defenders => "you are in the Defenders",
            Marvel::Shield => "you are in the Shield",
            Marvel::Hydra => "you are in the Hydra",
        }
    }

    fn in_avengers_team(self){
        if let Marvel::Avengers = self {
            println!("welcome  to avengers team")
        }else {
            println!("sorry,you are not in avengers team..you are in{:?}",self)
        }
    }

}
// ? this is a separate function and any one can use it
fn in_which_team(name:&Marvel) ->&str{
    match name{
        Marvel::Avengers => "you are in avengers team",
        Marvel::Fantastic =>"you are in the Fantastic 4 team",
        Marvel::Defenders => "you are in the Defenders",
        Marvel::Shield => "you are in the Shield",
        Marvel::Hydra => "you are in the Hydra",
    }
}





#[derive(Debug)]
enum Tier{
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription{
    Free,
    Basic(f64,u32),
    Premium{ tier: Tier },
}

impl Subscription{
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            }
            Subscription::Basic(price, months) => {
                println!("You have limited access to the site's premium features for {} for {} months", price, months);
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {:?} ",tier);
            }
        }
    }
}




fn main()  {
    // ? for a fn for match variant for variable
    let ironman =Marvel::Avengers;
    let mut team = in_which_team( &ironman);
    println!("{:?}",team);

    // ? implemented same function from impl method
    let fury = Marvel::Shield;
    team = &Marvel::in_which_team(&fury);
    println!("{:?}",team);

    fury.in_avengers_team();
    
    let free_sub = Subscription::Free;
    free_sub.summarize();

    let basic_sub = Subscription::Basic(9.99, 12);
    basic_sub.summarize();

    let premium_sub = Subscription::Premium { tier: Tier::Gold };
    premium_sub.summarize();
}



/*
Define a Tier enum with three variants: Gold, Silver, Platinum. Derive a Debug implementation for the Tier enum.

Define a Subscription enum with three variants: Free, Basic, and Premium. Derive a Debug implementation for the Subscription enum.

The Free variant should have no associated data.

The Basic variant should be a tuple variant with two pieces of data. The first one should be a f64 (the price per month) and the second one should be a u32 (the number of months).

The Premium variant should be a struct variant with a 'tier' field. The tier field should be a Tier enum value.

Define a 'summarize' method on the Subscription enum.

If the Subscription is Free, output the text "You have limited access to the site".

If the Subscription is Basic, output the text "You have limited access to the site's premium features for {price} for {months} months", where {price} amd {months} come from the tuple variant's associated data.

If the Subscription is Premium, output the text "You have full access to the site's premium features. Your tier is {tier:?}"", where {tier} comes from the struct variant's associated 'tier' field.

In the `main` function, create 3 instances of Subscription, each one with a different variant. Invoke the `summarize` method on each instance.
*/
