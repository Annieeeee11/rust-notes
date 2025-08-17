fn main() {

    let _a = 10;
    _a = 11; //will throw error cause a cant change value

    let mut b = 10;
    b = 11; //this will work perfectly



//Scope => variables exist only inside the block ({}) where you define them.
    let x = 5; // x exists here
    {
        let y = 10; // y exists only inside this block
        println!("Inside block: x = {}, y = {}", x, y);
    }
    // y is NOT available here. It was dropped when the block ended.
    println!("Outside block: x = {}", x);
    println!("y = {}", y);  //This will cause a compile error!



//Shadowing => happens when you declare a new variable with the same name as an existing one — it temporarily hides (shadows) the previous one.
    let m = 5;
    println!("m = {}", m); // 5
    let m = m + 1;  // Shadows the old m
    println!("m = {}", m); // 6
    {
        let m = m * 2; // Shadows again, but only in this inner block
        println!("Inner block x = {}", m); // 12
    }
    println!("Outer x = {}", m); // Back to 6



//EXAMPLE
    let a = 3;          // 👈 Outer 'a' is 3
    {
        let a = a + 2;  // 👈 New inner 'a' shadows the outer one (3 + 2 = 5)
        println!("Inner a = {}", a);  // → prints 5
    }
    println!("Outer a = {}", a);      // → prints 3 (because the shadowed one is gone)
}