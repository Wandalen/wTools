mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::openai::
  {
    assistants::*,
    files::*,
    runs::*
  };

  /// OpenAI API commands.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// OpenAI assistants.
    #[ command ( subcommand ) ]
    Assistants
    (
      AssistantsCommand
    ),

    /// OpenAI files.
    #[ command ( subcommand ) ]
    Files
    (
      FilesCommand
    ),

    /// OpenAI runs.
    #[ command ( subcommand ) ]
    Runs
    (
      RunsCommand
    ),
  }

  /// Execute OpenAI command.
  pub async fn execute_command
  (
    client : &Client,
    command : Command,
  )
  {
    match command
    {
      Command::Assistants( assistants_command ) =>
      {
        execute_assistants_command( client, assistants_command ).await;
      }

      Command::Files( files_command ) =>
      {
        execute_files_command( client, files_command ).await;
      }

      Command::Runs( runs_command ) =>
      {
        execute_runs_command( client, runs_command ).await;
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
    execute_command,
  };
}