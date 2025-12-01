// * dbg! :
            // ? dgb is a macro 
            // ? it helps to pretty print any thing and do all the dirty work
            // ! first it gives file location and file name  then line no. then what  was the dbg
                
                // it is strickly for only developer not for production(end user)


fn main(){
        let mut aa  = ["hello","hi","how","are","you"];

        dbg!(aa);
        // ^ [src\bin\dbgMacro.rs:11:9] aa = [
        // ^ "hello",
        // ^ "hi",
        // ^ "how",
        // ^ "are",
        // ^ "you",
        // ^ ]
}