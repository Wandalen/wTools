//!
//! CLI commands of the tool.
//!

/// Internal namespace.
mod private
{

  use std::error::Error;

  use clap::{ Parser, Subcommand };

  use crate::*;
  use commands::openai;

  /// CLI commands of the tool.
  #[ derive ( Debug, Parser ) ]
  pub struct Cli
  {
    /// Root of the CLI commands.
    #[ command ( subcommand ) ]
    pub subcommand : CliCommand,

    /// Table view configuration.
    #[ command ( flatten ) ]
    pub table_config : TableConfig,
  }

  const DEFAULT_MAX_TABLE_WIDTH: usize = 130;

  /// Table view configuration.
  #[ derive( Debug, Parser ) ]
  pub struct TableConfig
  {
    /// Show records as separate tables.
    #[ arg( long, default_value_t = false ) ]
    pub as_records : bool,

    /// Limit table width.
    #[ arg( long, default_value_t = DEFAULT_MAX_TABLE_WIDTH ) ]
    pub max_table_width : usize,
  }

  /// Root of the CLI commands.
  #[ derive ( Debug, Subcommand ) ]
  pub enum CliCommand
  {
    /// OpenAI API commands.
    #[ command ( subcommand, name = "openai" ) ]
    OpenAi( openai::Command ),
  }

  /// Execute CLI command.
  pub async fn execute( command : Cli ) -> Result< (), Box< dyn Error > >
  {
    let secret = Secret::load()?;

    let client = client::client( &secret )?;

    match command.subcommand
    {
      CliCommand::OpenAi( openai_command ) =>
      {
        commands::openai::command( &client, openai_command, command.table_config ).await;
      }
    }

    Ok( () )
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
    execute,
  };
}
