//! The test suite for the Unilang crate.
//!
//! This is the main test aggregator that organizes all tests according to the
//! systematic test organization structure defined in readme.md.
//!
//! ## Test Organization
//! - `unit` - Individual component testing
//! - `integration` - Component interaction testing
//! - `acceptance` - User scenario testing
//! - `regression` - Bug prevention testing
//! - `inc` - Legacy incremental tests (maintained for compatibility)

// Legacy incremental test structure (maintained for compatibility)
mod inc;

// New organized test structure
mod unit;
mod integration;
mod acceptance;
mod regression;
