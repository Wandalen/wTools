//!
//! Collection of assistants commands for OpenAI API.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::{ openai_assistants_list, TableConfig };
  
  /// OpenAI assistants.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// List OpenAI assistants.
    List
    {
      /// Configure table formatting.
      #[ clap( flatten ) ]
      table_config : TableConfig,
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
      Command::List{ table_config } => 
      {
        openai_assistants_list::command( client, table_config ).await;
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