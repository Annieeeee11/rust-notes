/*
-----MATCH-----
- Pattern matching in Rust is the process of comparing a value against one or more patterns
and running different code depending on which pattern matches.

- Clear branching: It makes your intent obvious.
- Exhaustive checking: The compiler forces you to cover all cases (prevents forgotten branches).
- Destructuring: You can pull out values from complex types like structs, tuples, and enums directly in the match arms.

- Some and None are enum variants of Rust’s built-in Option<T> type:

enum Option<T> {
    Some(T),  // Has a value of type T
    None,     // No value
}
- Some(value) → represents “I have a value”
- None → represents “I have no value”
- They are used instead of null to force you to handle the missing case safely

- The default case is written as _ => ...
- It matches anything that hasn’t matched earlier cases.
*/


enum Shape {
    Circle(u32),
    Rect(u32),
    Sqaure(u32),
}

fn testmain() {
    let s = Shape::Circle(3);

    match s {
        Shape::Circle => println!{"circle"},
        _ => {}
    }

    // sigle match case we can use if let (better syntax)
    if let Shape::Circle() = s {
        println!("circle")
    }
}