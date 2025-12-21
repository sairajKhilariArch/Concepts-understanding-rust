//              ! Rules for the methods and arguments:
//                      & 1. IMMUTABLE struct value (self parameter takes ownership) ==> self

//                      & 2. MUTABLE struct value  (self parameter takes ownership, has permission to mutate ) ==> mut self

//                      & 3. IMMUTABLE refrence to take struct instance (no ownership is moved)  ==> &self

//                      & 4. MUTABLE reference to the struct instance (no ownership is moved has permission to mutate)  ==> &mut self



//                      & Associated function: 
//                          ? it is like to creake a new function (like init in python....)


//                      ? multiple impl block for same block

#[derive(Debug)] 

struct Emp{
    name : String,
    age:u32,
    designation: String,
}


impl Emp{
    fn new(name:String,age:u32,designation:String)->Self{
        Self{
            name,
            age,
            designation,
        }
    }
}

impl Emp{
    
    // & 1. IMMUTABLE struct value (self parameter takes ownership)
    fn details( self){
        println!("{:#?}",self);
    }
    
    // & 2. MUTABLE struct value  (self parameter takes ownership, has permission to mutate )
    fn change_designation(mut self,cha:String){
        self.designation = cha;
        println!("designation has change");
        println!("{}",self.designation);
    }
    
    // & 3. IMMUTABLE refrence to take struct instance (no ownership is moved)
    fn details_with_ref(&self){
        println!("{:#?}",self);
    }
    
    // & 4. MUTABLE reference to the struct instance (no ownership is moved has permission to mutate)
    fn change_designation_with_ref(&mut self,cha:String){
        self.designation = cha;
        println!("designation has change");
        println!("{}",self.designation);
    }

}



fn main(){
    let mut emp1 = Emp{
        name : String::from("sairaj"),
        age : 25,
        designation : String::from("developer"),
    };

    emp1.change_designation_with_ref(String::from("python"));

    emp1.details_with_ref();

    // emp1.change_designation(String::from("rust"));

    let emp2 = Emp::new(String::from("soham"),30, String::from("developer"));

    emp2.details_with_ref();

} 