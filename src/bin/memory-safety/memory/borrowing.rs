// * references :
//              ? A references stores the memory address of a value ..
//              ? Borrowing means creating a references...
//              ? references enable the reuse of data  with moving ownership...

// * Immutable  References :
//              ? References are immutable by default ..
//              ? An immutable references does not have permission to modify the original value at the memory address....
//              ? A value can have any number of immutable references ...there is  no risk ..
//              ? immutable references implement the copy  trait rust will create full copy in situations where one is needed (variable assignment ,function parameters ,variable inside array ,etc...).


// * Mutable references :
//              ? An mutable reference has permission to modify the original value at the memory address.
//              ? A value can only have one mutable references at a time 
//              ? Mutable references do not implement the copy trait . Ownership will move on variable reassignment ..
//              ? the compiler understands the reference's lifetime ,which is the time it is being utilized in the program .A lifetime can end before the function's scope ...

// * Dangling references :
//              ? A dangling references is a pointer to a memory address that has been deallocated ...
//              ? Dangling references create bugs and unpredictable behaviors in other programming ...
//              ? The Rust compiler prevents dangling references .A reference is guaranteed to point to valid data ...
//              ? The referent (the original data ) must outlive the reference ...


    


    