# Changelog
All notable changes to `shared_cell` will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.3.0] - 2023-08-17
### Added
 - `SharedCell::duplicate()` method
 - `SharedCell::into_inner()` method
 - `TaskGroup::advance()` method
 - `TaskGroup::is_empty()` method
 - `TaskGroup::into_inner()` method
 - `shared_cell::spawn!()` macro

### Changed
 - Made `SharedCell::new()` safe
 - `SharedCell::with()` now takes a pinned mutable reference
 - `SharedCell` is now `!Send` and `!Unpin`
 - `TaskGroup::new()` now takes a mutable reference instead of a `SharedCell`
 - `TaskGroup::spawn()` is now unsafe, and the closure now takes ownership of
   the `SharedCell`

### Removed
 - `shared_cell!()` macro
 - `SharedCell::with_unchecked()` method
 - `TaskGroup::tasks()` method
 - `TaskGroup::shared_cell()` method

### Fixed
 - More (hopefully all remaining) unsoundness corner-case issues

## [0.2.0] - 2023-08-12
### Added
 - `shared_cell!()` macro
 - `SharedCell` type
   - `new()` associated function
   - `with()` method
   - `with_unchecked()` method

### Removed
 - `CellExt` trait

### Changed
 - `TaskGroup::new()` now takes `&'a SharedCell<'a, T>`
 - Renamed `TaskGroup::shared()` to `TaskGroup::shared_cell()`, and changed
   return type to `&'a SharedCell<'a, T>`

### Fixed
 - Unsoundess issues brought up at <https://users.rust-lang.org/t/announcing-shared-cell-an-additional-cell-api-with-zero-cost-concurrent-data-sharing-in-single-threaded-asynchronous-code/98342>

## [0.1.1] - 2023-08-12 (Yanked)
### Fixed
 - Mistakes in README

## [0.1.0] - 2023-08-12 (Yanked)
### Added
 - `CellExt` extension trait
   - `with()` method
 - `TaskGroup` struct (enabled with `alloc` trait)
   - `new()` associated function
   - `spawn()` method
   - `tasks()` method
   - `shared()` method

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://github.com/AldaronLau/semver/blob/stable/README.md
