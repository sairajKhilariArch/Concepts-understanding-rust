/*
* Closure :
?           A closure is a function without  a name . it is something called as a anonymous function  or a lambda....

?           A Functional programming treats a function like a other value in a program...


& Trait Hierarchy :

* 1. FnOnce :
?           • Closure captures values by move (Transferring ownership) ...
?           • Closure will be invoked once...

!               ^^^^^^^^
!               ||||||||

* 2. FnMut :
?           • Capture value is a mutable reference...
?           • Closure can invoke multiple times..

!               ^^^^^^^^
!               ||||||||

* 3. Fn :
?           • Closure captures value by immutable reference or does't capture anything at all..
?           • Closure can be invoke multiple times...



* Trait    | Captures by                  | Can modify?  | Can consume?  | Callable multiple times?  |
! -------- | ---------------------------- | -----------  | ------------  | ------------------------  |
& `Fn`     | `&T` (immutable reference)   | ❌ No        | ❌ No         | ✅ Yes                    |
& `FnMut`  | `&mut T` (mutable reference) | ✅ Yes       | ❌ No         | ✅ Yes                    |
& `FnOnce` | `T` (ownership)              | ✅ Yes       | ✅ Yes        | ❌ Only once              |










*/

// * Closure :
/*
    println!("hello ");

?    plain  function
    fn product(b: i32) -> i32 {
        b
        return number *b;
?        as you can see 'number' variable is giving an error it is not accepting 'number' cause it is out of the scope
    }

?    closure
    let number = 5;
    let product = |a: i32, b: i32| -> i32 { a * b };
?    but closure is accepting it .....

    println!("{}", product(number, 1));
    println!("{}", product(number, 2));
    println!("{}", product(number, 3));

*/

// * Closure FnOnce :
//?             Here unwrap_or_else() option is using the closure as a else in the method .....
/*
fn main() {
    let fruit_mango = Some("Mango");
    let fruit_none = None;
    let fruit = true;
    let fruit_print = fruit_none.unwrap_or_else(|| {
        if fruit {
            fruit_mango
        } else {
            Some("Not a fruit")
        }
    });
    println!("{:?}", fruit_print);
}


*/

// * Closure  FnMut :
/*
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut add_number = || numbers.push(6);
    add_number();
    add_number();
    println!("{:?}", numbers);
}
*/


// * Closure Heap data Transferring Clone data

/*
 * 
 * fn main() {
 *    let name = String::from("sairaj khilari");
 *    let transfer = || name.clone();
 *    println!("{}",transfer());
 * }

 */


