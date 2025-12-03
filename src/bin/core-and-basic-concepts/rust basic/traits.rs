
// * TRAITS:
            // ? Traits are like a CONTRACT if any one implement them 
            // ?then they have to fill the given methodes with (require code) 
            // ?but if it is already written((require code)) in trait then you do't have to specify  if you dont want to 
            
            // ! their are built in traits also like (debug , copy , clone ,etc...)


// * Default or built in traits are written like this 
// * EXample...


// ? Employee struct made 
struct Employee {
    salary: f64,
}

// ? BonusEligible trait made to implement it on employee
trait BonusEligible {
    fn bonus_apply(&self) -> String;
}

// ? BonusEligible trait implemented on employee
impl BonusEligible for Employee {
    fn bonus_apply(&self) -> String {
        format!("Bonus: {}", self.salary * 0.1)
    }
}

fn main() {
    let emp = Employee { salary: 50000.0 };
    println!("{}", emp.bonus_apply()); // Output: "Bonus: 5000"
    
}



// ? traits establish consistency between types;methods that represent the same behaviour have the dame name
// ? when a type opts in to honoring a trait's requiements , we say the type implements the tratis.
// ? types can vary in their implementation but still implement the trait.

// ? types can vary in their implementation but still implement the same trait...

// ? a type can choose to opting in to implementing a trait..

// ? A trait can  implement multiple  traits there are hundreds of traits available in rust .

// ? A trait is called  an interface or protocal  in other languages...

// ? eg  a built in traits such as DISPLAY trait




// & SUMMERY :  traits are like contracts which staits that if you implement them then they will provide you with certain things as the output is define but not the input  .....