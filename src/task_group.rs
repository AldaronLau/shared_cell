#![forbid(unsafe_code)]

use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Formatter, Result},
    future::Future,
    pin::Pin,
};

use crate::SharedCell;

/// A set of tasks that run together on the same thread, with shared data.
///
/// Requires the _**`alloc`**_ feature.
///
/// Can be used as a building block for concurrent actors.
///
/// # Actor Example (Futures Crate)
///
/// ```rust
#[doc = include_str!("../examples/actor.rs")]
/// ```
pub struct TaskGroup<'a, T, F = Pin<Box<dyn Future<Output = ()> + 'a>>> {
    shared_cell: &'a SharedCell<'a, T>,
    tasks: Vec<F>,
}

impl<T, F> Debug for TaskGroup<'_, T, F>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("TaskGroup")
            .field("shared_cell", &self.shared_cell)
            .field("tasks.len", &self.tasks.len())
            .finish_non_exhaustive()
    }
}

impl<'a, T, F> TaskGroup<'a, T, F>
where
    F: Future<Output = ()> + Unpin + 'a,
{
    /// Create a new [`TaskGroup`].
    pub fn new(shared_cell: &'a SharedCell<'a, T>) -> Self {
        let tasks = Vec::new();

        Self { shared_cell, tasks }
    }

    /// Spawn a task on the [`TaskGroup`].
    pub fn spawn(&mut self, f: impl FnOnce(&'a SharedCell<'a, T>) -> F) {
        self.tasks.push(f(self.shared_cell));
    }

    /// Get a mutable reference to the set of tasks.
    ///
    /// It is up to the library user how to use the list to select and `.await`
    /// from the set of tasks.
    pub fn tasks(&mut self) -> &mut Vec<F> {
        &mut self.tasks
    }

    /// Get the shared cell.
    pub fn shared_cell(&self) -> &'a SharedCell<'a, T> {
        self.shared_cell
    }
}
