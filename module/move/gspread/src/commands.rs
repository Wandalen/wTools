//!
//! Commands
//!

pub mod gspread;
pub mod gspread_header;
mod gspread_rows;
mod gspread_cells;

mod private
{

  use clap::{ Parser, Subcommand };

  use crate::*;
  use commands::gspread;

  /// CLI commands of the tool.
  #[ derive ( Debug, Parser ) ]
  pub struct Cli
  {
    /// Root of the CLI commands.
    #[ command ( subcommand ) ]
    pub command : CliCommand,
  }

  /// Root of the CLI commands.
  #[ derive ( Debug, Subcommand ) ]
  pub enum CliCommand
  {
    /// Google Sheets commands.
    #[ command ( subcommand, name = "gspread" ) ]
    GSpread( gspread::Command ),
  }

}

pub use private::
{
  Cli,
  CliCommand,
};


