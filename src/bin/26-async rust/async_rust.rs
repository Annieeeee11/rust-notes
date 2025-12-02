/* ----- ASYNC RUST ----- */

// WHY ASYNC?
// - Threads are expensive (memory, context switching)
// - Async lets you handle thousands of I/O tasks with few threads
// - Perfect for: web servers, APIs, file I/O, network calls

// THE CATCH:
// - Rust's async is "lazy", nothing runs until you .await it
// - You NEED a runtime (tokio, async-std) to actually execute async code
// - Can't use async in regular main() without a runtime


use tokio::time::{sleep, Duration, timeout};
use tokio::{join, select, spawn};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    basic_async().await;
    sequential_vs_concurrent().await;
    error_handling().await;
    borrowing_across_await().await;
    async_in_structs().await;
    useful_patterns().await;
}

// BASIC ASYNC --> it's just a function that returns a Future
async fn say_hello() -> String {
    // this looks like it returns String, but actually returns impl Future<Output = String>
    // the future is lazyz, this code doesn't run until someone .awaits it
    String::from("anaya")
}

async fn basic_async() {
    // calling async fn doesn't run it, just creates a Future
    let future = say_hello();  // nothing happened yet
    
    // .await actually runs it
    let result = future.await;
    println!("{}", result);
    
    // usually you just chain them
    let msg = say_hello().await;
    println!("{}", msg);
}

// THE BIG DIFFERENCE: sequential vs concurrent
async fn do_work(name: &str, secs: u64) -> String {
    println!("  {} starting...", name);
    sleep(Duration::from_secs(secs)).await;
    println!("  {} done!", name);
    format!("{} result", name)
}

async fn sequential_vs_concurrent() {
    // SEQUENTIAL - one after another (slow, 3 seconds total)
    println!("Sequential (watch the timing):");
    let start = std::time::Instant::now();
    
    let _a = do_work("Task A", 1).await;  // wait 1 sec
    let _b = do_work("Task B", 1).await;  // then wait 1 more sec
    let _c = do_work("Task C", 1).await;  // then 1 more sec
    
    println!("  took {:?}", start.elapsed());  // 3 seconds
    
    // CONCURRENT --> all at once (fast, 1 second total)
    println!("Concurrent (much faster):");
    let start = std::time::Instant::now();
    
    let (_a, _b, _c) = join!(
        do_work("Task A", 1),
        do_work("Task B", 1),
        do_work("Task C", 1)
    );
    
    println!("  took {:?}", start.elapsed());  // 1 second
}

// ERROR HANDLING --> works just like sync code with ?
async fn might_fail(succeed: bool) -> Result<String, String> {
    sleep(Duration::from_millis(100)).await;
    
    if succeed {
        Ok("it worked!".to_string())
    } else {
        Err("oops, something broke".to_string())
    }
}

async fn error_handling() {
    // use ? just like normal
    async fn do_stuff() -> Result<(), String> {
        let data = might_fail(true).await?;  // ? works fine
        println!("  got: {}", data);
        
        // this would return early with Err
        // let _fail = might_fail(false).await?;
        
        Ok(())
    }
    
    match do_stuff().await {
        Ok(_) => println!("  success!"),
        Err(e) => println!("  error: {}", e),
    }
    println!();
}

// BORROWING --> yes you can hold references across .await
async fn process(data: &str) -> usize {
    // data is borrowed, and we're about to .await
    // this is fine (but has implications for Send/Sync)
    sleep(Duration::from_millis(50)).await;
    data.len()
}

async fn borrowing_across_await() {
    let my_string = String::from("async world");
    
    // borrow lives across the await point totally valid
    let len = process(&my_string).await;
    
    // original still usable
    println!("  '{}' has {} chars", my_string, len);
}

// ASYNC IN STRUCTS --> common pattern for services
struct ApiClient {
    base_url: String,
}

impl ApiClient {
    fn new(url: &str) -> Self {
        ApiClient { base_url: url.to_string() }
    }
    
    // async methods work great
    async fn get(&self, endpoint: &str) -> Result<String, String> {
        println!("  GET {}{}", self.base_url, endpoint);
        sleep(Duration::from_millis(200)).await;  // fake network delay
        Ok(format!("response from {}", endpoint))
    }
    
    async fn post(&self, endpoint: &str, body: &str) -> Result<String, String> {
        println!("  POST {}{} with body: {}", self.base_url, endpoint, body);
        sleep(Duration::from_millis(200)).await;
        Ok("created".to_string())
    }
}

async fn async_in_structs() {
    let client = ApiClient::new("https://api.example.com");
    
    let users = client.get("/users").await.unwrap();
    println!("  got: {}", users);
    
    let created = client.post("/users", r#"{"name": "bob"}"#).await.unwrap();
    println!("  got: {}", created);
}

// USEFUL PATTERNS you'll actually use
async fn useful_patterns() {
    // TIMEOUT --> don't wait forever
    println!("Timeout:");
    let slow_task = async {
        sleep(Duration::from_secs(10)).await;
        "finally done"
    };
    
    match timeout(Duration::from_secs(1), slow_task).await {
        Ok(result) => println!("  completed: {}", result),
        Err(_) => println!("  timed out! (gave up after 1 sec)"),
    }
    
    // SELECT --> race multiple futures, take first one
    println!(" Select (first one wins):");
    select! {
        _ = sleep(Duration::from_millis(100)) => {
            println!("  timer won!");
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("  this won't print - other was faster");
        }
    }
    
    // SPAWN --> fire and forget (runs in background)
    println!(" Spawn (background task):");
    let handle = spawn(async {
        sleep(Duration::from_millis(50)).await;
        42  // return value
    });
    
    println!("  main continues while task runs...");
    let result = handle.await.unwrap();
    println!("  background task returned: {}", result);
    
    // CHANNELS --> communicate between tasks
    println!(" Channels:");
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    
    // producer task
    spawn(async move {
        for i in 1..=3 {
            tx.send(i).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
        // tx dropped here, closes channel
    });
    
    // consumer
    while let Some(msg) = rx.recv().await {
        println!("  received: {}", msg);
    }
    println!("  channel closed");
}