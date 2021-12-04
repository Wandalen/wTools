#![warn( missing_docs )]
// #![ feature( concat_idents ) ]

//!
//! wTools - development tools.
//!

pub mod former;
pub mod meta;
pub mod str;
pub mod time;
pub mod vector;

pub use werror;
pub use wtest_basic as test;
