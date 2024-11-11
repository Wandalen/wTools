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
    List,
  }

  /// Execute OpenAI command related to assistants.
  pub async fn command
  (
    client : &Client,
    command : Command,
    table_config : TableConfig,
  )
  {
    match command
    {
      Command::List => 
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