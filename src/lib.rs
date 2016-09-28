//! A collection of data types for handling time-series data in rust
//!

extern crate num_traits;
extern crate num_integer;
#[macro_use] extern crate quick_error;
#[cfg(test)] #[macro_use] extern crate quickcheck;

pub mod mem;


/// Measure true size of objects
pub trait ByteSize {
    fn size(&self) -> usize;
}
