/* 
-----IMPORTS----- 
mod part2;         // "define/load a module and make it public to other modules/crates".
use part2::{get_guess, check_guess, User};
mod enums;
use enums::Shape;

*/

/*
-----global variable----- 
const X: u32 = 1; 

=> not good to use this
*/  

/* Boiler plate code */
  fn main() {
    println!("Hello World!")
  }

/* -----random built in functions----- */
/* The module’s items (functions, structs, etc.) are private by default — you need to mark them with pub if you want to access them from outside. */ 
pub fn testfun() { 
    let secret_num = "15";
    let secret_num_2: String = Default::default();
    println!("{},{},{},{:p}", secret_num_2.capacity(), secret_num.len(), secret_num.is_empty(), secret_num.as_ptr()); 
}