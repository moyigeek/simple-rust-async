use rust_async_runtime::Runtime;
use std::time::Duration;
use std::thread;

async fn delayed_print(msg: &str, delay: Duration) {
    std::thread::sleep(delay);
    println!("{}", msg);
}

fn main() {
    let mut runtime = Runtime::new();

    runtime.spawn(delayed_print("Task 1", Duration::from_secs(1)));
    runtime.spawn(delayed_print("Task 2", Duration::from_secs(2)));

    loop {
        runtime.run_non_blocking();
        thread::sleep(Duration::from_millis(100));
    }
}