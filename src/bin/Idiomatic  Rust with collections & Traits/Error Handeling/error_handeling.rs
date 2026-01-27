// * Error handling :
//              ? Error handling is the process of dealing with potential  errors from operations that can go wrong.....

// * Recoverable Error :
//              ? Recoverable errors are errors that we can define code to handle..

// * Unrecoverable Error:
//              ? UNRecoverable Errors are errors that cause the program to be unable to process....

// * BackTrace :
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
