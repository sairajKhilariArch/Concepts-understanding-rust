
// * ITERATE :
            // ? TO iterate means to repeat, to do somthing over and over again....

// * LOOP:
            // ? loop is a code blcok which iterate again and agin 

            // ? eg: loop{println!("hello");}

            // ? This iteration in the example it will run untill the memory runs out or code crashesh..


// * BREAK: 
            // ? break keyword is used to break the loop 
            // ?eg: 
                    // ^let mut x = 100;
                    // ^loop{
                        //^ if x == 0{break;}
                        //^ println("{x}");
                        // ^ x-=1;
                    // ^}
            // ? in this example break will stop the loop once the x value becomes 0

// * CONTINUE
            // ? The CONTINUE keyword forces a loop to move to the next iteration..
            // ? it will left that running iteration and and go the next iteration
            // ? eg:
                    // ^ let mut x = 100;
                    // ^ loop{
                    //     ^ x-=1;
                    //     ^ if x == 0{break;}
                    //     ^ if x == 10 {continue;} // ? in this 10 will be not printed as it will be skiped and go to the next loop 
                    //     ^ println!("{x}");
                    // ^}


// * While Loop :
            // ? While loop is a loop which runs until the condition is mate ...
            // ?as long as the condition is true until then it will run  when it false loop will break
            
fn main(){
    let mut x = 100;
    loop{
        x-=1;
        if x == 0{break;}
        if x == 10 {continue;} // ? in this 10 will be not printed as it will be skiped and go to the next loop 
        println!("{x}");
}
}