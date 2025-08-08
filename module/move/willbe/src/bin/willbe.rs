#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/willbe/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Build and project management binary" ) ]

//! # willbe Binary Entry Point
//!
//! This binary provides the primary entry point for the willbe build and project management tool.
//! Following Design Rulebook principles:
//!
//! - Uses explicit error handling with proper Result types
//! - Delegates main functionality to library code for better testability
//! - Uses proper attribute formatting per Codestyle Rulebook

#[ allow( unused_imports, clippy::wildcard_imports ) ]
use ::willbe::*;

fn main() -> Result< (), error::untyped::Error >
{
  willbe::run( std::env::args().collect() )
}
