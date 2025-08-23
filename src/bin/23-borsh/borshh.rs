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

// output ==> User { id: 42, data: "hello kitty", v: [1, 2, 3] }