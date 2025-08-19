/* 
-----FREEZE-----
A mutable variable is temporarily made immutable, which prevents it from being changed — even if it was declared as mut.
This usually occurs when the same variable is borrowed immutably or shadowed immutably.
*/


fn main() {
    let mut x = 5;

    let y = &x;      // Immutable borrow — freezes `x`
    x += 1;         // ERROR: cannot assign to `x` because it's borrowed

    println!("y = {}", y); //Ok

    // Once `y` goes out of scope (after this line), `x` is unfrozen
    x += 1;       //Would work here



// Freezing through Shadowing
// Freezing can also happen if you shadow a mutable variable with an immutable one:
    let mut m = 10;
    let m = m;     // shadowing with an immutable binding
    // m += 1;     // ERROR: `m` is now immutable (frozen shadow)

    println!("m = {}", m);
}