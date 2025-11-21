
// * RANGE:
            // ? RANGE is a type of 'datatype'
            // ? in which you can give a range in between of character or integer 
            // ! float ,array and other iterable datatype are not included for this type of range

            // ? it is a iterable datatype 

            // ? for range two input: 
                                    //& from 
                                    //& end
                                    //& like : 1..100
                                    // !here 100 in not included
                                    // & 1..=100
                                    // ^ no it is included




fn main(){

    // ~ for integer
    let var_a = 1..100; // !  100 is not included it is only up to 99 
    println!("{var_a:#?}"); //^ 1..100

    for i in var_a {
        println!("{i}, "); // ^ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 
    }

        let var_a = 1..=100; // !  100 is included
    println!("{var_a:#?}"); //^ 1..100

    for i in var_a {
        println!("{i}, "); // ^ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 
    }

    // !character
    // ! if capatial letter starting and ending should be same capital
    // & if small then  starting and ending should be same small
    // ^ no mix allowed
    let var_b = 'a'..='z'; 
    println!("{var_b:#?}"); //^'a'..='z'

    for i in var_b {
        println!("{i}, "); //^ a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,
    }

}