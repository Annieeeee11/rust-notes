/* -----FUNCTION----- */

///main function
fn main() {
    println!("hii");
}

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

// pub =>  add if you want to access them from outside.
pub fn xyz() {
    //code
}

// Functions that dont return a value -- return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()` -- the return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}