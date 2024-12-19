use rust_async_runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_execution() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        let mut runtime = Runtime::new();
        
        runtime.block_on(async move {
            *counter_clone.lock().unwrap() += 1;
        });

        assert_eq!(*counter.lock().unwrap(), 1);
    }

    #[test]
    fn test_runtime_start_stop() {
        let mut runtime = Runtime::new();
        let completed = Arc::new(Mutex::new(false));
        let completed_clone = completed.clone();

        runtime.block_on(async move {
            std::thread::sleep(Duration::from_millis(100));
            *completed_clone.lock().unwrap() = true;
        });

        assert_eq!(*completed.lock().unwrap(), true);
    }

    #[test]
    fn test_executor_polling() {
        let mut runtime = Runtime::new();
        let steps = Arc::new(Mutex::new(Vec::new()));
        let steps_clone = steps.clone();

        runtime.block_on(async move {
            steps_clone.lock().unwrap().push(1);
            // Simulate async work
            std::thread::sleep(Duration::from_millis(50));
            steps_clone.lock().unwrap().push(2);
            std::thread::sleep(Duration::from_millis(50));
            steps_clone.lock().unwrap().push(3);
        });

        assert_eq!(*steps.lock().unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_waker_notification() {
        let mut runtime = Runtime::new();
        let woken = Arc::new(Mutex::new(false));
        let woken_clone = woken.clone();

        runtime.block_on(async move {
            // Create a future that will be woken up
            let future = async {
                std::thread::sleep(Duration::from_millis(50));
                *woken_clone.lock().unwrap() = true;
            };
            future.await;
        });

        assert_eq!(*woken.lock().unwrap(), true);
    }

    #[test]
    fn test_multiple_tasks() {
        let mut runtime = Runtime::new();
        let counter = Arc::new(Mutex::new(0));
        
        let mut handles = vec![];
        for _ in 0..3 {
            let counter_clone = counter.clone();
            handles.push(async move {
                *counter_clone.lock().unwrap() += 1;
            });
        }

        runtime.block_on(async move {
            for handle in handles {
                handle.await;
            }
        });

        assert_eq!(*counter.lock().unwrap(), 3);
    }
}