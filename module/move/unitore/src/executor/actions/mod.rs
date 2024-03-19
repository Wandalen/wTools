//! Endpoint for command execution.

pub mod frame;
pub mod feed;
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
