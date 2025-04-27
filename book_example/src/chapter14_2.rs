//! # chapter14_2
//!
//! `chapter14_2` is a collection of utilities to make performing certain
//! calculations more convenient.

pub mod art;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, crate::book_example::chapter14_2::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
