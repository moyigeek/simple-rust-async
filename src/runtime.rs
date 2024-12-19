use std::future::Future;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::executor::Executor;
use crate::task::Task;

pub struct Runtime {
    executor: Executor,
    task_sender: Sender<Arc<Task>>,
    task_receiver: Receiver<Arc<Task>>,
    timer_wheel: TimerWheel,
}

struct TimerWheel {
    timers: Vec<(Instant, Arc<Task>)>,
}

impl Runtime {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Runtime {
            executor: Executor::new(),
            task_sender: tx,
            task_receiver: rx,
            timer_wheel: TimerWheel { timers: Vec::new() },
        }
    }

    pub fn spawn<F>(&self, future: F) 
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task::new(future));
        self.task_sender.send(task).expect("Failed to send task");
    }

    pub fn spawn_after<F>(&mut self, future: F, delay: Duration)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task::new(future));
        let deadline = Instant::now() + delay;
        self.timer_wheel.timers.push((deadline, task));
    }

    pub fn run_non_blocking(&mut self) {
        // Process ready tasks
        while let Ok(task) = self.task_receiver.try_recv() {
            self.executor.spawn_task(task);
        }

        // Check timers
        let now = Instant::now();
        self.timer_wheel.timers.retain(|(deadline, task)| {
            if *deadline <= now {
                self.executor.spawn_task(task.clone());
                false
            } else {
                true 
            }
        });

        // Run one iteration of executor
        self.executor.run_once();
    }
}