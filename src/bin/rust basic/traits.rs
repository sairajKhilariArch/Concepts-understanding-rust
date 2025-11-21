
// * TRAITS:
            // ? Traits are like a CONTRACT if any one implement is 
            // ?then they have to fill the given methodes 
            // ?but if it is already written in trait then you dont have to specify  if you dont want to 
            
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
