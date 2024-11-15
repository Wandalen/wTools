//!
//! Collection of files commands for OpenAI API.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::openai_files_list;
  
  /// OpenAI files.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// List OpenAI files.
    List
    {
      /// Show records as separate tables.
      #[ arg( long, default_value_t = false ) ]
      show_records_as_tables : bool
    },
  }

  /// Execute OpenAI commands related to files.
  pub async fn command
  (
    client : &Client,
    command : Command,
  )
  {
    match command
    {
      Command::List{ show_records_as_tables } => 
      {
        openai_files_list::command( client, show_records_as_tables ).await;
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