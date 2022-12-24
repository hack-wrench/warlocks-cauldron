pub use std::collections::HashMap;
pub use crate::{dict, list};

pub type List = Vec<String>;
pub type ListDict = HashMap<&'static str, List>;
pub type StrDict = HashMap<&'static str, &'static str>;
pub type RecDict = HashMap<&'static str, StrDict>;
