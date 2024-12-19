//! Runtime module providing the main interface to the async runtime.
//!
//! This module exposes the Runtime type which serves as the primary API for users.

use std::future::Future;
use crate::executor::Executor;

/// The main runtime type for executing async tasks.
///
/// Runtime provides a high-level interface for spawning and managing async tasks.
pub struct Runtime {
    /// The underlying executor that handles task execution
    executor: Executor,
}

impl Runtime {
    /// Creates a new Runtime instance.
    ///
    /// # Returns
    ///
    /// A new Runtime ready to accept tasks
    pub fn new() -> Self {
        Runtime {
            executor: Executor::new(),
        }
    }

    /// Spawns a future onto the runtime.
    ///
    /// # Arguments
    ///
    /// * `future` - The future to spawn
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.executor.spawn(future);
    }

    /// Runs a future to completion, blocking the current thread.
    ///
    /// # Arguments
    ///
    /// * `future` - The future to run to completion
    pub fn block_on<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.spawn(future);
        self.executor.run();
    }
}