//! This crate contains core primitives, traits, and types for Hyperlane
//! implementations.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![forbid(where_clauses_object_safety)]

extern crate core;

pub use chain::*;
pub use error::{ChainCommunicationError, ChainResult, HyperlaneProtocolError};
pub use identifiers::HyperlaneIdentifier;
pub use traits::*;
pub use types::*;

/// Accumulator management
pub mod accumulator;

/// Async Traits for contract instances for use in applications
mod traits;
/// Utilities to match contract values
pub mod utils;

/// Testing utilities
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

pub mod config;
/// Prometheus metrics traits / utilities
pub mod metrics;

/// Core hyperlane system data structures
mod types;

mod chain;
mod error;

/// Enum for validity of a list of messages
#[derive(Debug)]
pub enum ListValidity {
    /// Empty list
    Empty,
    /// Valid list
    Valid,
    /// Invalid list. Does not build upon the correct prior element.
    InvalidContinuation,
    /// Invalid list. Contains gaps, but builds upon the correct prior element.
    ContainsGaps,
}
