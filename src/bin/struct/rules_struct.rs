//              ! Rules for the methods and arguments:
//                      & 1. IMMUTABLE struct value (self parameter takes ownership) ==> self

//                      & 2. MUTABLE struct value  (self parameter takes ownership, has permission to mutate ) ==> mut self

//                      & 3. IMMUTABLE refrence to take struct instance (no ownership is moved)  ==> &self

//                      & 4. MUTABLE reference to the struct instance (no ownership is moved has permission to mutate)  ==> &mut self



#[derive(Debug)] 

struct Emp{
    name : String,
    age:u32,
    designation: String,
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


} 