/* use to fetch current time */

use chrono::{Utc, Local};

fn main() {
    let utc = Utc::now();
    let local_time = Local::now();
    println!("{} , {}", utc, local_time);
}