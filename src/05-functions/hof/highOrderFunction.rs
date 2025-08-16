fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
/*
Find the sum of all odd squares less than 1000.
- That means:
- For numbers starting from 0:
- Square each number → n * n
- Stop if the square is ≥ 1000
- If the square is odd, keep it
- Finally, sum all those odd squares 
*/


    let upper = 1000;
/* Imperative Approach (Loop) 
Step-by-step, manual control (for, if) */
    let mut acc = 0; // Declare accumulator variable
    
    for n in 0.. {                // Iterate: 0, 1, 2, ... to infinity    
        let n_squared = n * n;    // Square the number
        if n_squared >= upper { 
            break;                // Break loop if exceeded the upper limit
        } else if is_odd(n_squared) {
            acc += n_squared;     // Accumulate value, if it's odd
        }
    }
    println!("imperative style: {}", acc);


/* Functional approach 
Uses iterator methods like map, filter */
    let sum_of_squared_odd_numbers: u32 =
        (0..) //This is a higher-order function.
             .map(|n| n * n)                             // A closure thats squares the natural numbers 
             .take_while(|&n_squared| n_squared < upper) // stop when n*n is >= 1000
             .filter(|&n_squared| is_odd(n_squared))     // keep only odd numbers
             .sum();                                     // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
/* 
1) (0..) => Starts an infinite iterator 1,2,3,.....

2) .map() => It is a method used on iterators like Vec, Range, etc.
- Take each item, apply a function to it, and return a new list.
- .map() always needs: A closure that tells how to transform each item

eg => 
{
let nums = vec![1, 2, 3, 4];
let squares: Vec<i32> = nums
    .iter()
    .map(|n| n * n)
    .collect();
println!("{:?}", squares);  // [1, 4, 9, 16]
/* 
.iter() → goes over each item: 1, 2, 3, 4
.map(|n| n * n) → replaces each n with n * n → 1, 4, 9, 16
.collect() → turns it back into a Vec
*/
}

3) .take_while() => Keeps items as long as the condition is true

4) .filter() => Keeps only the items that satisfy a condition.

5) .sum() => sum up everythimt
*/
}


