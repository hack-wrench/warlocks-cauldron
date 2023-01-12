#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]

//! # 🧙‍♀️ Warlock's Cauldron
//! 🦀 Fake Data Generator written in Rust - fully inspired by <https://mimesis.name> 🐍
//! 
//! ## Installation
//! All localizations are enabled by default feature, you can specify localizations in features!
//! ```toml
//! [dependencies.warlocks-cauldron]
//! version = "0.26.2"
//! ## git = "https://github.com/hack-wrench/warlocks-cauldron"
//! ## features = ["en"] # For example to use only english localization
//! ```
//! 
//! ## Supported languages
//! There are currently 26 languages available: `cs, da, de, el, en, es, et, fa, fi, fr, hu, is, it, ja, kk, ko, nl, no, pl, pt, ru, sk, sv, tr, uk, zh`
//! 
//! ## Examples
//! Visit [`/examples`](https://github.com/hack-wrench/warlocks-cauldron/tree/main/examples) for detailed examples. In the process of development it was decided to make the workflow as close to [mimesis](https://mimesis.name) as possible, most of the methods and namespace were taken from there.
//! 
//! ## License
//! This project is licensed under the [GPL-3.0 license](https://github.com/hack-wrench/warlocks-cauldron/blob/main/LICENSE)

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
