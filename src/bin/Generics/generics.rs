// * Generics:
//              ? A Generic is a type argument ..is's a placeholder for future type...

// * Turbofish Operator:
//              ?   ::<i32>
//              ? thi is hoe we show the built in operator....



#[derive(Debug)]
enum Cheesesteak<T>{
    Plain,
    Topping(T),
}

fn main()  {
    let mushroom =Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping(4);
    println!("{:?}",mushroom);
    println!("{:?}",onions);
}



