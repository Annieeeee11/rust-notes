/* ----- SMART POINTERS ----- */
/*
1. Box<T>:
   - Single owner
   - Heap allocation
   - No overhead except heap storage
   - Use for: recursive types, large data, trait objects

2. Rc<T>:
   - Multiple owners
   - Single-threaded only
   - Reference counting overhead
   - Immutable access only
   - Use for: shared data in single thread

3. Arc<T>:
   - Multiple owners
   - Thread-safe
   - Atomic reference counting overhead
   - Immutable access only
   - Use for: shared data across threads

4. RefCell<T>:
   - Single owner
   - Runtime borrow checking
   - Interior mutability
   - Single-threaded only
   - Use for: mutable data when compiler can't verify safety

5. Rc<RefCell<T>>:
   - Multiple owners with mutability
   - Single-threaded only
   - Use for: shared mutable data in single thread

6. Arc<Mutex<T>>:
   - Multiple owners with mutability
   - Thread-safe
   - Blocking on lock acquisition
   - Use for: shared mutable data across threads

7. Arc<RwLock<T>>:
   - Multiple owners with mutability
   - Thread-safe
   - Multiple readers or one writer
   - Use for: shared data with many readers, few writers

Traits:
- Deref: Allows smart pointers to act like references
- Drop: Automatic cleanup when value goes out of scope
- Clone: For Rc/Arc, increases reference count
*/


use std::rc::{Rc, Weak};
use std::cell::{RefCell, Cell};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    box_example();
    box_recursive_type();
    box_trait_objects();
    rc_example();
    refcell_example();
    rc_refcell_combination();
    weak_reference_example();
    reference_cycles_example();
    cell_example();
    deref_example();
    deref_coercion_example();
    drop_example();
    arc_mutex_example();
    arc_rwlock_example();
}

// BOX<T> --> HEAP ALLOCATION
fn box_example() {
    // Store a value on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Box is automatically deallocated when it goes out of scope
    // The data on the heap is also freed
    
    // Large data on heap to avoid stack overflow
    let large_data = Box::new([0; 1000000]);
    println!("Large data stored on heap, first element: {}", large_data[0]);
}

// BOX<T> --> RECURSIVE TYPES
fn box_recursive_type() {
    // Cons list a data structure from Lisp
    // Without Box, this would be infinite size
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    // Create a list: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list: {:?}", list);
}

// BOX<T> --> TRAIT OBJECTS
fn box_trait_objects() {
    // Define a trait
    trait Draw {
        fn draw(&self);
    }
    
    struct Button {
        width: u32,
        height: u32,
        label: String,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing button: {} ({}x{})", self.label, self.width, self.height);
        }
    }
    
    struct TextField {
        placeholder: String,
    }
    
    impl Draw for TextField {
        fn draw(&self) {
            println!("Drawing text field with placeholder: {}", self.placeholder);
        }
    }
    
    // Store different types that implement Draw
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
        Box::new(TextField {
            placeholder: String::from("Enter text..."),
        }),
    ];
    
    for component in components.iter() {
        component.draw();
    }
}

// RC<T> --> REFERENCE COUNTING
fn rc_example() {
    // Create a reference counted value
    let a = Rc::new(5);
    println!("a = {}, reference count = {}", a, Rc::strong_count(&a));
    
    // Clone creates a new reference, not a deep copy
    let b = Rc::clone(&a);
    println!("After creating b, reference count = {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("After creating c, reference count = {}", Rc::strong_count(&a));
    }  // c goes out of scope here
    
    println!("After c goes out of scope, reference count = {}", Rc::strong_count(&a));
    
    // Multiple ownership example
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a reference count = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("After creating b, a reference count = {}", Rc::strong_count(&a));
    
    let c = Cons(4, Rc::clone(&a));
    println!("After creating c, a reference count = {}", Rc::strong_count(&a));
}

// REFCELL<T> --> INTERIOR MUTABILITY
fn refcell_example() {
    // RefCell allows mutation through immutable reference
    let value = RefCell::new(5);
    
    // Borrow immutably
    {
        let borrowed = value.borrow();
        println!("Immutable borrow: {}", borrowed);
    }  // borrowed goes out of scope
    
    // Borrow mutably
    {
        let mut borrowed_mut = value.borrow_mut();
        *borrowed_mut += 10;
        println!("After mutation: {}", borrowed_mut);
    }  // borrowed_mut goes out of scope
    
    println!("Final value: {}", value.borrow());
    
    // This would panic at runtime (can't have mutable and immutable borrows simultaneously)
    // let borrowed = value.borrow();
    // let mut borrowed_mut = value.borrow_mut();  // PANIC!
}

// RC<REFCELL<T>> --> MULTIPLE OWNERS WITH MUTABILITY
fn rc_refcell_combination() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    // Create nodes with shared ownership and interior mutability
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    println!("leaf reference count = {}", Rc::strong_count(&leaf));
    println!("branch value = {}", branch.value);
    
    // Modify children through immutable reference
    branch.children.borrow_mut().push(Rc::clone(&leaf));
    println!("After adding another child, leaf reference count = {}", Rc::strong_count(&leaf));
}

// WEAK REFERENCES --> PREVENTING REFERENCE CYCLES
fn weak_reference_example() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    // Create a leaf node
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf strong = {}, weak = {}", 
             Rc::strong_count(&leaf), 
             Rc::weak_count(&leaf));
    
    // Create a branch node
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        // Set leaf's parent to branch (using Weak to avoid cycle)
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("branch strong = {}, weak = {}", 
                 Rc::strong_count(&branch), 
                 Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", 
                 Rc::strong_count(&leaf), 
                 Rc::weak_count(&leaf));
        
        // Access parent through weak reference
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    
    // branch is dropped here, but leaf still exists
    println!("After branch dropped, leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// REFERENCE CYCLES --> MEMORY LEAK DEMONSTRATION
fn reference_cycles_example() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }
    
    impl Drop for Node {
        fn drop(&mut self) {
            println!("Dropping node with value {}", self.value);
        }
    }
    
    // Create two nodes
    let a = Rc::new(Node {
        value: 5,
        next: RefCell::new(None),
    });
    
    let b = Rc::new(Node {
        value: 10,
        next: RefCell::new(Some(Rc::clone(&a))),
    });
    
    // Create a cycle: a -> b -> a
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    
    println!("a strong count = {}", Rc::strong_count(&a));
    println!("b strong count = {}", Rc::strong_count(&b));
    
    // To prevent memory leak, we should break the cycle
    // Uncommenting the next line would prevent the leak:
    // *a.next.borrow_mut() = None;
    
    println!("Note: This creates a reference cycle (memory leak)");
    println!("In real code, use Weak<T> to prevent this!");
}

// CELL<T> --> INTERIOR MUTABILITY FOR COPY TYPES
fn cell_example() {
    // Cell provides interior mutability for Copy types
    let x = Cell::new(5);
    let y = &x;
    let z = &x;
    
    // Can mutate through multiple immutable references
    y.set(10);
    z.set(15);
    
    println!("x value: {}", x.get());
    
    // Common use case: interior mutability in structs
    struct Counter {
        count: Cell<u32>,
    }
    
    impl Counter {
        fn new() -> Counter {
            Counter { count: Cell::new(0) }
        }
        
        fn increment(&self) {  // Takes &self, not &mut self
            self.count.set(self.count.get() + 1);
        }
        
        fn get(&self) -> u32 {
            self.count.get()
        }
    }
    
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Counter value: {}", counter.get());
}

// DEREF TRAIT
fn deref_example() {
    // Box implements Deref
    let x = 5;
    let y = Box::new(x);
    
    // Can use * to dereference Box
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x = {}, *y = {}", x, *y);
    
    // Custom smart pointer
    struct MyBox<T>(T);
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    use std::ops::Deref;
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    let m = MyBox::new(String::from("Rust"));
    
    // Manual dereferencing
    println!("Manual dereference: {}", *m);
    
    // Behind the scenes, Rust does: *(m.deref())
}

// DEREF COERCION
fn deref_coercion_example() {
    struct MyBox<T>(T);
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    use std::ops::Deref;
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    
    let m = MyBox::new(String::from("Rust"));
    
    // Deref coercion: &MyBox<String> -> &String -> &str
    hello(&m);  // Automatic deref coercion
    
    // Without deref coercion, we'd need:
    hello(&(*m)[..]);
    
    // Deref coercion rules:
    // 1. From &T to &U when T: Deref<Target=U>
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T: Deref<Target=U>
}

// DROP TRAIT
fn drop_example() {
    // Custom type with Drop implementation
    struct CustomSmartPointer {
        data: String,
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data: {}", self.data);
        }
    }
    
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created");
    }  // c and d go out of scope here, drop is called automatically
    
    println!("CustomSmartPointers dropped");
    
    // Manually drop a value early
    let e = CustomSmartPointer {
        data: String::from("early drop"),
    };
    println!("Before manual drop");
    drop(e);  // Call std::mem::drop to drop early
    println!("After manual drop");
}

// ARC<MUTEX<T>> --> THREAD-SAFE SHARED MUTABLE STATE
fn arc_mutex_example() {
    // Shared mutable state across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
    
    // Demonstrating lock scope
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    {
        let mut v = data.lock().unwrap();
        v.push(4);
    }  // Lock is released here
    
    println!("Data after modification: {:?}", *data.lock().unwrap());
}

// ARC<RWLOCK<T>> --> MULTIPLE READERS OR ONE WRITER
fn arc_rwlock_example() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Multiple readers can access simultaneously
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data.read().unwrap();
            println!("Reader {} sees: {:?}", i, *read_guard);
            // Simulate some work
            thread::sleep(std::time::Duration::from_millis(10));
        });
        handles.push(handle);
    }
    
    // One writer (will wait for all readers to finish)
    let data_clone = Arc::clone(&data);
    let writer = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50));
        let mut write_guard = data_clone.write().unwrap();
        write_guard.push(4);
        println!("Writer added 4");
    });
    handles.push(writer);
    
    // Another reader after writer
    let data_clone2 = Arc::clone(&data);
    let reader = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100));
        let read_guard = data_clone2.read().unwrap();
        println!("Final reader sees: {:?}", *read_guard);
    });
    handles.push(reader);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", *data.read().unwrap());
}