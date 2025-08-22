/* copy and clone trait  */

/* clone let you call the clone()  function */


/* these traits tell the rust that what to copy and where to pass ownership rules 
what is on stack and what is on heap 
*/

fn main() {
    let user1 = String::from("Hello");

    //give ownership
    print_it(user1);

    // copy -- easy way but it is not good and expensive 
    print_it(user1.clone());

    // pass 
    print_it(&user1);

    println!("{}", user1);
}

fn print_it(a: String) -> String { // add & here too 
    println!("{}",a);
    // i can return the string to give back the ownership if taken
    return a;
}



/* Struct */
/* rust is dumb it doesnt know that this stuct is on stack not on heap so we have to tell him that to avoid the ownership rules */

#[derive(Debug, Copy, Clone)] // r=to derive the clone i need to derive teh copy
struct User {
    is_name: String,
    age: bool,
    // if i add a sting here then i cant derive the copy trait on all the struct of it would have a sting
}

fn main() {
    let u1 = User { is_name: "hii", age: 32};
    let u2 = u1;
    println!("{:?}, {:?}",u1, u2);
}