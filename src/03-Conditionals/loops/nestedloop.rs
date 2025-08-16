// It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.

#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

// Returning from loops
//One of the uses of a loop is to retry an operation until it succeeds. 
//If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break,
//and it will be returned by the loop expression.

fn returnloop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}