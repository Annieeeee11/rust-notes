/* -----FUNCTION----- */

// normal function
fn sum() {
    println!("hii");
}

//function with agruments and return statement
fn sum(a: u32, b: u32) {
    a+b // if i simple give the last thing with a ; (not make it a statemnet) then function return it automatically
}

//function also have return types -- that can be anything
fn sum(a: u32, b: u32) -> u32 {
    return a+b;
}

//main function
fn main() {
    println!("hii");
}

// pub =>  add if you want to access them from outside.
pub fn xyz() {
    //code
}
