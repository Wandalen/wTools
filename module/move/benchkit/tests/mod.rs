//! Test suite organization for benchkit
//!
//! This module organizes all tests following Test-Driven Development principles
//! and the Test Matrix approach from the Design Rulebook.

// Import everything needed for tests
pub use benchkit::prelude::*;
pub use std::time::{Duration, Instant};
pub use std::collections::HashMap;

// Test modules organized by functionality
pub mod timing_tests;
pub mod generators_tests;  
pub mod reports_tests;
pub mod suite_tests;
pub mod analysis_tests;
pub mod integration_tests;