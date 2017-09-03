// No need for std.
#![no_std]

//! Provides the function `iter_from_closure` intended to quickly transform
//! a mutable closure of type `FnMut() -> Option<Item>` into an
//! `Iterator` where `Item = Item`.
//!
//! This is useful for creating one-time iterators from some state that you
//! have. This is meant to serve as a pain free alternative to the boilerplate
//! of creating a new struct and implementing [`Iterator`] for every such
//! occasion.
//!
//! # Example
//!
//! ```
//! use iter_from_closure::iter_from_closure;
//!
//! let mut count = 5;
//! let iter = iter_from_closure(|| {
//!     let c = count;
//!     count = c - 1;
//!     if c > 0 { Some(c) } else { None }
//! });
//!
//! assert_eq!(vec![5, 4, 3, 2, 1], iter.collect::<Vec<_>>());
//! ```

/// Converts a closure of form `FnMut() -> Option<Item>` into
/// a struct implementing `Iterator<Item = Item>`.
///
/// This is intended for use cases where you want to quickly create an
/// [`Iterator`] for one time use from some state that you have,
/// but where creating a struct to carry the state and then implementing
/// [`Iterator`] is verbose and adds a lot of boilerplate.
///
/// [`Iterator`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
pub fn iter_from_closure<Item, F>(closure: F) -> MutFn<Item, F>
where
    F: FnMut() -> Option<Item>,
{
    MutFn::new(closure)
}

/// An [`Iterator`] implemented by its underlying `FnMut() -> Option<Item>`.
///
/// [`Iterator`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
pub struct MutFn<Item, F>
where
    F: FnMut() -> Option<Item>,
{
    closure: F,
}

impl<Item, F> MutFn<Item, F>
where
    F: FnMut() -> Option<Item>,
{
    /// Creates the `Iterator`.
    fn new(closure: F) -> Self {
        MutFn { closure }
    }
}

impl<Item, F> Iterator for MutFn<Item, F>
where
    F: FnMut() -> Option<Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        (self.closure)()
    }
}