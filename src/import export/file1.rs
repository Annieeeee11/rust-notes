/* import a file which is inside a folder */
mod folder1;
use folder1::example::testfunction12;
/* simple import */
mod file2;
use file2::testfunction1;

/* 
mod part2;   // "define/load a module and make it public to other modules/crates".
use part2::{get_guess, check_guess, User};
*/

fn main() {
    let a = 5;
    let b = 10;

    let res1 = testfunction1(a,b); 
    let res2 = testfunction12(a,b);

    println!("{} , {}", res1,res2);
}