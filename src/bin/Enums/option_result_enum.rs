// * OPtion Enum:
//              ? THe option enum models a scenario where a type could be a valid where could be a valid value or nothing at all .....


// & Option::None :
//              ? Represents an absent value...

// & Option::Some(T):
//              ? Represents a present value...

// * Result Enum:
//              ? The Result enum 

// * get :
//              ? get is a method used to retrieve the index position data from an array any other type of lists...
//              ? But also it is used when a user want it dynamically not by in build developer  .....

// * Unwrap:
//              ? The unwrap method attempts to extract the associated data out of the Some variant...
//              ? When the data is get from the get method operator unwrap is used to get it out of the packing and give it to the variable.
//              ? As it is got by the some variant of option enum..

// * expect:
//              ? Expect is same as unwrap but with an additional task which is to 
//              ? expect the  error and paste the giving message to the terminal for the error....
//              ? it is used to give an error message to an error and not stop the running application or the compilation process....run time error..




fn wrap_unwrap_get(){

    let marvel_char = [
        String::from("Iron Man"),
        String::from("Thor"),
        String::from("Captain America"),
        String::from("Hulk"),
    ];
    // ? retrieved the index position from the arrays get is used for that
    let im = marvel_char.get(0);

    //? here unwrap it with the unwrap method
    let im_marvel_char = im.unwrap();

    // ? here is a printed output
    println!("{:?}",im);            // ^ Some("Iron Man")
    println!("{}",im_marvel_char);  // ^ Iron Man


    // ? This is a same process as upper code but insted of the unwrap here expect is used to give an expected error if the unwrap task fails...
    let im2 = marvel_char.get(3);
    let im_marvel_char2 = im2.expect("could'nt unwrap");
    println!("{}",im_marvel_char2); // ^ Hulk

    // ? Here you can see that their wan an error as an output for the index position which was not available 
    // ? which get an option enum as none 
    // ? and expect try to unwrap it and could't so gave an written error for the failed process....
    let im3 = marvel_char.get(5);
    let im_marvel_char3 = im3.expect("could'nt unwrap"); // |||||
    println!("{}",im_marvel_char3); // ^ thread 'main' (29709) panicked at src/main.rs:27:31:
                                    // ^ could'nt unwrap
                                    // ^ stack backtrace:
                                    // ^    0: __rustc::rust_begin_unwind
                                    // ^              at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/std/src/panicking.rs:698:5
                                    // ^    1: core::panicking::panic_fmt
                                    // ^              at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/panicking.rs:80:14
                                    // ^    2: core::panicking::panic_display
                                    // ^              at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/panicking.rs:264:5
                                    // ^    3: core::option::expect_failed
                                    // ^              at /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/option.rs:2183:5
                                    // ^    4: core::option::Option<T>::expect
                                    // ^              at /home/gohan/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:970:21
                                    // ^    5: Concepts_understanding_rust::main
                                    // ^              at ./src/main.rs:27:31
                                    // ^    6: core::ops::function::FnOnce::call_once
                                    // ^              at /home/gohan/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
                                    // ^ note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    
}






fn option_for_match(){
        let marvel_char = [
            String::from("Iron Man"),
            String::from("Thor"),
            String::from("Captain America"),
            String::from("Hulk"),
        ];

        let sairaj = marvel_char.get(0);
        which_char(sairaj);
    fn which_char(chare: Option<&String>){
        match chare {
            Option::Some(ch)=>println!("you are {ch} in the marvel character"),
            Option::None=>println!("you are not a marvel character"),
        }
    }
}







fn main()  {
}

