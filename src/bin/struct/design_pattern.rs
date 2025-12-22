// * Design pattern :
//              ?  A Design pattern is a recommended way to write structure code to solve specific problem 

//              & in and out 
//              & return type self 



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
    

    fn details( self){
        println!("{:#?}",self);
    }
    
    fn change_designation(&mut self,cha:String)-> &mut self {
        self.designation = cha;
        println!("designation has change");
        println!("{}",self.designation);
        self
    }
    
    fn change_age(&mut self,age:String)-> &mut Self {
        self.age = age;
        println!("age has change");
        println!("{}",self.age);
        self
    }

        fn change_name(&mut self,na:String)-> &mut Self {
        self.name = na;
        println!("name has change");
        println!("{}",self.name);
        self
    }

}



fn main(){
    let mut emp1 = Emp{
        name : String::from("sairaj"),
        age : 25,
        designation : String::from("developer"),
    };


    emp1
        .change_designation(String::from("rust developer"))
        .change_age(26)
        .change_name(String::from("Shadow"));

} 