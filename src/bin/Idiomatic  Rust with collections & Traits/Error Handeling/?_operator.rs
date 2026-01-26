// * ? operator :
// ? 
/*
 ==================================================================================
    THE RUST "?" OPERATOR: A COMPLETE GUIDE
 ==================================================================================

    WHAT IS IT?
    -----------
    The `?` is called the "Error Propagation Operator".
    It is a piece of syntax sugar that handles errors for you automatically.

    WHAT DOES IT DO?
    ----------------
    When you put `?` at the end of a function call (like `func()?`), it does
    three steps internally:

    1. EVALUATE: It checks the result of the function.
    2. SUCCESS (Ok): If the result is `Ok(value)`, it unwraps the value
       and lets your code continue.
    3. FAILURE (Err): If the result is `Err(error)`, it STOPS your function
       immediately and returns the error to the caller (just like `return Err(e)`).

    WHY USE IT?
    -----------
    Without `?`, Rust code gets very "nested" with `match` statements.
    With `?`, your "happy path" (the code that runs when everything works)
    is flat and easy to read.
*/

use std::fs::File;
use std::io::{self, Read};

// --------------------------------------------------------------------------------
// PART 1: The Main Function
// --------------------------------------------------------------------------------
// Note: We changed main to return Result<(), io::Error>.
// This allows us to use '?' even inside the main function!
fn main() -> Result<(), io::Error> {
    println!("--- STARTING TUTORIAL ---\n");

    println!("1. Running the verbose (ugly) version...");
    // We handle the error manually here just to show it works
    match read_file_verbose() {
        Ok(s) => println!("   Success! Read: {:?}", s),
        Err(e) => println!("   Failed: {}", e),
    }

    println!("\n2. Running the clean (?) version...");
    // We use ? here. If read_file_clean fails, main() returns the error immediately.
    let content = read_file_clean()?;
    println!("   Success! Read: {:?}", content);

    println!("\n--- TUTORIAL COMPLETE ---");
    Ok(())
}

// --------------------------------------------------------------------------------
// PART 2: The "Old Way" (Without ?)
// --------------------------------------------------------------------------------
// This function shows how much work you have to do manually without the operator.
fn read_file_verbose() -> Result<String, io::Error> {
    // We try to open a file that doesn't exist to trigger an error
    let file_result = File::open("non_existent_file.txt");

    // We have to explicitly match to see if it worked
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e), // We have to manually type "return Err"
    };

    let mut contents = String::new();

    // We have to match AGAIN for the reading part
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => return Err(e), // Repeating ourselves...
    }
}

// --------------------------------------------------------------------------------
// PART 3: The "New Way" (With ?)
// --------------------------------------------------------------------------------
// This function does the EXACT same thing as above, but in 3 lines.
fn read_file_clean() -> Result<String, io::Error> {
    // 1. Open the file.
    // If it fails, return the error NOW. If it works, give me the 'file'.
    // Note: We use "Cargo.toml" just so it finds a real file and succeeds.
    let mut file = File::open("Cargo.toml")?; 

    let mut contents = String::new();

    // 2. Read the file.
    // If it fails, return the error NOW.
    file.read_to_string(&mut contents)?;

    // 3. If we got here, everything is good. Wrap it in Ok().
    Ok(contents)
}


/*
 ==================================================================================
    DEEP DIVE: ADVANCED DETAILS "ALL THE STUFF"
 ==================================================================================

    RULE #1: RETURN TYPES MUST MATCH
    --------------------------------
    You can only use `?` in a function that returns a `Result` (or `Option`).
    You cannot use it in a function that returns nothing (void).
    
    // BAD:
    fn do_work() { 
        let f = File::open("foo.txt")?; // ERROR! Function returns nothing.
    }

    // GOOD:
    fn do_work() -> Result<(), io::Error> {
        let f = File::open("foo.txt")?; // OK! Function returns Result.
        Ok(())
    }

    RULE #2: ERROR CONVERSION (THE MAGIC)
    -------------------------------------
    The `?` operator is smarter than a simple `match`. It uses a trait called `From`.
    
    If your function is supposed to return `MyCustomError`, but `File::open` returns
    `io::Error`, the `?` operator will try to automatically convert `io::Error` 
    into `MyCustomError` if you have defined that conversion.
    
    This is why you often see Rust functions that return `Result<T, Box<dyn Error>>`.
    It means "Return any kind of error", and `?` handles the conversion automatically.

    RULE #3: IT WORKS ON OPTION TOO
    -------------------------------
    `?` also works on `Option<T>`.
    - If the value is `Some(x)`, it unwraps x.
    - If the value is `None`, it returns `None` from the function immediately.

    fn get_last_char(text: Option<&str>) -> Option<char> {
        // If text is None, return None. If Some, get string.
        let s = text?; 
        // If chars().last() is None, return None.
        let last = s.chars().last()?; 
        Some(last)
    }

 ==================================================================================
*/