# Talking about Async Rust

## What is Async Programming?

- Async programming allows you to run multiple tasks concurrently without using multiple threads
- It's a way to write non blocking code that can handle many operations at once
- Perfect for I/O bound tasks like network requests, file operations, or database queries
- More efficient than threads for handling many concurrent connections

---

## Sync vs Async vs Parallelism

### Synchronous (Blocking)
- Code runs one line at a time
- Each operation must complete before the next one starts
- Simple but inefficient for I/O operations

### Asynchronous (Non-Blocking)
- Multiple operations can be in progress at once
- While waiting for one operation, other work can be done
- Single-threaded or multi-threaded
- Great for I/O-bound tasks

### Parallel (Multi-Threading)
- Multiple operations run simultaneously on different CPU cores
- True simultaneous execution
- Great for CPU bound tasks

**Understanding:**
- Async is about **waiting efficiently** (I/O bound)
- Parallel is about **computing simultaneously** (CPU bound)
- Async can run on a single thread or multiple threads

---

## Concepts

### 1) Future

- A `Future` is a value that might not be ready yet
- Represents an asynchronous computation
- Must be polled to make progress
- Won't do anything until awaited

```rust
async fn hello() -> String {
    String::from("hello")
}
```

### 2) async/await

- `async` keyword creates a function that returns a Future
- `await` keyword pauses execution until the Future is ready
- Can only use `await` inside `async` functions

```rust
async fn fetch_data() {
    let data = get_data().await;  // Wait for result
    process(data);
}
```

### 3) Executor/Runtime

- Futures are lazy, they don't run until called
- An executor/runtime is needed to run async code
- Popular runtimes: `tokio`, `async-std`, `smol`
- The runtime schedules and runs Futures

---

## The async Keyword

### Async Functions

```rust
async fn do_something() -> u32 {
    42
}
```

**Understanding:**
- Returns a `Future` that resolves to the return type
- Actual return type is `impl Future<Output = u32>`
- Function body doesn't run until the Future is awaited

### Async Blocks

```rust
let future = async {
    println!("Hello from async block");
    42
};
```

**Understanding:**
- Creates an anonymous async function
- Returns a Future
- Can capture variables from surrounding scope

---

## The await Keyword

### Basic Usage

```rust
async fn example() {
    let result = some_async_function().await;
    println!("Result: {}", result);
}
```

**Understanding:**
- Pauses execution until the Future completes
- Returns the value from the Future
- Only works inside async functions or blocks
- Non-blocking other tasks can run while waiting

### Chaining awaits

```rust
async fn fetch_and_process() {
    let data = fetch_data().await;
    let processed = process_data(data).await;
    let saved = save_data(processed).await;
}
```

---

## Futures in Rust

### What is a Future?

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

**Understanding:**
- A Future is polled to check if it's ready
- Returns `Poll::Ready(value)` when complete
- Returns `Poll::Pending` when still waiting
- Executor keeps polling until Ready

### Future Characteristics

- **Lazy**: Don't run until polled/awaited
- **Zero-cost**: No heap allocation required
- **Composable**: Can combine multiple Futures
- **Cancellable**: Dropping a Future cancels it

---

## Async Runtimes

### Why We Need a Runtime

- Futures don't run by themselves
- Need an executor to poll Futures
- Need a reactor to handle I/O events
- Runtime provides both

### Popular Runtimes

#### Tokio
```rust
#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");
}
```

- Most popular async runtime
- Great for network applications

#### async-std
```rust
#[async_std::main]
async fn main() {
    println!("Hello from async-std!");
}
```

- Standard library like API
- Simple and easy to use

#### smol
- Lightweight and minimal
- Good for embedded systems

---

## Running Async Code

### Method 1: Runtime Attribute Macro

```rust
#[tokio::main]
async fn main() {
    hello().await;
}

async fn hello() {
    println!("Hello, async world!");
}
```

### Method 2: Manual Runtime

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        hello().await;
    });
}
```

### Method 3: block_on

```rust
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async_function());
}
```

---

## Concurrent Execution

### Sequential Execution (Slow)

```rust
async fn sequential() {
    let a = task_a().await;  // Wait for A
    let b = task_b().await;  // Then wait for B
    let c = task_c().await;  // Then wait for C
}
```

### Concurrent Execution (Fast)

```rust
use tokio::join;

async fn concurrent() {
    let (a, b, c) = join!(
        task_a(),
        task_b(),
        task_c()
    );  // All run concurrently!
}
```

**Understanding:**
- `join!` runs multiple Futures concurrently
- Waits for all to complete
- Returns results as a tuple

---

## Common Async Patterns

### Pattern 1: join! (Wait for All)

```rust
use tokio::join;

let (result1, result2) = join!(
    async_task1(),
    async_task2()
);
```

- Runs tasks concurrently
- Waits for all to complete
- Returns all results

### Pattern 2: select! (Wait for First)

```rust
use tokio::select;

select! {
    result1 = async_task1() => {
        println!("Task 1 finished first: {:?}", result1);
    }
    result2 = async_task2() => {
        println!("Task 2 finished first: {:?}", result2);
    }
}
```

- Runs tasks concurrently
- Returns when first one completes
- Cancels other tasks

### Pattern 3: spawn (Background Task)

```rust
use tokio::spawn;

let handle = spawn(async {
    // This runs in the background
    expensive_operation().await
});

// Do other work...

let result = handle.await.unwrap();
```

- Spawns a task to run in the background
- Returns a handle to await later
- Task runs independently

---

## Async Traits and Methods

### Async Methods in Impl Blocks

```rust
struct Database;

impl Database {
    async fn connect(&self) -> Result<Connection, Error> {
        // Async connection logic
    }
    
    async fn query(&self, sql: &str) -> Result<Vec<Row>, Error> {
        // Async query logic
    }
}
```

### Async Closures (Unstable)

```rust
// Currently requires nightly Rust
let closure = async || {
    println!("Async closure");
};
```

---

## Error Handling in Async

### Using Result with await

```rust
async fn fetch_data() -> Result<String, Error> {
    let response = http_get("url").await?;
    let data = response.text().await?;
    Ok(data)
}
```

**Understanding:**
- `?` operator works with async functions
- Propagates errors just like sync code
- Returns early on error

### Handling Multiple Async Errors

```rust
async fn process() -> Result<(), Error> {
    let data1 = fetch1().await?;
    let data2 = fetch2().await?;
    save(data1, data2).await?;
    Ok(())
}
```

---

## Async and Lifetimes

### Borrowing in Async Functions

```rust
async fn process_data(data: &str) -> usize {
    // Can borrow data across await points
    let len = data.len();
    tokio::time::sleep(Duration::from_secs(1)).await;
    len
}
```

**Understanding:**
- References must live long enough
- Borrowed data must outlive the Future
- Compiler ensures safety

---

## Common Async Operations

### 1) Sleeping/Delays

```rust
use tokio::time::{sleep, Duration};

async fn delayed_hello() {
    sleep(Duration::from_secs(1)).await;
    println!("Hello after 1 second!");
}
```

### 2) Timeouts

```rust
use tokio::time::{timeout, Duration};

async fn with_timeout() {
    match timeout(Duration::from_secs(5), long_operation()).await {
        Ok(result) => println!("Completed: {:?}", result),
        Err(_) => println!("Timeout!"),
    }
}
```

### 3) Intervals

```rust
use tokio::time::{interval, Duration};

async fn periodic_task() {
    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("Tick!");
    }
}
```

---

## Channels for Async Communication

### mpsc (Multiple Producer, Single Consumer)

```rust
use tokio::sync::mpsc;

async fn channel_example() {
    let (tx, mut rx) = mpsc::channel(32);
    
    // Send messages
    tx.send("hello").await.unwrap();
    
    // Receive messages
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}
```

### oneshot (One-time Communication)

```rust
use tokio::sync::oneshot;

async fn oneshot_example() {
    let (tx, rx) = oneshot::channel();
    
    spawn(async move {
        tx.send("result").unwrap();
    });
    
    let result = rx.await.unwrap();
}
```

---

## Async vs Threads

### When to Use Async

- **I/O bound tasks**: Network, file operations, databases
- **Many concurrent connections**: Web servers, chat apps
- **Efficient resource usage**: Less memory per task
- **Cooperative multitasking**: Tasks yield control

### When to Use Threads

- **CPU-bound tasks**: Heavy computations, data processing
- **Blocking operations**: Legacy APIs, system calls
- **True parallelism**: Utilize multiple CPU cores
- **Simpler mental model**: Easier to reason about