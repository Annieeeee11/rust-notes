fn main() {

    let a = String::from(""); //String type 
    let b = &a; // or &a[0..3]; has a view into the original string/is a reference 
    let c = "asdf"; 
    //literal also known as &str but it points to an address in the binary, this string is hardcorded in the final build binary and this points to that 


    //creating
    let mut name = String::from("Hiiiii");

    //mutating
    name.push_str("helloooo");

    //deleting
    name.replace_range(5..name.len(), "");

    println!("{}", name);


    //slice can also be applied to collections
    let arr = [1,2,4];
    let arr_slice = &arr[0..1];
    println!("{:?}", arr_slice);


    //assigment get first word 
    fn first_word(s:&String) -> &str {
        let mut space = 0;
        for i in s.chars() {
            if i == ' ' {
                break;
            }
            space += 1;
        }
        return &s[0..space];
    }
    
    let s = String::from("hello jii");
    let ans = first_word(&s);
    print!("{}", ans);

    //Append a single character
    let mut test_s = String::from("lo");
    test_s.push('l');
    //Append a slice
    test_s.push_str("bar");

    //Using + or concatenation:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!"); 
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed

    // using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    //Can’t Index a String
    let test_s2 = String::from("hi");
    let h = s[0]; // error
    /* because a String is actually a Vec<u8> underneath. so characters have different bytes so 
    if Rust let you do s[0], you’d only get part of a character (invalid Unicode).  */
}
