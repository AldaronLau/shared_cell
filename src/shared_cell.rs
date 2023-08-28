use core::{
    cell::Cell,
    fmt::{Debug, Formatter, Result},
    marker::PhantomPinned,
    pin::Pin,
};

/// Type alias for pinned [`SharedCell`]
pub type Shared<'a, T> = Pin<&'a mut SharedCell<'a, T>>;

/// Shared cell type
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/shared_cell.rs")]
/// ```
/// 
/// The code will not compile if you try to get two mutable references:
/// ```rust,compile_fail
#[doc = include_str!("../examples/should_fail/shared_cell.rs")]
/// ```
pub struct SharedCell<'a, T: ?Sized>(&'a Cell<T>, PhantomPinned);

impl<T: ?Sized> Debug for SharedCell<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("SharedCell")
            .field(&format_args!("_"))
            .finish()
    }
}

impl<'a, T: ?Sized> SharedCell<'a, T> {
    /// Create a new [`SharedCell`]
    pub fn new(value: &'a mut T) -> Self {
        Self(Cell::from_mut(value), PhantomPinned)
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
    ///  - The duplicated [`SharedCell`] must not be moved by value (includes
    ///    calls to [`SharedCell::into_inner()`]), unless there is only one
    ///    instance remaining.
    pub unsafe fn duplicate(&mut self) -> Self {
        Self(self.0, PhantomPinned)
    }

    /// Acquire a mutable reference to the cell's interior value.
    pub fn with<R>(
        self: &mut Pin<&mut Self>,
        f: impl FnOnce(&mut T) -> R,
    ) -> R {
        // SAFETY: By isolating the `SharedCell` to one instance per scope, we
        // prevent reëntrant calls to `with()`.
        //
        // SAFETY: Cannot yield to code that could call `with()` due to safety
        // invariant on `duplicate()`.
        unsafe { f(&mut *self.0.as_ptr()) }
    }

    /// Return a mutable reference to the internal data.
    pub fn into_inner(self) -> &'a mut T {
        unsafe { &mut *self.0.as_ptr() }
    }
}
