//!
//! CLI commands of the tool.
//!

/// Internal namespace.
mod private
{

  use clap::{ Parser, Subcommand };

  use crate::*;
  use commands::openai;

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
    /// OpenAI API commands.
    #[ command ( subcommand, name = "openai" ) ]
    OpenAi( openai::Command ),
  }

  const DEFAULT_MAX_TABLE_WIDTH: usize = 130;

  /// Common collection of arguments for formatting tabular data.
  #[ derive( Debug, Parser ) ]
  pub struct TableConfig
  {
    /// Limit table widht.
    #[ arg( long, default_value_t = DEFAULT_MAX_TABLE_WIDTH ) ]
    pub max_table_width : usize,

    /// Show records as separate tables.
    #[ arg( long, default_value_t = false ) ]
    pub as_records : bool,
  }

}

crate::mod_interface!
{
  layer openai;
  layer openai_assistants;
  layer openai_assistants_list;
  layer openai_runs;
  layer openai_runs_list;
  layer openai_files;
  layer openai_files_list;

  own use
  {
    Cli,
    CliCommand,
    TableConfig,
  };
}
