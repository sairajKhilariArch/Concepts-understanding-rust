
// * match in enum :
//              ? The match keyword is often paired with enums ...
//              ? The match keyword validates that eve enum variant has an associated code block....
//              ? A match pattern can extract the associated data from a tuple variant or a struct variant .....
//              ? A match pattern can match against an exact pieces of associated data....


// * The if let Construct :
//              ? The if let constant executes a block of code if there is a match against a specific enum variant ....
//              ? The if let construct declares variables for the associated data .the block has access to the variables....
//              ? Declare the hardcoded enum variant on the dynamic values on the right-hand side....





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

}
