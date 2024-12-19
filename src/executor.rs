use std::sync::Arc;
use std::collections::VecDeque;
use std::task::{Context, Poll};
use crate::task::Task;
use crate::waker::TaskWaker;

pub struct Executor {
    task_queue: VecDeque<Arc<Task>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new()
        }
    }

    pub fn spawn_task(&mut self, task: Arc<Task>) {
        self.task_queue.push_back(task);
    }

    pub fn run_once(&mut self) -> bool {
        if let Some(task) = self.task_queue.pop_front() {
            let waker = TaskWaker::new(task.clone());
            let mut context = Context::from_waker(&waker);
            
            match task.poll(&mut context) {
                Poll::Pending => {
                    self.task_queue.push_back(task);
                    true
                }
                Poll::Ready(()) => true
            }
        } else {
            false
        }
    }
}