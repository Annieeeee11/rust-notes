fn main() {

    let a = String::from(""); //String type 
    let b = &a; // or &a[0..3];  -- has a view into the original string/is a reference 
    let c = "asdf"; 
    //literal also known as &str but it points to an address in the binary -- this string is hardcorded in the final build binary and this points to that 


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
}