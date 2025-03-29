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

  /// # Cli
  ///
  /// The main structure representing the CLI interface of the tool.
  ///
  /// This struct is the entry point for parsing and handling command-line arguments using the `clap` crate.
  ///
  /// ## Fields:
  /// - `command`:  
  ///   A `CliCommand` enum that specifies the root command and its subcommands.
  #[ derive ( Debug, Parser ) ]
  pub struct Cli
  {
    /// Root of the CLI commands.
    #[ command ( subcommand ) ]
    pub command : CliCommand,
  }

  /// # CliCommand
  ///
  /// An enumeration of all root-level CLI commands.
  ///
  /// Each variant represents a category of commands or a specific functionality the tool provides.
  ///
  /// ## Variants:
  /// - `GSpread`:  
  ///   Handles commands related to Google Sheets (`gspread`).  
  ///   Delegates to the `gspread::Command` for further subcommands and logic.
  #[ derive ( Debug, Subcommand ) ]
  pub enum CliCommand
  {
    #[ command ( subcommand, long_about = "\n\nGoogle Sheets commands.", name = "gspread" ) ]
    GSpread( gspread::Command ),
  }

}

crate::mod_interface!
{
  layer gspread;
  layer gspread_header;
  layer gspread_rows;
  layer gspread_cell;
  layer gspread_row;
  layer gspread_column;
  layer gspread_clear;
  layer gspread_clear_custom;
  layer gspread_copy;

  own use
  {
    Cli,
    CliCommand,
  };
}

