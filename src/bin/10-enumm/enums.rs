/* -----ENUMS AND MATCH----- */

/*
----ENUM-------
- A type that can be one value out of a fixed set of possible variants.
or maybe Represents a choice or state.
- use it When you want to limit a variable to a few possible options and handle them explicitly.

- This is why enums in Rust are more powerful than C-style enums — they can be both a choice and a data container

- enums can store data too in rust and the difference in struct and enum is that you can choose only one of the 
value present in enum likhe i have 2 values north or south then i can only choose north yaa south 
and the data will store according like -- like the tag will be stored (north or south) and if any data related to that

- match is a control flow construct, It’s often used with enums, because it lets you handle each variant of the enum explicitly.
Some and None are enum variants of Option<T> that you often use inside a match.
*/

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

-----ENUMS WITH DATA-----
- In rust enums can store values with a variant.
not actually a value but like a data type in which the variant accepts the value or many a value limit too.
Square(f32),
*/

pub enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32,f32),
}
impl Shape {
    pub fn calculate_area(&self) -> f32 { // used self so that shape ownership doesnt come to this function
        match self {
            Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width , height) => width * height,
        } // to return => add return before match and end with ; or dont add ; or return in a variable 
    }
}

fn main() {
    let a = Shape::Circle(10.2);
    let b = Shape::Square(11.2);
    let c = Shape::Rectangle(5.0,4.0);
    println!("Area: {}", a.calculate_area());
    println!("Area: {}", b.calculate_area());
    println!("Area: {}", c.calculate_area());
}