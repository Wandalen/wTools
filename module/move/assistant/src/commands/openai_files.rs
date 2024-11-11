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
    List
    {
      /// Configure table formatting.
      #[ clap( flatten ) ]
      table_config : TableConfig,
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
      Command::List{ table_config } => 
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