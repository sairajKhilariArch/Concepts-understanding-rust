// * Struct:
//              ? A Struct (structure) is a container to store its related data...
//              ?rule (community): A struct name first letter should be a capitalized one 

//          

//              ? Rust has a three kind of struct :
//                  ? Named field struct 
//                  ? tuple field struct 
//                  ? unit field struct


// * Instance:
//              ? An instance is concrete value made from type(struct)
//              ? to access the instance of the coffee 
//                          & struct_instance_name.field_name
//                          ^ eg: latte.name
//                          



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


    // ---------------------------------------------------------------------

    let mut beverages = Coffee{
        name:String::from("pizza"),
        is_hot:true,
        prise:100.0
    };

    beverages.name = String::from("panner_capsicum_pizza");
    beverages.is_hot = false;
    beverages.prise = 250.0;

    println!("{0},{1},{2}",beverages.name,beverages.is_hot,beverages.prise)  // ^ Panner_capsicum_pizza,false,250
}