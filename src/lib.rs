//! #### Interior mutability between concurrent tasks on the same thread
//!
//! --------------------------------------------------------------------
//!
//! This is essentially an alternative to [`RefCell`](core::cell::RefCell)
//! without runtime borrow checking.  Instead of using borrow guards, uses a
//! closure API inspired by [`LocalKey::with()`] for greater guarantees in the
//! asynchronous context (prevents holding onto the mutable reference over an
//! `.await` point that yields to other tasks that have access to the [`Cell`]).
//!
//! # How It Works / Why It's Safe
//! A [`Cell`] makes it possible to have multiple references data with interior
//! mutability.  Being `!Sync`, it is impossible to call methods on [`Cell`]
//! from another thread, preventing data races.
//!
//! The lifetime of the mutable reference is bound by the closure's scope,
//! making it impossible to [`drop()`] the interior data while borrowed.  Since
//! [`Cell`] doesn't let you get an immutable reference to the interior data,
//! not having any existing immutable references is guaranteed, making it safe
//! to construct a mutable reference to pass into the closure.
//!
//! Taking advantage of the fact that [`Cell`] is `!Sync`, by requiring [`Sync`]
//! in the closure provided to [`CellExt::with()`], it is impossible to create a
//! second mutable reference to the data through a reëntrant borrow.
//!
//! ## Reëntrant Borrow Prevention Example
//! ```rust
#![doc = include_str!("../examples/hello.rs")]
//! ```
//! 
//! [`LocalKey::with()`]: https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.with
//! [`Cell`]: core::cell::Cell

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/shared_cell"
)]
#![no_std]
#![forbid(missing_docs)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod cell;

#[cfg(feature = "alloc")]
mod task_group;

pub use self::cell::CellExt;
#[cfg(feature = "alloc")]
pub use self::task_group::TaskGroup;
