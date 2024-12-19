//! Executor module for scheduling and running tasks.
//!
//! This module provides the task executor that manages the lifecycle of async tasks.

use std::sync::Arc;
use std::collections::VecDeque;
use std::future::Future;
use std::task::Context;

use crate::task::Task;
use crate::waker::TaskWaker;

/// The task executor that manages task scheduling and execution.
///
/// Executor maintains a queue of tasks and processes them in FIFO order.
pub struct Executor {
    /// Queue of tasks pending execution
    task_queue: VecDeque<Arc<Task>>,
}

impl Executor {
    /// Creates a new executor instance.
    ///
    /// # Returns
    ///
    /// A new empty executor
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    /// Spawns a new future onto the executor.
    ///
    /// # Arguments
    ///
    /// * `future` - The future to execute
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task::new(future));
        self.task_queue.push_back(task);
    }

    /// Runs the executor until all tasks complete.
    ///
    /// This method will continuously poll tasks until they are all finished.
    pub fn run(&mut self) {
        while let Some(task) = self.task_queue.pop_front() {
            let waker = TaskWaker::new(task.clone());
            let mut context = Context::from_waker(&waker);
            
            match task.poll(&mut context) {
                std::task::Poll::Pending => {
                    self.task_queue.push_back(task);
                }
                std::task::Poll::Ready(()) => {}
            }
        }
    }
}