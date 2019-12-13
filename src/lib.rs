#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate futures;

pub mod error;
pub mod protocol;

#[cfg(test)]
mod test_util;
