// #![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Library of utility to operate files from a command line.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

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
// pub mod string;
