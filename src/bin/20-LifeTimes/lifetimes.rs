/*  the following function with lead to memory issues  according to compiler cause it is dumb */
/*  so lifetimes are made to solve that more over you have to specify the annotation where you have 
    like two borrow else rust is smart enough to assume the lifetime.
*/

fn main() {
    let str1 = String::from("hello1");
    let str2 = String::from("hey2");
    let ans = long_string(&str1, &str2);
    println!("{}", ans)
}

// this function wil throw error the function return type will throw error and code wont compile
fn long_string<'a , 'b>(s1: &'a String, s2: &'b String) -> &'a String { // we pass a string reference
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}