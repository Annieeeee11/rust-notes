/* ----- ADVANCED FEATURES ----- */
/*
1. Unsafe Rust:
   - Dereference raw pointers
   - Call unsafe functions
   - Access mutable static variables
   - Implement unsafe traits
   - Access union fields

2. Advanced Traits:
   - Associated types
   - Default generic type parameters
   - Fully qualified syntax
   - Supertraits
   - Newtype pattern

3. Advanced Types:
   - Type aliases
   - Never type (!)
   - Dynamically sized types (DST)

4. Advanced Functions:
   - Function pointers
   - Returning closures

5. Macros:
   - Declarative macros (macro_rules!)
   - Procedural macros (derive, attribute, function-like)
*/

fn main() {
    unsafe_rust_example();
    raw_pointers_example();
    associated_types_example();
    default_generic_types_example();
    fully_qualified_syntax_example();
    supertraits_example();
    newtype_pattern_example();
    type_aliases_example();
    never_type_example();
    function_pointers_example();
    returning_closures_example();
    declarative_macros_example();
    operator_overloading_example();
}

// UNSAFE RUST
fn unsafe_rust_example() {
    let mut num = 5;
    
    // Creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // Dereferencing raw pointers requires unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// RAW POINTERS
fn raw_pointers_example() {
    let mut num = 5;
    
    // Immutable and mutable raw pointers to the same location
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        // Can have both immutable and mutable raw pointers
        // This would not be allowed with regular references
        println!("r1: {}", *r1);
        
        *r2 = 10;
        println!("r2 after modification: {}", *r2);
    }
    
    // Creating a raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let r = address as *const i32;
    // Dereferencing this would be undefined behavior!
}

// ASSOCIATED TYPES
fn associated_types_example() {
    trait Iterator {
        type Item;  // Associated type
        
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    struct Counter {
        count: u32,
    }
    
    impl Iterator for Counter {
        type Item = u32;  // Concrete type for associated type
        
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    let mut counter = Counter { count: 0 };
    println!("Counter: {:?}", counter.next());
    println!("Counter: {:?}", counter.next());
}

// DEFAULT GENERIC TYPE PARAMETERS
fn default_generic_types_example() {
    use std::ops::Add;
    
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    // Add trait has default generic type parameter
    impl Add for Point {
        type Output = Point;
        
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    
    println!("Point addition: {:?}", p3);
}

// FULLY QUALIFIED SYNTAX
fn fully_qualified_syntax_example() {
    trait Pilot {
        fn fly(&self);
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human;
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    
    let person = Human;
    
    // Calling methods with same name
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    
    // Fully qualified syntax
    <Human as Pilot>::fly(&person);
}

// SUPERTRAITS
fn supertraits_example() {
    use std::fmt;
    
    // OutlinePrint requires Display (supertrait)
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    impl OutlinePrint for Point {}
    
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

// NEWTYPE PATTERN
fn newtype_pattern_example() {
    use std::fmt;
    
    // Wrapper around Vec<String>
    struct Wrapper(Vec<String>);
    
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    
    // Type safety example
    struct Meters(u32);
    struct Kilometers(u32);
    
    let m = Meters(1000);
    let k = Kilometers(1);
    
    // These are different types even though both wrap u32
    // let sum = m.0 + k.0;  // Would need explicit conversion
}

// TYPE ALIASES
fn type_aliases_example() {
    // Type alias for long type
    type Kilometers = i32;
    
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y);  // Can mix because it's just an alias
    
    // Common use with Result
    type Result<T> = std::result::Result<T, std::io::Error>;
    
    // Now can write Result<T> instead of std::result::Result<T, std::io::Error>
    fn read_file() -> Result<String> {
        Ok(String::from("file contents"))
    }
    
    println!("File read result: {:?}", read_file());
}

// NEVER TYPE
fn never_type_example() {
    // Function that never returns
    fn bar() -> ! {
        loop {
            // This loop never ends
            break;  // Added break to make example runnable
        }
    }
    
    // Never type in match
    let guess = "3";
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // continue has type !
            // Can be coerced to u32
            0  // Using 0 instead of continue for example
        },
    };
    
    println!("Guess: {}", guess);
}

// FUNCTION POINTERS
fn function_pointers_example() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    // Function that takes a function pointer
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    
    // Using with map
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    
    // Or using function pointer directly
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
    
    println!("Strings: {:?}", list_of_strings);
}

// RETURNING CLOSURES
fn returning_closures_example() {
    // Must use Box because closures don't have known size
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    
    let closure = returns_closure();
    println!("Closure result: {}", closure(5));
}

// DECLARATIVE MACROS
fn declarative_macros_example() {
    // Simple macro definition
    macro_rules! say_hello {
        () => {
            println!("Hello from macro!");
        };
    }
    
    say_hello!();
    
    // Macro with arguments
    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("Function {:?} is called", stringify!($func_name));
            }
        };
    }
    
    create_function!(foo);
    foo();
    
    // Variadic macro
    macro_rules! print_all {
        ($($x:expr),*) => {
            $(
                println!("{}", $x);
            )*
        };
    }
    
    print_all!(1, 2, 3, "hello");
}

// OPERATOR OVERLOADING
fn operator_overloading_example() {
    use std::ops::Add;
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Point;
        
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    
    println!("{:?} + {:?} = {:?}", p1, p2, p3);
}

// UNSAFE FUNCTION EXAMPLE
unsafe fn dangerous() {
    println!("This is an unsafe function");
}

fn call_unsafe_function() {
    unsafe {
        dangerous();
    }
}

// STATIC VARIABLES
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn static_variables_example() {
    add_to_count(3);
    
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}