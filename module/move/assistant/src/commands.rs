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
  };
}
