use core::cell::Cell;

use traitful::extend;

/// Cell extension trait for thread-local-inspired API
///
/// # Tokio Example
///
/// ```rust
#[doc = include_str!("../examples/tokio.rs")]
/// ```
/// 
/// # Pasts Example
/// ```rust
#[doc = include_str!("../examples/pasts.rs")]
/// ```
#[extend(Cell<T>)]
pub trait CellExt<T> {
    /// Acquires a mutable reference to the cell's interior value.
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R + Send + Sync) -> R {
        unsafe { f(&mut *self.as_ptr()) }
    }
}
