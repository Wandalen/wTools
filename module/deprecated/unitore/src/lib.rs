//! Feed client
#![ cfg_attr( not( feature = "enabled" ), allow( unused ) ) ]
pub mod retriever;
pub mod feed_config;
pub mod executor;
pub mod tool;
pub mod command;
pub mod action;
pub mod entity;
pub mod sled_adapter;

/// Empty cell value for table display.
pub const EMPTY_CELL: &str = "";

/// General report trait for commands return type.
pub trait Report: core ::fmt ::Display + core ::fmt ::Debug
{
  /// Print report of executed command.
  fn report( &self )
  {
  println!( "{self}" );
 }
}
