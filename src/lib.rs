//! WIP svelte parser. Not ready for use yet.

pub mod error;
mod generated;
pub mod parser;
mod state;
pub use swc_ecma_ast as ecma;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use generated::*;
