//!
//! Commands
//!


mod private
{

  use clap::
  {
    Parser,
    Subcommand
  };

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

crate::mod_interface!
{
  layer gspread;
  layer gspread_header;
  layer gspread_rows;
  layer gspread_cell;
  layer gspread_cells;

  own use
  {
    Cli,
    CliCommand,
  };
}

