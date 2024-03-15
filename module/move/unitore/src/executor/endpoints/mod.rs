//! Endpoint for command execution.

pub mod list_fields;
pub mod frames;
pub mod feeds;
pub mod config;
pub mod query;
pub mod table;

/// General report.
pub trait Report : std::fmt::Display + std::fmt::Debug
{
  /// Print report of executed command.
  fn report( &self )
  {
    println!( "{self}" );
  }
}
