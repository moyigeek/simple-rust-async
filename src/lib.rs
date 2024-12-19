//! A minimal async runtime implementation for Rust.
//! 
//! This crate provides a simple async runtime with basic task scheduling
//! and execution capabilities. It includes:
//! 
//! - Task management
//! - Custom waker implementation
//! - Basic executor
//! - Runtime interface
//! 
//! # Example
//! ```
//! use rust_async_runtime::Runtime;
//! 
//! async fn hello() {
//!     println!("Hello, async world!");
//! }
//! 
//! let mut runtime = Runtime::new();
//! runtime.block_on(hello());
//! ```

mod executor;
mod task;
mod waker;
mod runtime;

pub use runtime::Runtime;