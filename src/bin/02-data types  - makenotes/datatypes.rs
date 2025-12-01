/* ----- DATA TYPES ----- */

fn main() {
    // Call different functions to demonstrate data types
    integers_example();
    floating_point_example();
    boolean_example();
    character_example();
    tuple_example();
    array_example();
    type_annotation_example();
    type_conversion_example();
    number_formatting_example();
}

// INTEGERS 
fn integers_example() {
    // Signed integers (can be negative)
    let x: i32 = -42;      // signed 32-bit integer
    let y: u32 = 42;       // unsigned 32-bit integer
    let z = 100;           // defaults to i32
    
    println!("Signed: {}", x);
    println!("Unsigned: {}", y);
    println!("Default: {}", z);
}

// FLOATING-POINT NUMBERS
fn floating_point_example() {
    let x = 2.0;           // f64 (default)
    let y: f32 = 3.0;      // f32
    let a = 3.234345500;
    
    println!("Default float (f64): {}", x);
    println!("f32 float: {}", y);
    println!("Formatted to 2 decimals: {a:.2}");  // prints with 2 decimal places: 3.23
}

// BOOLEAN
fn boolean_example() {
    let t = true;
    let f: bool = false;
    
    println!("true: {}", t);
    println!("false: {}", f);
}

// CHARACTER
fn character_example() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("Simple: {}", c);
    println!("Unicode: {}", z);
    println!("Emoji: {}", heart_eyed_cat);
}

// TUPLES
fn tuple_example() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring
    let (x, y, z) = tup;
    println!("Destructured tuple y value: {}", y);
    
    // Access by index
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    
    println!("Tuple access by index: {}, {}, {}", a, b, c);
}

// ARRAYS
fn array_example() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March"];
    
    // Array with type annotation
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Initialize array with same value
    let c = [3; 5];  // [3, 3, 3, 3, 3]
    
    // Access elements
    let first = a[0];
    let second = a[1];
    
    println!("First: {}", first);
    println!("Second: {}", second);
    println!("Array initialized with same value: {:?}", c);
}

// TYPE ANNOTATIONS
fn type_annotation_example() {
    // Without type annotation, compiler wouldn't know what type to parse to
    let guess: u32 = "42".parse().expect("Not a number!");
    
    println!("Parsed string to u32: {}", guess);
}

// TYPE CONVERSION
fn type_conversion_example() {
    let x = 5;
    let y = 2.5;
    
    // This won't work (different types):
    // let z = x + y;
    
    // You need to convert using 'as' keyword
    let z = x as f64 + y;
    
    println!("Type conversion result: {}", z);
}

// NUMBER FORMATTING
fn number_formatting_example() {
    let pi = 3.14159265359;
    
    println!("Full precision: {}", pi);           // 3.14159265359
    println!("2 decimal places: {:.2}", pi);      // 3.14
    println!("4 decimal places: {:.4}", pi);      // 3.1416
}

