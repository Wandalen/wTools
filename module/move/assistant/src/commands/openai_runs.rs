//!
//! Collection of runs commands for OpenAI API.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::{ openai_runs_list, TableConfig };
  
  /// OpenAI runs.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// List OpenAI runs in a thread.
    List
    {
      /// Thread ID.
      thread_id : String,
    },
  }

  /// Execute OpenAI commands related to runs.
  pub async fn command
  (
    client : &Client,
    command : Command,
    table_config : TableConfig,
  )
  {
    match command
    {
      Command::List { thread_id } => 
      {
        openai_runs_list::command( client, thread_id, table_config ).await;
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