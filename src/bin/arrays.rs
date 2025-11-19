
// * ARRAYS:
            // ? arrays is a fixed-sized collection of homogenous data (same type of data)........
            // ? like int or floats or string 
            // ? arrays are not expandable


fn main(){
    
    // let a = ["hello","hi","how","are","you",5,5.5]; 
    // println!("{:?}",a) // ^ compilation error : expected `&str`, found integer

    let a = ["hello","hi","how","are","you"]; //^ ["hello", "hi", "how", "are", "you"]

    println!("{:?}",a); //^ ["hello", "hi", "how", "are", "you"]

    // * length  : check the length of the array
    println!("{:?}",a.len()); //^ 5

    // *
    
    // let b = a.copy_within(0,3);
    // println!("{:?}",b);







}