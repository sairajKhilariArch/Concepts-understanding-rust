// * RECURSION :

//             ? Recursion is when a functtion call it self.....

//             ? each recursion represents  a seperate ,independent execution of the function .. the caller remains in a pending state..

//             * BASE CASE :
//                         ? A base case is the condition that stops the recursion..

//                         ? the BASE CASE involves some conditional that indicates that the end has been reached...

fn conter_count(x:i32){
    if x== 0 {
        println!("its over"); //! here it will end the fn...
    }else{
        println!("count is on {x}");
        conter_count(x-1) //! here it call it selfs ....
    }
}




fn main(){
    conter_count(10)
}