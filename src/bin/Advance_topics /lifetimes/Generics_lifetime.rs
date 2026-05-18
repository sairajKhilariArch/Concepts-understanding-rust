/*
* Generic :
?           A generic is a placeholder for a future type ...
?           A generic add flexibility by not hardcoding an exact type....
?           code can use a variety of types in place of the generic...
!           previously we <T> we were using this as a generic an angled bracket and a name or a character but to state that this generic type is a..
&           Lifetime Generic type <'a> a single inverted comma and a small character that inverted single comma will tell you that it is a lifetime and character will till which generic type it is ......

* Generic Lifetimes vs.Concrete lifetimes..:
?           A Concrete lifetime is the region of code that a value exists in the program  ( the time it lives in its memory address )....
?           A generic lifetime is more abstract. it  is a hypothetical lifetime , a non-specific lifetime , a future lifetime that can vary...
?           we can annotate generic lifetimes in our code ..This enables  functions that are flexible enough to handle varying lifetime.....

&           A concrete lifetime is the region of code that a  value exists in the program , which corresponds to the time that it lives in its memory address..
&           A generic lifetime is a hypothetical lifetime ,a non-specific lifetime , a future lifetime that can vary...



* Lifetime Annotation :
?           A lifetime annotation is a name or label for a lifetime...
?           lifetime annotations don't change the reference's lifetime . They don't affect the logic in sny way..
?           A lifetime annotation is a piece of matadata that we provide to a borrow checker so that it can validate that references are valid....

?           Lifetime annotations declare generic lifetimes..
?           Lifetime annotations identify relationships between reference parameters and reference values....
?           An annotation creates a coupling between the return reference's lifetime and the referent's lifetime....



* Lifetime Syntax I :
?           Use a pair of angle brackets.
?           Assign the lifetime a short name starting with a tick. 'a is common choice...
?           Use the lifetime to mark the connections between references and the return value ....
?           Think of the lifetime as augmenting a type . A type of &str is different from a type of &'a str.


* Lifetime Syntax II:
?           Marking multiple reference parameters with the same lifetime annotating does not mean they have to have an identical lifetime..
?           If there are different concrete lifetimes , the returned reference must live within the overlapping lifetime (the shorter of the two lifetimes )



* Elision :
?           Elision is the act of omitting something . lifetime elision means emitting generic lifetime annotations in situations where the borrow checker can infer the lifetime relationships automatically....

* Elision Rules :
&           First Elision Rule : The Compiler assigns a lifetime to each parameter that is reference...
&           Second Elision Rule : if there is one reference , the borrow checker will infer that their lifetime are related..........
&           Third Elision Rule : in a method definition , if there are multiple references parameter but one of them is Self the borrow checker will check lifetime of the instances is connected to the return value ..... 
?           1. The borrow checker assigns a separate lifetime to each reference parameter.
?           2. If there is one reference parameter and the return value is a reference, the borrow checker infers that the parameter's lifetime will apply to the return value...
?           3. In a method definition, if the parameter is a reference to the instance (&self ) , the borrow checker assigns the lifetime of self (the instance ) to the return value.....


* Lifetimes in struct:
?           A struct can store a reference in a field ..
?           The lifetime of the struct must be connected with lifetime of the field ...
?           The lifetime of the struct must end before the lifetime of the field's referent .
?           Otherwise , the struct's field would be a dangling reference.....



*/


// * Generic Lifetime :

/*

fn select_first_second<'a>(one: &'a [String]) -> &'a [String] {
    &one[..2]
}

fn main() {
    let names = vec![
        String::from("orange"),
        String::from("mango"),
        String::from("apple"),
    ];

    let second = select_first_second(&names);
    println!("{:?}", second);
}

*/ 


// * Elision  :

/*

fn select_first_second<'a, 'b>(one: &'a [String], second: &'b [String]) -> &'a [String] {
    &one[..2]
}

fn main() {
    let names = vec![
        String::from("orange"),
        String::from("mango"),
        String::from("apple"),
    ];

    let names2 = vec![
        String::from("banana"),
        String::from("papaya"),
        String::from("pineapple"),
    ];

    let second = select_first_second(&names, &names2);
    println!("{:?}", second);
}

*/

//* Elision 1st and 2nd rule :

/*
& giving every referenced parameter a generic Elision ......
& and checking that both the parameters lifetime is matched .....
fn bigger_which<'a>(one: &'a String, second: &'a String) -> &'a String {
    if one.len() >= second.len() {
        one
    } else {
        second
    }
}

fn main() {
    let names = String::from("pune");
    let names2 = String::from("Mumbai");
    let second = bigger_which(&names, &names2);
    println!("{:?}", second);
}
*/



