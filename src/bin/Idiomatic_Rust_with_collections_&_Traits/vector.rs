// * Vector :
//              ? Vector is a data structure which is similar to a array where ara is fixed vector is a flexible and dynamic
//              ? A Vector is a collection type for storing homogenous in size...

//              ? Each element is assigned an index position reflecting its place in line. The index starts counting at 0...

//              ? Rust's vector type is Vec.it has one generic which represents the type of the stored elements(Vec<String>).

// * Create a Vector :
//              ? The Vec::new constructor function returns an empty vector.
//              ? The Vec::new function requires a manual type annotation. provide it with the variable or using the turbofish (::<T>) operator...

//              ? Ad soon as code inserts an element into the vector, the compile can infer its generic type ..
//              ? The vec![] macro creates a vector with pre-population elements ...

//  * Methods:
//              ? The PUSH method appends an element to the end of the vector ...
//              ? The INSERT method adds an element at a specific index position...
//              ? The POP method attempts to removes an element by index position .it panics at runtime if the index is invalid...

// * reading  vec Elements:

//              ? Use square brackets to extract a vector element by its index position ..

//              ? Rules of ownership apply .if a type does not implement the copy trait ,use the borrow operator(&) to avoid moving ownership...

// * Get:

//              ?  The get method extracts a vector element by index position . it returns an Option enum ....
//              ? The GET method accepts an index position and returns an OPtion enum . the SOme variant will store a reference to the value ....

//              ? Borrow a slice with the borrow operator, the value square brackets, and a range...

// * Vector Capacity :

//              ? The vector capacity is the maximum number of elements that the vector can contain....

// * Writing a vector Elements :

//              ? Use Square brackets to target an index  position ,then overwrite its value with an equal sign...

//              ? Rust permits one mutable reference to a value at a time ...

//              ? Rust permits any number of immutable reference to a value at a time ....

fn aboutVectors() {
    let robert_downey_jr_movies = Vec::<&str>::new();
    println!("{robert_downey_jr_movies:?}",); // ^ []

    let fruits: Vec<&str> = Vec::new();
    println!("{fruits:?}"); // ^ []

    let movies: Vec<&str> = vec!["iron man", "avengers", "endgame", "dr doom"];
    println!("{movies:?}"); // ^ ["iron man", "avengers", "endgame", "dr doom"]

    let mut movies01: Vec<&str> = vec!["iron man", "avengers", "endgame", "dr doom"];
    println!("{movies01:?}"); // ^ ["iron man", "avengers", "endgame", "dr doom"]

    movies01.push("iron man 2");
    println!("{movies01:?}"); // ^ ["iron man", "avengers", "endgame", "dr doom", "iron man 2"]

    movies01.push("avengers : age of alton");
    println!("{movies01:?}"); // ^ ["iron man", "avengers", "endgame", "dr doom", "iron man 2", "avengers : age of alton"]

    movies01.insert(1, "Iron man 3");
    println!("{movies01:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgame", "dr doom", "iron man 2", "avengers : age of alton"]

    let hi = movies01.pop();
    println!("{movies01:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgame", "dr doom", "iron man 2"]
    println!("{hi:?}"); // ^ Some("avengers : age of alton")

    movies01.remove(5);
    println!("{movies01:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgame", "dr doom"]

    let hii = &movies01[4];
    println!("{hii:?}"); // ^ dr doom

    let hii = &movies01[1..];
    println!("{hii:?}"); // ^["Iron man 3", "avengers", "endgame", "dr doom"]
}

fn match_option_vector() {
    let movies01: Vec<&str> = vec![
        "iron man",
        "Iron man 3",
        "avengers",
        "endgame",
        "dr doom",
        "iron man 2",
        "avengers : age of alton",
    ];
    println!("{movies01:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgame", "dr doom", "iron man 2", "avengers : age of alton"]

    let which_movie = &movies01.get(9);

    match which_movie {
        Some(movie) => println!("you have selected {movie}"),
        None => println!("at this index position their is no movie..."),
    }
}

fn String_ownership_methods() {
    let movie1 = String::from("iron man");
    let movie2 = String::from("Iron man 3");
    let movie3 = String::from("avengers");
    let movie4 = String::from("endgame");
    let movie5 = String::from("dr doom");
    let movie6 = String::from("iron man 2");
    let movie7 = String::from("avengers : age of alton");

    let mut robert_downey_jr_movies = vec![movie1, movie2, movie3, movie4, movie5, movie6, movie7];
    println!("{robert_downey_jr_movies:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgame", "dr doom", "iron man 2", "avengers : age of alton"]

    let target_movie_4 = &robert_downey_jr_movies[3];
    println!("{target_movie_4:?}"); // ^ "endgame"

    robert_downey_jr_movies[3].push_str("EndGame");
    println!("{robert_downey_jr_movies:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgameEndGame", "dr doom", "iron man 2", "avengers : age of alton"]

    robert_downey_jr_movies[3].clear();
    println!("{robert_downey_jr_movies:?}"); // ^ ["iron man", "Iron man 3", "avengers", "", "dr doom", "iron man 2", "avengers : age of alton"]

    robert_downey_jr_movies[3].insert_str(0, "EndGame");
    println!("{robert_downey_jr_movies:?}"); // ^ ["iron man", "Iron man 3", "avengers", "EndGame", "dr doom", "iron man 2", "avengers : age of alton"]

    robert_downey_jr_movies.pop(); // & removed the last index position string
    println!("{robert_downey_jr_movies:?}"); // ^  ["iron man", "Iron man 3", "avengers", "EndGame", "dr doom", "iron man 2"]

    robert_downey_jr_movies[0].pop(); // & removed the last char from 0th index string
    println!("{robert_downey_jr_movies:?}"); // ^  ["iron ma", "Iron man 3", "avengers", "EndGame", "dr doom", "iron man 2"]
}

fn vec_capacity_grow_behind() {
    // vec with capacity.. it grows behind the scene when needed

    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Length:{} , capacity:{}", seasons.len(), seasons.capacity()); // ^ Length:0 , capacity:4

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");

    println!("Length:{} , capacity:{}", seasons.len(), seasons.capacity()); // ^ Length:4 , capacity:4

    println!("{:?}", seasons); // ^ ["Summer", "Fall", "Winter", "Spring"]

    seasons.push("Summer2");

    println!("Length:{} , capacity:{}", seasons.len(), seasons.capacity()); // ^ Length:5 , capacity:8

    println!("{:?}", seasons); // ^ ["Summer", "Fall", "Winter", "Spring", "Summer2"]
}

fn coding_challenge() {
    /*
    Let's model a file system on a computer.

    Define a File struct with a `name` field set to a
    String. Derive a Debug implementation.

    Define a Folder struct with a `name` field set to
    a String and a `contents` field set to a vector of
    File structs. Derive a Debug implementation.

    On the Folder struct...

    Define a `new` constructor function that accepts a
    `name` String. The method should create and return
    a new Folder with that name. For the `contents` field,
    provide a hardcoded empty vector.

    Define a `create_file` method that accepts a `name`
    String. The method should create a new File with that
    name and add it to the end of the `contents` vector.

    Define a `delete_file` method that accepts an `index`
    parameter of type `usize`. The method should remove the
    File at the specified index position from the `contents`
    vector. It should also return the File.

    Define a `get_file` method that accepts an `index`
    parameter of type `usize`. The method should return
    an Option containing a reference to the File at
    that index position.

    In the `main` function, use the `new` function to
    create a Folder instance with a `name` of your choosing.

    Call the `create_file` method two times. Print out
    the Folder in Debug format.

    Delete one of the two files using the `delete_file`
    method. Print out the Folder in Debug format.

    Call the `get_file` method. Use a match statement
    to react to both Option variants. For the Some variant,
    print out the File in Debug format. For the None variant,
    print out the text "There was no file".
    */

    #[derive(Debug)]
    struct File {
        name: String,
    }
    #[derive(Debug)]
    struct Folder {
        name: String,
        contents: Vec<File>,
    }
    impl Folder {
        fn new(name: String) -> Self {
            Self {
                name,
                contents: vec![],
            }
        }

        fn create_file(&mut self, name: String) {
            let file = File { name };
            self.contents.push(file);
        }

        fn delete_file(&mut self, index: usize) -> File {
            self.contents.remove(index)
        }

        fn get_file(&self, index: usize) -> Option<&File> {
            self.contents.get(index)
        }
    }

    fn fn_main() {
        let mut folder = Folder::new("sairaj".to_string());

        folder.create_file("rust.rs".to_string());

        folder.create_file("name.rs".to_string());

        println!("{:#?}", folder); // ^ Folder {
                                   // ^     name: "sairaj",
                                   // ^     contents: [
                                   // ^         File {
                                   // ^             name: "rust.rs",
                                   // ^         },
                                   // ^         File {
                                   // ^             name: "name.rs",
                                   // ^         },
                                   // ^     ],
                                   // ^ }

        folder.delete_file(1);
        println!("{:#?}", folder); // ^ Folder {
                                   // ^     name: "sairaj",
                                   // ^     contents: [
                                   // ^         File {
                                   // ^             name: "rust.rs",
                                   // ^         },
                                   // ^     ],
                                   // ^ }

        match folder.get_file(9) {
            Some(hii) => println!("file name{hii:?}"),
            None => println!("There was no file"),
        }; // ^ There was no file
    }
    fn_main();
}

fn main() {
    let movie1 = String::from("iron man");
    let movie2 = String::from("Iron man 3");
    let movie3 = String::from("avengers");
    let movie4 = String::from("endgame");
    let movie5 = String::from("dr doom");
    let movie6 = String::from("iron man 2");
    let movie7 = String::from("avengers : age of alton");

    let mut robert_downey_jr_movies = vec![movie1, movie2, movie3, movie4, movie5, movie6, movie7];
    println!("{robert_downey_jr_movies:?}"); // ^ ["iron man", "Iron man 3", "avengers", "endgame", "dr doom", "iron man 2", "avengers : age of alton"]

    aboutVectors();

    match_option_vector();

    String_ownership_methods();

    coding_challenge();
}
