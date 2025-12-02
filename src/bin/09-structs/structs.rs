/* -----STRUCT----- */

pub struct User {
    pub name: String,
    pub id: u32,
    pub email: String,
    pub age: u32,
}

impl User {
    pub fn _get_chang(&self) {
        // method body here
    }
}

fn main() {
   let user1 = User {
        name: String::from("anaya"),
        id: 1,
        email: String::from("anaya@example.com"),
        age: 20,
    };
    println!("{},{},{},{}",user1.name,user1.id,user1.email,user1.age ); 
}