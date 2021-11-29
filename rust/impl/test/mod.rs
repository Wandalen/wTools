#![ warn( missing_docs ) ]
#![ allow( dead_code ) ]
// #![no_std]

//!
//! Tools for writing tests and runnint tests.
//!
//! # Sample
//! ``` rust
//! use wtest::test_suite;
//!
//! //
//!
//! fn _pass1()
//! {
//!   assert_eq!( true, true );
//! }
//!
//! //
//!
//! fn _pass2()
//! {
//!   assert_eq!( 1, 1 );
//! }
//!
//! //
//!
//! test_suite!
//! {
//!   pass1,
//!   pass2,
//! }
//!
//! ```

pub use wtest_basic as basic;
pub use basic::*;
