// * TUPLE :
            // ? tuple is a fixed, immutable, mixed types
            
            // ! to acess the data use "."

            // ?pythons tuple is similar to the rusts tuple 

            // ? index starts with 0

            // ~ pythons tuple == rusts tuple

            // & | Python | Rust  | Mutable? | Same Types? |
            // & | ------ | ----- | -------- | ----------- |
            // & | list   | Vec   | ✅ Yes   | Usually     |
            // & | tuple  | tuple | ❎ No    | No          |

            
fn main(){

    let t_uple = ("hi","hello","how","are","you",(2,4,6,8,10));
//&index position(  0     1      2      3    4  5:(0,1,2,3,4))


    let _a =t_uple.5.3; //^ 8

    let (a,b,c,d,e,f) = t_uple;

    println!("{}",a);    //^ hi
    println!("{}",b);    //^ hello
    println!("{}",c);    //^ how
    println!("{}",d);    //^ are 
    println!("{}",e);    // ^ you
    println!("{:#?}",f); // ^ (
                         // ^     2,
                         // ^     4,
                         // ^     6,
                         // ^     8,
                         // ^     10,
                         // ^ )
    print!("");
}