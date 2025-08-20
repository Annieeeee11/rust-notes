/* used for env */

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("XYZ");

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => println!("error"),
    }
}