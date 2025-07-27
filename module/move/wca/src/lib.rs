#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/wca/latest/wca/")]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "doc/", "wca.md" ) ) ]

use mod_interface::mod_interface;

pub mod ca;

mod private {}

crate::mod_interface! {
  exposed use ca::grammar;
  exposed use ca::parser;
  exposed use ca::verifier;
  exposed use ca::executor;
  exposed use ca::input;
  exposed use ca::tool;
  exposed use ca::aggregator;
  exposed use ca::help;
  exposed use ca::formatter;
  
  // Re-export commonly used types at root level
  exposed use ca::aggregator::{ CommandsAggregator, Order, Error, ValidationError };
  exposed use ca::grammar::{ Type, Value, Command, Dictionary, types::TryCast };
  exposed use ca::verifier::VerifiedCommand;
  exposed use ca::executor::Executor;
  exposed use ca::input::{ Input, IntoInput };
  exposed use ca::help::HelpVariants;
}
