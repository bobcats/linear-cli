//! Re-exports all GraphQL query types from the `linear-queries` crate.
//!
//! This module is a thin shim so that existing `use crate::client::queries::X`
//! imports continue to work unchanged. The actual type definitions live in
//! `crates/linear-queries/src/lib.rs`.

pub use linear_queries::*;
