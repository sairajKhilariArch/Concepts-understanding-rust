// * Struct:
//              ? A Struct (structure) is a container to store its related data...
//              ?rule (community): A struct name first char should be a capitalized one 

//          

//              ? Rust has a three kind of struct :
//                  ? Named field struct 
//                  ? tuple field struct 
//                  ? unit field struct


//              ! debug trait is not implemented to the struct from get go (not given to rust y the rust developer)
//                  & to impl the debug trait to the Struct following command is used (debug trait: developer printing on console)
//                      ^ 

// * Instance:
//              ? An instance is concrete value made from type(struct)
//              ? to access the instance of the coffee 
//                          & struct_instance_name.field_name
//                          ^ eg: latte.name
//                          




//              ! Rules for the methods and arguments:
//                      & 1. IMMUTABLE struct value (self parameter takes ownership)
//                      & 2. MUTABLE struct value  (self parameter takes ownership, has permission to mutate )
//                      & 3. IMMUTABLE refrence to take struct instance (no ownership is moved)
//                      & 4. MUTABLE reference to the struct instance (no ownership is moved has permission to mutate)


// * Associated function:
//              ? Associated function are function which are attached/related to type.....


// * Constructer Function:
//              ? Constructer function is a function that return a new instance of a type.....






#[derive(Debug)]  // debug trait called manually
// & coffee is a struct 
struct Coffee{
    name : String,
    prise:f32,
    is_hot: bool
}

fn main(){
    // & latte is a instance of the coffee struct
    let latte=Coffee{
        name:String::from("latte"),
        is_hot:true,
        prise:130.0
    };
    
    println!("name of the coffee is {0}",latte.name); // ^ name of the coffee is latte 
    

    // !-------------------------------------------------------------------
    // & this is a mutable instance of the coffee struct
    let mut beverages = Coffee{
        name:String::from("pizza"),
        is_hot:true,
        prise:100.0
    };
    
    beverages.name = String::from("panner_capsicum_pizza");
    beverages.is_hot = false;
    beverages.prise = 250.0;
    
    println!("{0},{1},{2}",beverages.name,beverages.is_hot,beverages.prise);  // ^ Panner_capsicum_pizza,false,250
    
    // !-------------------------------------------------------------------
    
    // & debug trait print  manually implemented
    println!("{:?}",beverages);  // ^ Coffee { name: "panner_capsicum_pizza", prise: 250.0, is_hot: false }
    
    // & debug trait pretty print
    println!("{:#?}",beverages); // ^ Coffee {
                                 // ^    name: "panner_capsicum_pizza",
                                 // ^    prise: 250.0,
                                 // ^    is_hot: false,
                                 // ^}
    
    // !-------------------------------------------------------------------
    



    
}