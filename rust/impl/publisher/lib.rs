// #![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Library of utility to operate files from a command line.
//!

// #![feature( pattern )]

// use wtools::former::Former;

///
/// Result of parsing.
///

pub mod instruction;

///
/// Properties parsing.
///

pub mod props;

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
