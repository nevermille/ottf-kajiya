#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

/// The errors
pub mod error;
/// The main structure
pub mod kajiya;
/// The OTTF tables
pub mod tables;
/// The traits
pub mod traits;
/// The OTTF var types
pub mod types;
