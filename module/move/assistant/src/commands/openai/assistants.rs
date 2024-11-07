mod private
{

  use clap::Subcommand;

  use crate::*;
  use client::Client;
  
  use super::list;

  /// OpenAI assistants.
  #[ derive ( Debug, Subcommand ) ]
  pub enum AssistantsCommand
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
  pub async fn execute_assistants_command
  (
    client : &Client,
    command : AssistantsCommand,
  )
  {
    match command
    {
      AssistantsCommand::List{ show_records_as_tables } => 
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
    AssistantsCommand,
    execute_assistants_command,
  };
}