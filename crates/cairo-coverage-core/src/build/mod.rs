//! This module is responsible for **transforming** already loaded data into
//! [`CoverageInput`](coverage_input::CoverageInput).
//!
//! ## Naming Conventions
//! - Functions begin with the `build` prefix.
//! - Prefer standalone functions over struct-based factory methods to
//!   avoid wrapping or redefining external types unnecessarily.
pub mod coverage_input;
mod executed_statement_count;
pub mod filter;
pub mod statement_information;
