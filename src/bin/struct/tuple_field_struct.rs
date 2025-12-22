// this is a second type of struct 
// * tuple field struct:
//                  ? A tuple struct is a struct that assigns each piece  of data an order in line rather than a name....








struct long_Duration(u32 , u32);

struct short_Duration(u32,u32);

fn main()  {

    let my_shift = short_Duration(6,9);

    let bros_shift = long_Duration(9,6);

    println!("my shift start at {}pm and end at {}pm.",my_shift.0,my_shift.1 );  // ^ my shift start at 6pm and end at 9pm.

    println!("bro's shift start at {}am and end at {}pm.",bros_shift.0,bros_shift.1); // ^ bro's shift start at 9am and end at 6pm.
}