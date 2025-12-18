// * The str type :
//                  ? Rust has 2 core strings types...
//                  ? the first type is a string literal , also called a string slice .this is the str version..
//                  ? creates a string slice with a pair of double quotes and a hardcoded values ...
//                  ? String Literals are embedded directly into the binary ,the executable file that the rust compilers produces from our source code...


// * The String type 1:
//                  ? The String type lives on the heap and supports dynamic text . it can grow and shrink in size ...
//                  ? The String type does not implement the copy trait..
//                  ? therefore ,⁡⁣⁣⁢ownership⁡ moves from one owner to another when we assign a String to another variable or pass it into function..

// * The string type 2 :
//                  ? the clone method creates a duplicates of a value ..
//                  ? cloning creates a separate ,independent copy of the values ,so ownership does not move ...
//                  ? The drop function invalidates a name and deallocates the corresponding heap memory ...
//                  ? Rust calls drop automatically at the end of the scope ..

//  				? each english char contains 1 bytes as space in string... but
//  				? where as if we choose a different character like emojis or a different language  character it will take as per the character requires but for english character they are as per ascii so they take 1 bytes as shown in example....





fn main() {
    let one = "abcd😊😂🤣❤😍 ";
    let two = & one[0..23]; 
    let three = one.len();  
    println!("{three}"); // 24

}