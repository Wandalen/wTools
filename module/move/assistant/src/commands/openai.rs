//!
//! Collection of OpenAI API commands.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::openai::
  {
    assistants,
    files,
    runs,
  };

  /// OpenAI API commands.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// OpenAI assistants.
    #[ command ( subcommand ) ]
    Assistants
    (
      assistants::Command
    ),

    /// OpenAI files.
    #[ command ( subcommand ) ]
    Files
    (
      files::Command
    ),

    /// OpenAI runs.
    #[ command ( subcommand ) ]
    Runs
    (
      runs::Command
    ),
  }

  /// Execute OpenAI command.
  pub async fn command
  (
    client : &Client,
    command : Command,
  )
  {
    match command
    {
      Command::Assistants( assistants_command ) =>
      {
        assistants::command( client, assistants_command ).await;
      }

      Command::Files( files_command ) =>
      {
        files::command( client, files_command ).await;
      }

      Command::Runs( runs_command ) =>
      {
        runs::command( client, runs_command ).await;
      }
    }
  }

}

crate::mod_interface!
{
  layer assistants;
  layer files;
  layer runs;

  orphan use
  {
    Command,
    command,
  };
}