// serde will import these two macros and then you can implement them and no need to worry about the json or others
use serde::{Serialize, Deserialize};
// importing this alone will import the trait 

// doing this will import the macro that do our work 
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

fn main() {
    let s = User{ username: String::from("hiii1") , password: String::from("hii")};

    let res = serde_json::to_string(&s); // this functipn comes witht he derive feature and it converts struct to string

    match res {
        Ok(s) => println!{"{}",res},
        Err(_) => println!{"error"},
    }

    // easy way 
    let res1 = res.unwrap(); // ugly because if the process fail it will create panic and end program


    /* String to struct */
    let s1 = String::from("{\"username\" : \"annie\", \"password\" : \"12345\"}");
    let s1_res: Result<User, serde_json::Error> = serde_json::from_str(&s1);

    match s1_res {
        Ok(s) => println!("{:?}",s),
        Err(_) => println!("error"),
    }
}