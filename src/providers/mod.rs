mod dependecies;

#[cfg(feature = "address")]
mod address;

#[cfg(feature = "code")]
mod code;

mod cryptographic;
mod date;
mod development;
mod file;
mod finance;
mod food;
mod hardware;
mod internet;
mod numeric;
mod path;
mod payment;
mod person;
mod science;
mod text;
mod transport;

#[cfg(feature = "address")]
pub use address::{Address, Coordinates, FloatNumber};

#[cfg(feature = "code")]
pub use code::Code;
