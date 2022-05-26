#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

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
