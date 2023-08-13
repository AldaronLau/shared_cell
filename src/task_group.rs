use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::SharedCell;

/// A set of tasks that run together on the same thread, with shared data.
///
/// Can be used as a building block for concurrent actors.
///
/// # Actor Example (Futures Crate)
///
/// ```rust
#[doc = include_str!("../examples/task_group.rs")]
/// ```
pub struct TaskGroup<'a, T, F = Pin<Box<dyn Future<Output = ()> + 'a>>>
where
    T: ?Sized,
{
    tasks: Vec<F>,
    shared_cell: SharedCell<'a, T>,
}

impl<T, F> Debug for TaskGroup<'_, T, F>
where
    T: Debug + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("TaskGroup")
            .field("shared_cell", &self.shared_cell)
            .field("tasks.len", &self.tasks.len())
            .finish_non_exhaustive()
    }
}

impl<'a, T, F> TaskGroup<'a, T, F>
where
    F: Future<Output = ()> + Unpin + 'a,
    T: ?Sized,
{
    /// Create a new [`TaskGroup`].
    pub fn new(value: &'a mut T) -> Self {
        let shared_cell = SharedCell::new(value);
        let tasks = Vec::new();

        Self { shared_cell, tasks }
    }

    /// Spawn a task on the [`TaskGroup`].
    pub fn spawn(&mut self, f: impl FnOnce(SharedCell<'a, T>) -> F) {
        // SAFETY: SharedCell is only ever exposed from within the closure,
        // meaning it can't be captured from the outside environment.
        self.tasks.push(f(unsafe { self.shared_cell.duplicate() }));
    }

    /// Advance the execution of tasks within the task group.
    pub async fn advance(&mut self) {
        Tasks(&mut self.tasks).await
    }

    /// Return true if no more tasks
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Attempt to convert into `SharedCell`
    pub fn into_shared_cell(self) -> Result<SharedCell<'a, T>, Self> {
        if self.is_empty() {
            Ok(self.shared_cell)
        } else {
            Err(self)
        }
    }
}

struct Tasks<'a, F>(&'a mut Vec<F>);

impl<F> Future for Tasks<'_, F>
where
    F: Future<Output = ()> + Unpin,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        let len = this.0.len();
        let start = 0;

        for task in (start..len).chain(0..start) {
            if let Poll::Ready(output) = Pin::new(&mut this.0[task]).poll(cx) {
                this.0.swap_remove(task);

                return Poll::Ready(output);
            }
        }

        Poll::Pending
    }
}
