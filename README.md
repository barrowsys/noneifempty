# noneifempty

[![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/noneifempty)](https://github.com/barrowsys/noneifempty)
[![Crates.io](https://img.shields.io/crates/v/noneifempty)](https://crates.io/crates/noneifempty/)
[![Docs.rs](https://docs.rs/noneifempty/badge.svg)](https://docs.rs/noneifempty)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

## About

Adds a trait NoneIfEmpty that converts a T to an Option<T> by turning an empty T into None.

## Usage

Add to your Cargo.toml:
```
[dependencies]
noneifempty = "0.1.0"
```

## Examples
```
// Bring the trait into scope
use noneifempty::NoneIfEmpty;

// Converts empty strings to None
let empty_str = "";
assert_eq!(empty_str.none_if_empty(), None);

// And full strings to Some
let full_str  = "hello, world!";
assert_eq!(full_str.none_if_empty(),  Some("hello, world!"));

// Also works with vecs, hashmaps, hashsets, custom types...
let empty_vec: Vec<&str> = vec![];
let full_vec:  Vec<&str> = vec!["hi"];
assert_eq!(empty_vec.none_if_empty(), None);
assert_eq!(full_vec.none_if_empty(),  Some(vec!["hi"]));

// Automatically implemented for Option<T: NoneIfEmpty>
let no_vec:    Option<Vec<&str>>  = None;
let empty_vec: Option<Vec<&str>>  = Some(vec![]);
let full_vec:  Option<Vec<&str>>  = Some(vec!["hi"]);
assert_eq!(no_vec.none_if_empty(),    None);
assert_eq!(empty_vec.none_if_empty(), None);
assert_eq!(full_vec.none_if_empty(),  Some(vec!["hi"]));
```

