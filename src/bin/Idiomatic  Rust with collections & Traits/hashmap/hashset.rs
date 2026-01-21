// * HASHSET :
//              ? A hash set is a collective type that stoes unique values

//              ? In other languages it is called as the set

//              ? It does not allow dublications...









use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    println!("{:?}", concert_queue); // ^ {}

    concert_queue.insert("molly");
    concert_queue.insert("meg");
    println!("{:?}", concert_queue); // ^ {"molly", "meg"}

    println!("{:?}", concert_queue.contains("molly")); // ^ true
    println!("{:?}", concert_queue); // ^ {"molly", "meg"}

    println!("{:?}", concert_queue.remove("molly")); // ^ true
    println!("{:?}", concert_queue); // ^ {"meg"}

    println!("{:?}", concert_queue.contains("molly")); // ^ false
    println!("{:?}", concert_queue); // ^ {"meg"}

    println!("{:?}", concert_queue.get("molly")); // ^ None
    println!("{:?}", concert_queue.get("meg")); // ^ Some("meg")

    // println!("{:?}", concert_queue.get_or_insert("solly")); // ^ 




    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("molly");
    concert_queue.insert("meg");

    let mut movie_queue: HashSet<&str> = HashSet::new();

    movie_queue.insert("meg");
    movie_queue.insert("stewie");

    println!("{:?}", movie_queue.union(&concert_queue));
    println!("{:?}", concert_queue.union(&movie_queue));

    println!("{:?}", movie_queue.difference(&concert_queue));
    println!("{:?}", concert_queue.difference(&movie_queue));

    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));
    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));

    println!("{:?}", movie_queue.is_disjoint(&concert_queue));
    println!("{:?}", concert_queue.is_disjoint(&movie_queue));

    println!("{:?}", movie_queue.is_subset(&concert_queue));
    println!("{:?}", concert_queue.is_subset(&movie_queue));

    println!("{:?}", movie_queue.is_superset(&concert_queue));
    println!("{:?}", concert_queue.is_superset(&movie_queue));


}
