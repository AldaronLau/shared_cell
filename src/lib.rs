//! #### Interior mutability between concurrent tasks on the same thread
//!
//! --------------------------------------------------------------------
//!
//! This is an alternative to [`RefCell`](core::cell::RefCell) without runtime
//! borrow checking, specifically tailored towards async.  Instead of using
//! borrow guards, uses a closure API inspired by [`LocalKey::with()`] for
//! greater guarantees in the asynchronous context (prevents holding onto the
//! mutable reference over an `.await` point that yields to other tasks that
//! have access to the [`SharedCell`]).
//!
//! [`LocalKey::with()`]: https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.with

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

extern crate alloc;

mod shared_cell;
mod task_group;

pub use self::{
    shared_cell::{Shared, SharedCell},
    task_group::TaskGroup,
};
