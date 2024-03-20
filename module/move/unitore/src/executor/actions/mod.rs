//! Actions for command execution.

// qqq : reogranize files structure
// there must be folders
//
// action - with all actions
// entity - with all entities
// tool - with something not directly related to the problem, but convenient to have as a separate function/structure

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
