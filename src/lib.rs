//! Tuples such as Points and Vectors
//!
//! Provides abstractions for Point and Vector types.
//!
//!

#![warn(missing_docs)]

// Dumb stuff to get rid of warning.
#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(not(test))]
extern crate approx;

pub mod canvas;
pub mod vectors;
