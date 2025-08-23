/*
-----global variable----- 
const X: u32 = 1; 

=> not good to use this
*/  

/* Boiler plate code */
  // fn main() {
  //   println!("Hello World!")
  // }

/* -----random built in functions----- */
pub fn testfun() { 
    let secret_num = "15";
    let secret_num_2: String = Default::default();
    println!("{},{},{},{:p}", secret_num_2.capacity(), secret_num.len(), secret_num.is_empty(), secret_num.as_ptr()); 
}

// import from borsh
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct User{
  id: u64,
  data: String,
  v: Vec<u32>,
}

fn main() {

  let s = User { id: 42, data: "hello kitty".into(), v: vec![1,2,3]};
  let mut buffer = Vec::new();

  s.serialize(&mut buffer).unwrap();

  let d = User::try_from_slice(&mut buffer).unwrap();

  assert_eq!(s, d);
  println!("{:?}",d);
}
