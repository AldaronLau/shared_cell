use core::{
    cell::Cell,
    fmt::{Debug, Formatter, Result},
};

/// Shared cell type
///
/// # Example
///
/// ```
#[doc = include_str!("../examples/shared_cell.rs")]
/// ```
pub struct SharedCell<'a, T: ?Sized>(&'a Cell<T>);

impl<T> Debug for SharedCell<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("SharedCell").field(&format_args!("_"))
            .finish()
    }
}

impl<'a, T> SharedCell<'a, T> {
    /// Create a new [`SharedCell`]
    pub fn new(value: &'a mut T) -> Self {
        Self(Cell::from_mut(value))
    }

    /// Duplicate the [`SharedCell`].
    ///
    /// # Safety
    ///
    ///  - The duplicated [`SharedCell`] may only be used in a scope where no
    ///    other [`SharedCell`] instance is used.
    ///  - The scope containing the duplicated [`SharedCell`] must not have the
    ///    ability to resume execution of an asynchronous task that holds onto
    ///    another [`SharedCell`].
    pub unsafe fn duplicate(&mut self) -> Self {
        Self(self.0)
    }

    /// Acquires a mutable reference to the cell's interior value.
    pub fn with<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        // SAFETY: By isolating the `SharedCell` to one instance per scope, we
        // prevent reëntrant calls to `with()`.
        //
        // SAFETY: Cannot yield to code that could call `with()` due to safety
        // invariant on `duplicate()`
        unsafe { f(&mut *self.0.as_ptr()) }
    }
}
