#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![doc = core::include_str!("./docs/lib.md")]
#[warn(missing_docs)]
/// The Implentation for football_match
pub mod impls;
/// The Utilities libary for football_match
pub mod libs;
/// The Interface for football_match
pub mod traits;

pub use impls::*;
pub use libs::*;
