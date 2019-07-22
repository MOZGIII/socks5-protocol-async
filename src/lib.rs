#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
#![feature(async_await)]

#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate futures;

#[macro_use]
extern crate enum_primitive_derive;

pub mod error;
pub mod protocol;

#[cfg(test)]
mod test_util;
