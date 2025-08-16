// OPTION ENUM 

/*
- Option enum was introduce to handle the concept of nullbility in a safe way.
- like javascript -- rust doesnt have the concept of the null it used option enum 

----- -----
- handles null for rust
- use when returned value cannot exist
- Return an option instead of Nulll
*/

// self option enum 

enum Option1 {
    Some(u32),
    None,
}

fn testfunction1(str: String) -> Option1 {
    return Option1::Some(2);
    return Option2::None;
}

fn testfunction2(str: String) -> Option<u32> {
    let mut index = 0;
    for c in str.char() {
        index = index + 1;
        if c == "a"  {
            return Some(index);
        }
    }
    None
}

fn testmain() {
    let ans = testfunction1(String::from("hello"));
    match ans {
        Option1::Some(val) => print("hii {}", val),
        Option1::None => print("hey"),
    }

    let ans2 = testfunction2(String::from("hello"));
    match ans2 {
        Option1::Some(val) => print("hii {}", val),
        Option1::None => print("hey"),
    }
}

// actual option enum 

pub enum Option<T>{
    None, 
    Some(T)
}
pub fn testfuction() {
    let mastring = String::from("hellow this is a string");
    let mut res2 = first_a(mastring);
    match res2 {
        Some(location) => print!("{}", location),
        None=> println!("Nothing found")
    }
    res2 = first_a(String::from("hellow this is 
    string"));
    match res2 {
        Some(location) => print!("{}", location),
        None=> print!("Nothing found")
    }
}