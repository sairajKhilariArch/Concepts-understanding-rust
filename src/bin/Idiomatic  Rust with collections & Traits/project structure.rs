// * packages:
//              ? Our previous Rust program al lived in a single main.rs file....
//              ? When we run the cargo new command , we create a rust package....
//              ? A Package is a folder with a cargo.toml file. The Cargo.toml file holds metadata about the package like its name and version....

// * crates :
//              ? A package is a collection of one or more crates...
//              ? A crate is a collection of rust code that produces an executable or a library....
//              ? A crate is the smallest amount of code that the Rust compiler considers at a time....

// * Types of Crates :

//  * 1.Binary Crate:

//              ? A BINARY CRATE is a crate that compiles to an executable...
//              ? A binary crate has a main function that is the entrypoint for the executable...


// * 2.Library Crate:

//              ? A LIBRARY CRATE exports functionality for other Rust programs to share and use...
//              ? A library crate does not have a main function and does not compile to be  an executable program.......



// * Our Project:

//              ? The cargo new command will default to creating a package with a binary crate...

//              ? The cargo.toml file's name field sets the name of the package..
//                  ? Our package name is warehouse


//              ? Cargo will look for a src/main.rs file .if it exists, Rust infers that we have a binary crate named warehouse...

//              ? Cargo will look for a src/lib.rs file .if it exists, Rust infers that we  have a library crate named warehouse....



// * Module :
//              ? A module is an organizational container that encapsulates related code...

// *  Ways to declare module:

//              ? 1st:in the main file
//                  ? you can declare it in the main file with mod in front of that module name with the module contents...
//                          ^ eg: mod hello{"here is the all code for the module"}

//              ? 2nd: separate file same folder /across multiple files
//                  ? you can declare it in the folder of the main file with the file name of module with .rs extension
//                  ? you don't have to declare the {} block of cde in the main file it should be save in that module file with out that {}....
//                          ^ eg: hello.rs      src/main.rs    //in same folder as main.rs file
//                          ^       contains will be same but not  in {}


//              ? 3rd : dir for mod
//                  ? you can declare it in the folder of the of its own with its name of module with with file name mod.rs containing all the related code
//                  ? you don't have to declare the {} block of cde in the main file it should be save in that module file with out that {}....
//                          ^ eg: src/hello/mod.rs      src/main.rs   //in same folder as main.rs file with separate folder
//                          ^       contains will be same but not  in {}





// * Crate root :
//              ? A Crate Root is the base/foundation of a carate (the starting point for the compiler)....

