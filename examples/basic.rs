use std::time::Duration;
use rust_async_runtime::Runtime;

async fn async_print(id: u32) {
    println!("Task {} started", id);
    // Simulate some async work
    std::thread::sleep(Duration::from_secs(1));
    println!("Task {} completed", id);
}

fn main() {
    let mut runtime = Runtime::new();
    
    runtime.block_on(async {
        async_print(1).await;
        async_print(2).await;
    });
}