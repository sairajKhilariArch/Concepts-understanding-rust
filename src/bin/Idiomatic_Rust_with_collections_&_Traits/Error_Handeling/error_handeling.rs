// * Error handling :
//              ? An error is an operation that goes wrong ......

//              ? Error handling is the process of dealing with potential  errors from operations that can go wrong.....

// * Recoverable Error :
//              ? A RECOVERABLE ERROR  is one  that we can  react  to handle in the code.....
//              ? Recoverable errors are errors that we can define code to handle..

// * Unrecoverable Error:
//              ? An UNRecoverable Error is one that prevents the program from procedding...
//              ? UNRecoverable Errors are errors that cause the program to be unable to process....

// * BackTrace :
//              ? The BackTrace  is the list of functions that were running at the point that the error occurred.......
//              ? Backtrace is the list of files and function that were running at the point that the error occurred..

// ! error handeling with the ?  operator

fn main() {
    let mut hi = vec!["hellow", "lion", "tiger"];
    println!("{:?}", hhh(&mut hi)); // ^ Some(5)1
    println!("{:?}", hhh(&mut hi)); // ^ Some(4)
    println!("{:?}", hhh(&mut hi)); // ^ Some(6)
    println!("{:?}", hhh(&mut hi)); // ^ None
}

fn hhh(mut iii: &mut Vec<&str>) -> Option<usize> {
    let so = iii.pop()?;
    Some(so.len())
}

// * Exit function :
//              ? The PROCESS::exit  function terminates  a program.......
//              ? The function accepts a code  parameter that must be a integer ...
//              ? A error  of 0 indicaters the progra exited with out an error..
//              ? Any number  grater than 0 indicates that the program encountered an error....

// * Standard  error :
//              ? The println! macro outputs a message to the standard  output....
//              ? The eprintln! macro outputs a message to the standard error.....
//              ? Standard error is another Strem/Channel to send message to ..
//              ? Code can log to  different streams (standard output vs. standard error ) to segment messages....


// * Propagating Errors :
//              ? A common pattern is to define a function that sends a Result back to the caller at any point of error..
//              ? The Err varient can store different error massages based on what went wrong...
//              ? To propagate an error means to send it back up to the caller...
//              ? The caller can then customize how to react to the error...