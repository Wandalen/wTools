//!
//! Collection of files commands for OpenAI API.
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  use commands::{ openai_files_list, TableConfig };
  
  /// OpenAI files.
  #[ derive ( Debug, Subcommand ) ]
  pub enum Command
  {
    /// List OpenAI files.
    List,
  }

  /// Execute OpenAI commands related to files.
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
        openai_files_list::command( client, table_config ).await;
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