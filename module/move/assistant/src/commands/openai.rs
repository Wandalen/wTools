mod private
{

  use crate::*;
  use client::Client;

  use clap::Subcommand;

  /// OpenAI API commands.
  #[ derive ( Debug, Subcommand ) ]
  pub enum OpenAiCommand
  {
    /// OpenAI assistants.
    #[ command ( subcommand ) ]
    Assistants
    (
      OpenAiAssistantsCommand
    ),

    /// OpenAI files.
    #[ command ( subcommand ) ]
    Files
    (
      OpenAiFilesCommand
    ),

    /// OpenAI runs.
    #[ command ( subcommand ) ]
    Runs
    (
      OpenAiRunsCommand
    ),
  }

  /// Execute OpenAI command.
  pub async fn execute_command
  (
    client : &Client,
    command : OpenAiCommand,
  )
  {
    match command
    {
      OpenAiCommand::Assistants( assistants_command ) =>
      {
        execute_assistants_command( client, assistants_command ).await;
      }

      OpenAiCommand::Files( files_command ) =>
      {
        execute_files_command( client, files_command ).await;
      }

      OpenAiCommand::Runs( runs_command ) =>
      {
        execute_runs_command( client, runs_command ).await;
      }
    }
  }

}

crate::mod_interface!
{
  orphan use
  {
    OpenAiCommand,
    execute_command,
  }
}