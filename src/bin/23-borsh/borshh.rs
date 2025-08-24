// import from borsh
use borsh::{BorshSerialize, BorshDeserialize};

// using macro from the derive feature so that i dont have to write long functions everywhere
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct User{
  id: u64,
  data: String,
  v: Vec<u32>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct User1 {
  username:String,
  password: String,
}

fn main() {

  let s = User { id: 42, data: "hello kitty".into(), v: vec![1,2,3]};
  let mut buffer = Vec::new();

  s.serialize(&mut buffer).unwrap();

  let d = User::try_from_slice(&mut buffer).unwrap();

  assert_eq!(s, d);
  println!("{:?}",d);

  // output ==> User { id: 42, data: "hello kitty", v: [1, 2, 3] }


  /* easy serialize */
  let u = User1 {
    username: String::from("hiii1"),
    password: String::from("hiii2"),
  };
  let mut v = Vec::new();

  let ans = u.serialize(&mut v);

  match ans {
    Ok(_) => println!("{:?}", v),
    Err(_) => print!("error"),
  }
  // output => [5, 0, 0, 0, 104, 105, 105, 105, 49, 5, 0, 0, 0, 104, 105, 105, 105, 50]

  /* easy deserialize */

  let user = User1::try_from_slice(&v).unwrap();
  println!("{:?}",user);
  // output => User { username: "hiii1", password: "hiii2" }
}

