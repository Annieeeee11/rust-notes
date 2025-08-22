//Procedural macro 
/* manully */
use std:: {fmt::format, path::Display, path::Debug};

struct User {
    a: u32,
    b: u32,
}
impl Display for User {} // now i can print the user struct using println becuase i have implement the display and debug trait on the struct
impl Debug for User {} // {:?}
/* using macro */

#[derive(Debug)] // single line will help me do what i did above long code manually 
struct Man {
    a: u32,
    b: u32,
}
// now i can print the each field of this Man using -- println!("{:?}"); 




/* marcos are defined like this  */
macro_rules!  say_hii {
    () => {
        println!("hii");
    };
}

fn main() {
// Declerative macro
    say_hii!(); // expands to the macro i have defined above
    println!("hii"); // this is a macro that has lower code 
    let v = vec![1,2,3]; //another macro for vector 

//Procedural macro 

}