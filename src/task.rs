//! Task module for managing async tasks.
//! 
//! This module provides the core Task type that wraps futures and manages their execution state.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::Mutex;

/// Represents an async task that can be polled to completion.
/// 
/// A Task wraps a future and provides the necessary synchronization primitives
/// to safely poll it from different contexts.
pub struct Task {
    /// The underlying future, wrapped in a mutex for safe concurrent access
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl Task {
    /// Creates a new task from a future.
    ///
    /// # Arguments
    ///
    /// * `future` - The future to wrap in a task
    ///
    /// # Returns
    ///
    /// A new Task instance containing the provided future
    pub fn new<F>(future: F) -> Self 
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task {
            future: Mutex::new(Box::pin(future)),
        }
    }

    /// Polls the underlying future.
    ///
    /// # Arguments
    ///
    /// * `cx` - The context for polling the future
    ///
    /// # Returns
    ///
    /// Poll::Ready(()) when the future completes, Poll::Pending otherwise
    pub fn poll(&self, cx: &mut Context<'_>) -> Poll<()> {
        let mut future = self.future.lock().unwrap();
        future.as_mut().poll(cx)
    }
}