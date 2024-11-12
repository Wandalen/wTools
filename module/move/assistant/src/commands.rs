//!
//! CLI commands of the tool.
//!

/// Internal namespace.
mod private
{

  use clap::{ Parser, Subcommand, ValueEnum };

  use derive_tools::Display;

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

    /// Table style option.
    #[ arg( long, default_value = "table" ) ]
    pub style : TableStyle,

    /// Filter columns of tabular data.
    #[ arg( long, value_delimiter( ',' ) ) ]
    pub filter_columns : Vec< String >,
  }

  /// Table style.
  #[ derive( Debug, Clone, Display, ValueEnum ) ]
  pub enum TableStyle
  {
    /// Show data as an ordinary table.
    Table,

    /// Show entities as separate tables.
    AsRecords,

    /// Show only columns of the tables, no records.
    Columns
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
    TableStyle,
  };
}
