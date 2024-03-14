//!
//! Utility to publish multi-crate and multi-workspace environments and maintain their consistency.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ allow( unused_imports ) ]
use::willbe::*;

fn main() -> Result< (), wtools::error::for_app::Error >
{
  Ok( willbe::run( std::env::args().collect() )? )
}
