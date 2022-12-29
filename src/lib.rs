#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use] extern crate itertools;

#[macro_use] extern crate lazy_static;

pub(crate) mod data;
pub(crate) mod random;

mod providers;
mod enums;
mod pool;

mod macros;

#[cfg(test)]
mod tests;

pub use pool::RandomPool;
pub use providers::*;
pub use enums::*;
