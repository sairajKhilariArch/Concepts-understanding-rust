// * Result Enum:
//              ? The Result enum models an evaluation that can produce either a success or an error ...
//              ? The OK variant indicates a success.it stores an associated piece of data of generics type T...
//              ? The Err variant indicates an error .it stores an associated piece of data of generics type E.

//              ?   pun enum Result<T,E>{ok(T),Err(E),}


// * Parse:
//              ? it is same as the expect but it dose't stop the application from running it gives an error for if the wrong data type text is input in it
//              ? same as the Err in the result enum*
fn main() {
    let hi: Result<i8,&str> = Result::Ok(5);
    println!("{hi:?}"); // ^ Ok(5)

    let disaster :Result<&str, &str> = Result::Err("this is not good");
    println!("{disaster:?}"); // ^ Err("this is not good")

    let text = "50";
    let text_as_number =text.parse::<i32>();
    println!("{:?}",text_as_number); // ^ Ok(50)

    let text = "maharashtra";
    let text_as_number =text.parse::<i32>();
    println!("{:?}",text_as_number); // ^ Err(ParseIntError { kind: InvalidDigit })

}