//! cargo_unilang library interface
//!
//! This crate is primarily a binary CLI tool. This library module exists
//! to satisfy documentation test requirements in the test framework.
//!
//! The main functionality is in the `cargo_unilang` binary.

#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_hr.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_hr.png")]
#![allow(missing_docs)]
#![allow(missing_debug_implementations)]

// Re-export main modules for documentation purposes
pub mod commands;
pub mod templates;
pub mod checks;
