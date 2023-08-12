#![forbid(unsafe_code)]

use alloc::{boxed::Box, format, vec::Vec};
use core::{
    cell::Cell,
    fmt::{Debug, Formatter, Result},
    future::Future,
    pin::Pin,
};

use crate::CellExt;

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
    shared: &'a Cell<T>,
    set: Vec<F>,
}

impl<T, F> Debug for TaskGroup<'_, T, F>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let set_len = self.set.len();
        let shared = self.shared.with(|shared| format!("{shared:?}"));

        f.debug_struct("TaskGroup")
            .field("shared", &shared)
            .field("set.len", &set_len)
            .finish_non_exhaustive()
    }
}

impl<'a, T, F> TaskGroup<'a, T, F>
where
    F: Future<Output = ()> + Unpin + 'a,
{
    /// Create a new [`TaskGroup`].
    pub fn new(shared: &'a Cell<T>) -> Self {
        let set = Vec::new();

        Self { shared, set }
    }

    /// Spawn a task on the [`TaskGroup`].
    pub fn spawn(&mut self, f: impl FnOnce(&'a Cell<T>) -> F) {
        self.set.push(f(self.shared));
    }

    /// Get a mutable reference to the set of tasks.
    ///
    /// It is up to the library user how to use the list to select and `.await`
    /// from the set of tasks.
    pub fn tasks(&mut self) -> &mut Vec<F> {
        &mut self.set
    }

    /// Get the shared cell.
    pub fn shared(&self) -> &'a Cell<T> {
        self.shared
    }
}
