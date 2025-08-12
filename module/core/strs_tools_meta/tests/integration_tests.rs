//! Integration tests for `strs_tools_meta` procedural macros
//!
//! # Test Matrix Summary
//! 
//! This file provides the main entry point for integration tests.
//! Detailed Test Matrices are contained in individual test modules:
//! 
//! - `optimize_split_tests`: Tests for `optimize_split` macro
//! - `optimize_match_tests`: Tests for `optimize_match` macro
//!

#[ cfg( feature = "optimize_split" ) ]
mod optimize_split_tests;

#[ cfg( feature = "optimize_match" ) ]
mod optimize_match_tests;