use std::collections::HashMap;

fn main() {
    let mut user = HashMap::new(); // create a hashmap

    let mut user1: HashMap<String, u32> = HashMap::new(); // also create a hashmap 

    user.insert(String::from("Hello"), 34); //insert in hashmap
    user1.insert(String::from("Helo"), 34);

    /*
    {
        Hello: 34,
        Helo: 34,
    }
    */

    let first_user = user.get("Hello"); //returns an option<22> - so gotta pattern match cannot print
    let second_user = user1.get("Helo");

    match first_user {
        // work
        Some(val) => println!("{}", val),
        None => println!(" error "),
    }
    match second_user {
        //throw error cause data is comment out
        Some(val) => println!("{}", val),
        None => println!(" error "),
    }
}

/* Assigment code*/

fn assigment_function(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut new_vec = HashMap::new();
    for (key, value) in vec {
        new_vec.insert(key, value);
    }
    new_vec
}
fn main() {
    let input_vec = vec![(String::from("Hello"), 34), (String::from("Helo"), 34)];
    let res_vec = assigment_function(input_vec);

    println!("{:?}", res_vec);
}
