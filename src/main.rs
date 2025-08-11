use rand::Rng;
mod part2;
use part2::{get_guess, check_guess, User};
  
  fn main() {

    //guess number game 
    println!("You have 10 guesses to guess the number.");
    println!("how can rounds do you want to play?");
    let rounds = get_guess();
    for i in 0..rounds {
        let secret_num = rand::thread_rng().gen_range(1..101);
        println!("Round {}: Guess the number!", i + 1);
        check_guess(secret_num);
    }

    //Structs
    let user1 = User {
        name: String::from("John"),
        id: 1,
        email: String::from("john@example.com"),
        age: 20,
    };
    println!("{},{},{},{}",user1.name,user1.id,user1.email,user1.age ) 
  }
