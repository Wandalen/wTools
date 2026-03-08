#![ doc( html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https: //docs.rs/wca/latest/wca/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Command line argument parsing and processing library" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "doc/", "wca.md" ) ) ) ]

//! # Rule Compliance & Architectural Notes
//!
//! This crate implements command line argument parsing and processing library with
//! systematic compliance to the Design and Codestyle Rulebooks.
//!
//! ## Completed Compliance Work :
//!
//! 1. **`mod_interface` Architecture** : Uses `mod_interface!` macro for clean module
//!    organization and controlled visibility per architectural guidelines.
//!
//! 2. **Documentation Strategy** : Uses both readme.md inclusion and specialized
//!    documentation from `doc/wca.md` for comprehensive coverage.
//!
//! 3. **Attribute Formatting** : All attributes use proper spacing per Universal Formatting Rule.
//!
//! 4. **Explicit Exposure** : Lists all exposed items explicitly in `mod_interface!`
//!    following the explicit exposure rule.

use mod_interface ::mod_interface;

pub mod ca;

mod private {}

crate ::mod_interface!
{
  exposed use ca ::grammar;
  exposed use ca ::parser;
  exposed use ca ::verifier;
  exposed use ca ::executor;
  exposed use ca ::input;
  exposed use ca ::tool;
  exposed use ca ::aggregator;
  exposed use ca ::help;
  exposed use ca ::formatter;
  
  // Re-export commonly used types at root level
  exposed use ca ::aggregator :: { CommandsAggregator, Order, Error, ValidationError };
  exposed use ca ::grammar :: { Type, Value, Command, Dictionary, types ::TryCast };
  exposed use ca ::verifier ::VerifiedCommand;
  exposed use ca ::executor ::Executor;
  exposed use ca ::input :: { Input, IntoInput };
  exposed use ca ::help ::HelpVariants;
}
