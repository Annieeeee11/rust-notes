/* -----ERROR HANDLING----- */

/*
- Error handling is very important in languages to handle the error before if it appears
- javascript do it by try and catch block 

- in rust it is done by result enum 
*/

fn handleing() {
    let contests = fs::read_to_string("a.txt"); 
    // function returns an enum in both success and fail 
    
    match contests { // we can match them throw ok and err 
        Ok(contests) =>  println!("success {}", contests),
        Err(e) => println!("failed"),
    }
}

fn main() {
    let contests = fs::read_to_string("a.txt");
    match contests {
        Ok(contests) =>  println!("success {}", contests),
        Err(e) => println!("failed"),
    }
}