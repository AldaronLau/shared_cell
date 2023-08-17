use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt::{Debug, Formatter, Result},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::SharedCell;

/// Spawn a task on a [`TaskGroup`], giving it a [`SharedCell`] handle.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/actor.rs")]
/// ```
#[macro_export]
macro_rules! spawn {
    ($tasks: expr, $callback: ident ( $($args: expr),+ $(,)? ) $(,)?) => {{
        let tasks: &mut $crate::TaskGroup<'_, _, _> = &mut $tasks;

        let cb = $callback;

        // SAFETY: The `SharedCell` can't move, as it is pinned
        unsafe {
            tasks.spawn(|data| async move {
                let mut data = core::pin::pin!(data);

                cb(&mut data, $($args),+).await
            });
        }
    }};

    ($tasks: expr, $callback: ident ( ) $(,)?) => {{
        let tasks: &mut $crate::TaskGroup<'_, _, _> = &mut $tasks;

        let cb = $callback;

        // SAFETY: The `SharedCell` can't move, as it is pinned
        unsafe {
            tasks.spawn(|data| async move {
                let mut data = core::pin::pin!(data);

                cb(&mut data).await
            });
        }
    }};
}

/// A set of tasks that run together on the same thread, with shared data.
///
/// Can be used as a building block for concurrent actors, or to share data
/// between multiple tasks without a mutex or borrow-checking.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/task_group.rs")]
/// ```
pub struct TaskGroup<'a, T, R>
where
    T: ?Sized,
{
    tasks: Vec<Pin<Box<dyn Future<Output = R> + 'a>>>,
    shared_cell: SharedCell<'a, T>,
}

impl<T, R> Debug for TaskGroup<'_, T, R>
where
    T: Debug + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("TaskGroup")
            .field("shared_cell", &self.shared_cell)
            .field("tasks.len", &self.tasks.len())
            .finish_non_exhaustive()
    }
}

impl<'a, T, R> TaskGroup<'a, T, R>
where
    T: ?Sized,
{
    /// Create a new [`TaskGroup`].
    pub fn new(value: &'a mut T) -> Self {
        let shared_cell = SharedCell::new(value);
        let tasks = Vec::new();

        Self { shared_cell, tasks }
    }

    /// Advance the execution of tasks within the task group.
    ///
    /// Returns the output of the first task to complete.
    ///
    /// This method attempts minimum-effort fairness, and the future returned is
    /// safe to cancel.  The priorities of subtasks will be reset upon
    /// cancelation or completion of the future.  Completion may alter the
    /// priorities of the remaining subtasks.
    pub async fn advance(&mut self) -> R {
        Tasks(self, 0).await
    }

    /// Return true if the task group is currently empty (no running tasks).
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Cancel all subtasks, returning the inner value.
    pub async fn cancel(mut self) -> &'a mut T {
        self.tasks.clear();

        // SAFETY: There are no more duplicated instances of `SharedCell`
        unsafe { self.shared_cell.into_inner() }
    }

    /// Advance all subtasks until completion, returning the inner value.
    pub async fn finish(mut self) -> &'a mut T {
        while !self.is_empty() {
            drop(self.advance().await);
        }

        // SAFETY: There are no more duplicated instances of `SharedCell`
        unsafe { self.shared_cell.into_inner() }
    }

    /// Spawn a task on the [`TaskGroup`].
    ///
    /// For a safe spawning API, see the [`spawn!()`] macro.
    ///
    /// # Safety
    ///
    ///  - The `SharedCell` must never move outside of the closure.
    pub unsafe fn spawn<A>(&mut self, f: impl FnOnce(SharedCell<'a, T>) -> A)
    where
        A: Future<Output = R> + 'a,
    {
        self.tasks
            .push(Box::pin(f(unsafe { self.shared_cell.duplicate() })));
    }
}

struct Tasks<'a, 'b, T: ?Sized, R>(&'b mut TaskGroup<'a, T, R>, usize);

impl<T: ?Sized, R> Future for Tasks<'_, '_, T, R> {
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<R> {
        let this = self.get_mut();
        let list = &mut this.0.tasks;
        let len = list.len();
        let start = this.1;

        for task in (start..len).chain(0..start) {
            if let Poll::Ready(output) = Pin::new(&mut list[task]).poll(cx) {
                list.swap_remove(task);

                return Poll::Ready(output);
            }
        }

        // Cycle through priorities for "fairness" in long-running poll cycles
        this.1 = if len == 0 { 0 } else { (this.1 + 1) % len };

        Poll::Pending
    }
}
