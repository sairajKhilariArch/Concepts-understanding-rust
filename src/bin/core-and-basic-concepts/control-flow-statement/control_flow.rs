// * CONTROL FLOW STATEMENT :
                            // ? control flow statement is refer to how a program will flow or execute or say the flow of a code written......

                            // * if :
                                    // ? The if steatement executes ablock when its condition evalutes to true ....

                            // * else :
                                    // ? the else block is a statement block which executes when no other if statement or else if statement will execute
                                    // ? the block can produce a value and the complete if/else if/else construct can be assigned to a veriable..

                            // * else if :
                                    // ? the else if statement executes a block when its condition evalutes to true and the original if statements condition evalutes to false...

                            // * match statements :
                                                
                                                // ? The match statement can react to all possible variants of a value ....
                                                // ? each arm should reaturn a same data type in match statement ......
                                                // ? every arm is ended with a comma if their are no curly brackets are used ..
                                                // ? 

                                                // ? After every statement / arm of a match statement a last arm named default shoulde be written
                                                // ? as it is a arm which will return if no other arm are satisfied like a else statement....
                                                // ? this arm is a last resost so this should be always be written last of every arm..
                                                // ?underscore(_)
                                                // ? eg: _ =>"this is a last statement",

                                                // ? we can check multiple arm in one arm with one output with 
                                                // ? | symbol to seperate the each are like in eg

                            
                            // *refactor :
                                        // ? To refactor means to restructure or improve exxisting code without altering its design .....


fn color_to_number(color: &str)->i32{
    match color{
        red =>1,
        blue =>2,
        green =>3,
        _ => 0,
    }
}

fn main() {
    let number = 99;

    match number {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => println!("ones"),
        10..100 => println!("tens"), // ! here 100 is not include up to 99
        100..1000 => println!("thousands"), // ^ this will include all in between integers from 100 to 1000 but not  1000 
        _ => println!("hello"),
    }
}

