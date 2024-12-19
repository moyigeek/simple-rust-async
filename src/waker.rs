use std::task::{RawWaker, RawWakerVTable, Waker};
use std::sync::Arc;

/// A waker implementation for task notification.
/// 
/// `TaskWaker` is responsible for waking up tasks when they're ready to make progress.
/// It implements the core waking mechanism required by the async runtime.
pub struct TaskWaker {
    task: Arc<super::task::Task>,
}

impl TaskWaker {
    /// Creates a new `Waker` from a task.
    /// 
    /// # Arguments
    /// 
    /// * `task` - The task to create a waker for
    /// 
    /// # Returns
    /// 
    /// A new `Waker` instance that can be used to wake the task
    pub fn new(task: Arc<super::task::Task>) -> Waker {
        let raw = Arc::into_raw(Arc::new(Self { task })) as *const ();
        let vtable = &TaskWaker::VTABLE;
        unsafe { Waker::from_raw(RawWaker::new(raw, vtable)) }
    }

    /// Virtual table for waker operations
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_raw,
        Self::wake_raw,
        Self::wake_by_ref_raw,
        Self::drop_raw,
    );

    /// Clones the raw waker.
    /// 
    /// # Safety
    /// 
    /// The data pointer must be valid for the lifetime of the new `RawWaker`.
    unsafe fn clone_raw(data: *const ()) -> RawWaker {
        let arc = Arc::from_raw(data as *const TaskWaker);
        std::mem::forget(arc.clone());
        RawWaker::new(data, &Self::VTABLE)
    }

    /// Wakes the task associated with this waker.
    /// 
    /// # Safety
    /// 
    /// The data pointer must be valid and point to a properly initialized `TaskWaker`.
    unsafe fn wake_raw(data: *const ()) {
        let arc = Arc::from_raw(data as *const TaskWaker);
        let _ = arc.task.poll(&mut std::task::Context::from_waker(
            &TaskWaker::new(arc.task.clone()),
        ));
    }

    /// Wakes the task by reference.
    /// 
    /// # Safety
    /// 
    /// The data pointer must be valid and point to a properly initialized `TaskWaker`.
    unsafe fn wake_by_ref_raw(data: *const ()) {
        let arc = Arc::from_raw(data as *const TaskWaker);
        let _ = arc.task.poll(&mut std::task::Context::from_waker(
            &TaskWaker::new(arc.task.clone()),
        ));
        std::mem::forget(arc);
    }
    
    /// Drops the waker.
    /// 
    /// # Safety
    /// 
    /// The data pointer must be valid and point to a properly initialized `TaskWaker`.
    unsafe fn drop_raw(data: *const ()) {
        drop(Arc::from_raw(data as *const TaskWaker));
    }
}