mod private
{

  use crate::*;
  use client::Client;

  /// OpenAI runs.
  #[ derive ( Debug,Subcommand ) ]
  pub enum RunsCommand
  {
    /// List OpenAI runs in a thread.
    List
    {
      /// Thread ID.
      thread_id : String,

      /// Show records as separate tables.
      #[ arg( long, default_value_t = false ) ]
      show_records_as_tables : bool
    },
  }

  /// Execute OpenAI commands related to runs.
  pub async fn execute_runs_command
  (
    client : &Client,
    command RunsCommand,
  )
  {
    match command
    {
      RunsCommand::List { thread_id, show_records_as_tables } => 
      {
        list( client, thread_id, show_records_as_tables ).await;
      }
    }
  }

}

crate::mod_interface!
{
  orphan use
  {
    RunsCommand,
    execute_runs_command,
  }
}