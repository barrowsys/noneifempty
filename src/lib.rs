/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */
//! Adds a trait NoneIfEmpty that converts a T to an Option<T> by turning an empty T into None.
//! ```
//! // Bring the trait into scope
//! use noneifempty::NoneIfEmpty;
//!
//! // Converts empty strings to None
//! let empty_str = "";
//! assert_eq!(empty_str.none_if_empty(), None);
//!
//! // And full strings to Some
//! let full_str  = "hello, world!";
//! assert_eq!(full_str.none_if_empty(),  Some("hello, world!"));
//!
//! // Also works with vecs, hashmaps, hashsets, custom types...
//! let empty_vec: Vec<&str> = vec![];
//! let full_vec:  Vec<&str> = vec!["hi"];
//! assert_eq!(empty_vec.none_if_empty(), None);
//! assert_eq!(full_vec.none_if_empty(),  Some(vec!["hi"]));
//!
//! // Automatically implemented for Option<T: NoneIfEmpty>
//! let no_vec:    Option<Vec<&str>>  = None;
//! let empty_vec: Option<Vec<&str>>  = Some(vec![]);
//! let full_vec:  Option<Vec<&str>>  = Some(vec!["hi"]);
//! assert_eq!(no_vec.none_if_empty(),    None);
//! assert_eq!(empty_vec.none_if_empty(), None);
//! assert_eq!(full_vec.none_if_empty(),  Some(vec!["hi"]));
//! ```

use std::collections::{HashMap, HashSet};

/// Trait for converting an empty T to None
pub trait NoneIfEmpty
where
    Self: Sized,
{
    type Output;
    fn none_if_empty(self) -> Option<Self::Output>;
}
/// Automatically implement NoneIfEmpty for Option<T: NoneIfEmpty>
///
/// This is basically a flatten
impl<N: NoneIfEmpty> NoneIfEmpty for Option<N> {
    type Output = <N as NoneIfEmpty>::Output;
    fn none_if_empty(self) -> Option<Self::Output> {
        (self?).none_if_empty()
    }
}
/// Converts a &str to None if empty
impl<'s> NoneIfEmpty for &'s str {
    type Output = &'s str;
    fn none_if_empty(self) -> Option<&'s str> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
/// Converts a String to None if empty
impl NoneIfEmpty for String {
    type Output = String;
    fn none_if_empty(self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
/// Converts a Vec to None if empty
impl<T> NoneIfEmpty for Vec<T> {
    type Output = Self;
    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
/// Converts a HashMap to None if empty
impl<K, V> NoneIfEmpty for HashMap<K, V> {
    type Output = Self;
    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
/// Converts a HashSet to None if empty
impl<T> NoneIfEmpty for HashSet<T> {
    type Output = Self;
    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
