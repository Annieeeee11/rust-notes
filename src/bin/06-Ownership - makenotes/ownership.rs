/* ----- OWNERSHIP ----- */

// OWNERSHIP RULES:
//    - Each value has one owner
//    - Only one owner at a time
//    - Value dropped when owner goes out of scope

fn main() {
    scope_ex();
    string_literal_vs_string_type();
    move_ex();
    clone_ex();
    copy_ex();
    ownership_and_functions();
    return_values_and_ownership();
    copy_trait_types();
}

// VARIABLE SCOPE
fn scope_ex() {
    {
        let s = "hello";   // s is valid from this point forward
        println!("Inside scope: {}", s);
    }   // this scope is now over, and s is no longer valid so you cant use it ok
    
    // println!("{}", s);  // ERROR! s is not in scope here told you 
    
    println!("Scope ended, s is dropped here so you cant use it ok");
}

// STRING LITERAL VS STRING TYPE
fn string_literal_vs_string_type() {

    // String literal - immutable, fixed size, stored in binary
    let s1 = "hello";
    println!("String literal: {}", s1);
    
    // String type --> mutable, growable, stored on heap
    let mut s2 = String::from("hello");
    s2.push_str(", world!");  // Can modify String type
    println!("String type: {}", s2);
    
    // String literal cannot be modified
    // s1.push_str("!"); // ERROR! String literals are immutable
    println!();
}

// MOVE EXAMPLE (Heap Data)
fn move_ex() {
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    
    let s2 = s1;  // s1 is MOVED to s2, s1 is no longer valid
    println!("s2: {}", s2);
    
    // println!("{}", s1);  // ERROR! s1 is no longer valid
    // This prevents double free error only s2 will call drop
    
    println!("s1 was moved to s2, s1 is no longer valid ok");
}

// CLONE EXAMPLE (Deep Copy)
fn clone_ex() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy of heap data
    
    println!("s1: {}, s2: {}", s1, s2);
    println!("Both s1 and s2 are valid because we used clone()");
}

// COPY EXAMPLE (Stack Data)
fn copy_ex() {    
    // Integers are stored on stack and implement Copy trait
    let x = 5;
    let y = x;  // x is COPIED to y, both remain valid
    
    println!("x: {}, y: {}", x, y);
    println!("Both x and y are valid because integers implement Copy trait");
    
    // Other examples of Copy types
    let a = true;
    let b = a;  // bool implements Copy
    println!("bool: a = {}, b = {}", a, b);
    
    let c = 3.14;
    let d = c;  // f64 implements Copy
    println!("f64: c = {}, d = {}", c, d);
    
    let e = 'z';
    let f = e;  // char implements Copy
    println!("char: e = {}, f = {} ", e, f);
}

// OWNERSHIP AND FUNCTIONS
fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope
    println!("Before function call: s = {}", s);

    takes_ownership(s);  // s's value moves into the function
                         // s is no longer valid here ok
    
    // println!("{}", s);  // ERROR! s is no longer valid told you 
    
    let x = 5;  // x comes into scope
    println!("Before function call: x = {}", x);
    
    makes_copy(x);  // x would move into the function,
                    // but i32 is Copy, so it's okay to still use x afterward ok
    
    println!("After function call: x = {}", x);  // x is still valid told you
}

fn takes_ownership(some_string: String) {  // some_string comes into scope
    println!("Inside takes_ownership: {}", some_string);
}  // some_string goes out of scope and `drop` is called
   // The backing memory is freed

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("Inside makes_copy: {}", some_integer);
}  // Here, some_integer goes out of scope. Nothing special happens

// RETURN VALUES AND OWNERSHIP
fn return_values_and_ownership() {
    let s1 = gives_ownership();  // gives_ownership moves its return value into s1
    println!("s1 from gives_ownership: {}", s1);
    
    let s2 = String::from("hello");  // s2 comes into scope
    println!("s2 before function: {}", s2);
    
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back
                                         // which also moves its return value into s3
    
    // println!("{}", s2);  // ERROR! s2 is no longer valid
    println!("s3 from takes_and_gives_back: {}", s3);
}  // Here, s3 goes out of scope and is dropped
   // s2 was moved, so nothing happens
   // s1 goes out of scope and is dropped

fn gives_ownership() -> String {  // gives_ownership will move its return value
                                  // into the function that calls it
    let some_string = String::from("yours");  // some_string comes into scope
    some_string  // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

// COPY TRAIT TYPES
fn copy_trait_types() {
    // All integer types implement Copy
    let i1: i32 = 10;
    let i2 = i1;
    println!("i32: i1 = {}, i2 = {}", i1, i2);
    
    let u1: u32 = 20;
    let u2 = u1;
    println!("u32: u1 = {}, u2 = {}", u1, u2);
    
    // Boolean implements Copy
    let b1 = true;
    let b2 = b1;
    println!("bool: b1 = {}, b2 = {}", b1, b2);
    
    // Floating point types implement Copy
    let f1: f32 = 3.14;
    let f2 = f1;
    println!("f32: f1 = {}, f2 = {}", f1, f2);
    
    let f3: f64 = 2.71;
    let f4 = f3;
    println!("f64: f3 = {}, f4 = {}", f3, f4);
    
    // Character type implements Copy
    let c1 = 'a';
    let c2 = c1;
    println!("char: c1 = {}, c2 = {}", c1, c2);
    
    // Tuples implement Copy if all their elements implement Copy
    let t1 = (1, 2, 3);
    let t2 = t1;
    println!("tuple: t1 = {:?}, t2 = {:?}", t1, t2);
}
