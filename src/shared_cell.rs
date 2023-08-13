use core::{
    cell::UnsafeCell,
    fmt::{Debug, Formatter, Result},
};

/// Construct a `SharedCell<'_, T>`, by locally constructing a `value: T`.
#[macro_export]
macro_rules! shared_cell {
    ($($value:ident),* $(,)?) => { $(
        // SAFETY: Move and shadow the variable to prevent access
        let mut $value = $value;
        let $value = &unsafe { $crate::SharedCell::new(&mut $value) };
    )* };
}

/// Shared cell type
///
/// # Tokio Example
///
/// ```
#[doc = include_str!("../examples/tokio.rs")]
/// ```
/// 
/// # Pasts Example
/// ```
#[doc = include_str!("../examples/pasts.rs")]
/// ```
pub struct SharedCell<'a, T: ?Sized>(&'a mut UnsafeCell<T>);

impl<T> Debug for SharedCell<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            self.with_unchecked(|sc| {
                f.debug_tuple("SharedCell").field(sc).finish()
            })
        }
    }
}

impl<'a, T> SharedCell<'a, T> {
    /// Create a new [`SharedCell`]
    ///
    /// # Safety
    ///
    ///  - `value` must be a locally constructed value.
    pub unsafe fn new(value: &'a mut T) -> Self {
        // WAITING FOR: https://github.com/rust-lang/rust/issues/111645
        let value: *mut T = value;
        let value: *mut UnsafeCell<T> = value.cast();
        // SAFETY: `UnsafeCell<T>` has the same memory layout as `T` due to
        // `#[repr(transparent)]`.
        let value = &mut *value;

        Self(value)
    }

    /// Acquires a mutable reference to the cell's interior value.
    pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R + 'static) -> R {
        // SAFETY: Locally-constructed values are guaranteed to be not
        // `'static`; By requiring `'static` we prevent reëntrant calls to
        // `with()`.
        //
        // SAFETY: Yielding to code that could call `with()` is also impossible
        // because the async executor handle would need to have to hold onto a
        // not `'static` future, making the executor inaccessible during this
        // method.
        unsafe { self.with_unchecked(f) }
    }

    /// Acquires a mutable reference to the cell's interior value.
    ///
    /// # Safety
    ///
    ///  - Must not call [`SharedCell::with()`] or
    ///  [`SharedCell::with_unchecked()`] from within the provided closure.
    ///  - Must not yield to code that can do the above.
    pub unsafe fn with_unchecked<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut *self.0.get())
    }
}
