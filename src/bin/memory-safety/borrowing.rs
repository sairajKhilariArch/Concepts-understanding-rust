// * refrences :
//              ? A refrences stores the memory address of a value ..
//              ? Borrowing means creating a refrences...
//              ? refrences enable the reuse of data  with moving ownership...

// * Immutable  Refrences :
//              ? Refrences are immutable by deffault ..
//              ? An immutable reffrences does not have permission to modify the original value at the memory address....
//              ? A value can have any number of immutable refrences ...there is  no risk ..
//              ? immutable refrences implement the copy  trait rust will create full copy in situations where one is needed (veriable assignment ,function paremeters ,variable inside array ,etc...).


// * Mutable Refrences :
//              ? An muteble refrence has permission to modify the original value at the memory address.
//              ? A value can only have one mutable refrences at a time 
//              ? Mutable refrences do not implement the copy trait . Ownership will move on veriable reassignment ..
//              ? the compiler understands the reference's lifetime ,which is the time it is being utilized in the program .A lifetime can end before the function's scope ...

// * Dangling Refrences :
//              ? A dangling refrences is a pointer to a memory address that has been deallocated ...
//              ? Dangling refrences create bugs and unpredictable behaviors in other programming ...
//              ? The Rust compiler prevents dangling refrences .A reference is guaranteed to point to valid data ...
//              ? The refernt (the original data ) must outlive the reference ...


    


    