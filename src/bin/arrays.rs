

// * COMPOUND TYPE :
                    // ? COLLECTION OF MULTIPLE VALUES

// * ARRAYS:
            // ? arrays is a fixed-sized collection of homogenous data (same type of data)........
            // ? like int or floats or string 
            // ? arrays are not expandable it is a fixeed size
            // ? 


fn main(){
    
    // let a = ["hello","hi","how","are","you",5,5.5]; 
    // println!("{:?}",a) // ^ compilation error : expected `&str`, found integer

    let mut aa  = ["hello","hi","how","are","you"]; //^ ["hello", "hi", "how", "are", "you"]

    println!("{:?}",aa); //^ ["hello", "hi", "how", "are", "you"];

    // ? length  : check the length of the array
    println!("{:?}",aa.len()); //^ 5

    // ?
    
    // let b = a.copy_within(0,3);
    // println!("{:?}",b);


    // !  Cant change the size of array BUT!!!!!!!
                // ? can change the items in the array
                

    println!("{:?}",aa);
    aa[0] = "Hello!....";
    println!("{:?}",aa);


}