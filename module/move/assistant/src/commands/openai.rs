//!
//! Collection of OpenAI API commands.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::{ openai_assistants, openai_files, openai_runs };

  /// OpenAI API commands.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// OpenAI assistants.
    #[ command ( subcommand ) ]
    Assistants
    (
      openai_assistants::Command
    ),

    /// OpenAI files.
    #[ command ( subcommand ) ]
    Files
    (
      openai_files::Command
    ),

    /// OpenAI runs.
    #[ command ( subcommand ) ]
    Runs
    (
      openai_runs::Command
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
        openai_assistants::command( client, assistants_command ).await;
      }

      Command::Files( files_command ) =>
      {
        openai_files::command( client, files_command ).await;
      }

      Command::Runs( runs_command ) =>
      {
        openai_runs::command( client, runs_command ).await;
      }
    }
  }

}

crate::mod_interface!
{
  own use
  {
    Command,
    command,
  };
}