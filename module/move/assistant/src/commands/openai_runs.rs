//!
//! Collection of runs commands for OpenAI API.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::openai_runs_list;
  
  /// OpenAI runs.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
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
  pub async fn command
  (
    client : &Client,
    command : Command,
  )
  {
    match command
    {
      Command::List { thread_id, show_records_as_tables } => 
      {
        openai_runs_list::command( client, thread_id, show_records_as_tables ).await;
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