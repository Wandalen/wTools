#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Tools for writing and running tests.
//!
// //! # Sample
// //! ```
// //! use wtest_basic::*;
// //!
// //! //
// //!
// //! fn _pass1()
// //! {
// //!   assert_eq!( true, true );
// //! }
// //!
// //! //
// //!
// //! fn _pass2()
// //! {
// //!   assert_eq!( 1, 1 );
// //! }
// //!
// //! //
// //!
// //! test_suite!
// //! {
// //!   pass1,
// //!   pass2,
// //! }
// //! ```
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

pub use wtest_basic as basic;
pub use basic::*;
