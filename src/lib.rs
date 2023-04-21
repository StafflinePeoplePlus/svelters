//! WIP svelte parser. Not ready for use yet.

pub mod error;
mod generated;
pub mod parser;
mod state;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use generated::*;
