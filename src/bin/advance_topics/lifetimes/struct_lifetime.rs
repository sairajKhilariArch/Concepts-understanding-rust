// * Struct Lifetimes : multiple Reference struct

/*

struct TrainTime<'a, 'b> {
    from: &'a str,
    to: &'b str,
}

fn main() {
    let from = String::from("pune");

    let plan = {
        let to = String::from("mumbai");

        let hi = TrainTime {
            from: &from,
            to: &to,
        };
        hi.from
    };

    println!("{}", plan);
}

*/
