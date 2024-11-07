mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  
  use super::list;

  /// OpenAI files.
  #[ derive ( Debug, Subcommand ) ]
  pub enum FilesCommand
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
  pub async fn execute_files_command
  (
    client : &Client,
    command : FilesCommand,
  )
  {
    match command
    {
      FilesCommand::List{ show_records_as_tables } => 
      {
        list( client, show_records_as_tables ).await;
      }
    }
  }

}

crate::mod_interface!
{
  layer list;

  orphan use
  {
    FilesCommand,
    execute_files_command,
  };
}