pub mod instruction;
pub mod error;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;