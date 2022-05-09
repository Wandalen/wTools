#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Library of utility to operate packages from a command line.
//!

///
/// Work with bools.
///

pub mod bool;

///
/// Manipulate over files.
///

pub mod files;

///
/// Manipulate over manifest.
///

pub mod manifest;

///
/// Work with crate on `crates.io`.
///

pub mod http;

///
/// Run external processes.
///

pub mod process;

///
/// Make sha-1 hash for data.
///

pub mod digest;
