#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use] extern crate itertools;

#[macro_use] extern crate lazy_static;

extern crate strum_macros;
extern crate strum;

pub mod enums;
mod data;
mod macros;

#[cfg(test)]
mod tests;
