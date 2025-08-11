use std::io;


//FUNCTION

/* this is a guess number game and the concepts that i learned from this is are following:
1. use of rand crate to generate random numbers
2. use of io crate to read user input
3. use of loop
4. use of if else to check the user input
5. Ownership & borrowing (mutable + immutable)
6. Copy vs move
7. Functions taking & returning values
8. Loops & conditionals
9. User input
*/

pub fn get_guess() -> u32 {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let users_input: u32 = user_input.trim().parse().expect("Please enter a number");
    users_input
  }
  
pub fn check_guess(secret_num: u32) {
    for attempt in 1..=10 {
        let user_guess = get_guess();

        if user_guess == secret_num {
            println!("You got it!");
            return;
        } else if user_guess > secret_num {
            println!("Too high!");
        } else {
            println!("Too low!");
        }

        if attempt < 10 {
            println!("Please guess again.");
        }
    }
    println!("You lost! The number was {}.", secret_num);
  }









// structs

/*
The module’s items (functions, structs, etc.) are private by default — you need to mark them with pub if you want to access them from outside.
*/ 
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


// random built in functions
pub fn testfun() {
    let secret_num = "15";
    let secret_num_2: String = Default::default();
    println!("{},{},{},{:p}", secret_num_2.capacity(), secret_num.len(), secret_num.is_empty(), secret_num.as_ptr()); 
}