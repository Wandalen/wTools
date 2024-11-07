//!
//! Collection of assistants commands for OpenAI API.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  
  use super::list;

  /// OpenAI assistants.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// List OpenAI assistants.
    List
    {
      /// Show records as separate tables.
      #[ arg( long, default_value_t = false ) ]
      show_records_as_tables : bool
    },
  }

  /// Execute OpenAI command related to assistants.
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
        list::command( client, show_records_as_tables ).await;
      }
    }
  }

}

crate::mod_interface!
{
  layer list;

  own use
  {
    Command,
    command,
  };
}