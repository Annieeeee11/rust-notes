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
pub fn testfun() { 
    let secret_num = "15";
    let secret_num_2: String = Default::default();
    println!("{},{},{},{:p}", secret_num_2.capacity(), secret_num.len(), secret_num.is_empty(), secret_num.as_ptr()); 
}