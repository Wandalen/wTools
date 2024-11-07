//!
//! CLI commands of the tool.
//!

/// Internal namespace.
mod private
{

  use clap::{ Parser, Subcommand };

  use crate::*;
  use client::Client;

  /// CLI commands of the tool.
  #[ derive ( Debug,Parser ) ]
  pub struct Cli
  {
    /// Root of the CLI commands.
    #[ command ( subcommand ) ]
    pub command : CliCommand,
  }

  /// Root of the CLI commands.
  #[ derive ( Debug,Subcommand ) ]
  pub enum CliCommand
  {
    /// OpenAI API commands.
    #[ command ( subcommand, name = "openai" ) ]
    OpenAi(OpenAiCommand),
  }

  /// OpenAI API commands.
  #[ derive ( Debug,Subcommand ) ]
  pub enum OpenAiCommand
  {
    /// OpenAI assistants.
    #[ command ( subcommand ) ]
    Assistants
    (
      OpenAiAssistantsCommand
    ),

    /// OpenAI files.
    #[ command ( subcommand ) ]
    Files
    (
      OpenAiFilesCommand
    ),

    /// OpenAI runs.
    #[ command ( subcommand ) ]
    Runs
    (
      OpenAiRunsCommand
    ),
  }

  /// Execute OpenAI command.
  pub async fn openai
  (
    client : &Client,
    command : OpenAiCommand,
  )
  {
    match command
    {
      OpenAiCommand::Assistants( assistants_command ) =>
      {
        openai_assistants( client, assistants_command ).await;
      }

      OpenAiCommand::Files( files_command ) =>
      {
        openai_files( client, files_command ).await;
      }

      OpenAiCommand::Runs( runs_command ) =>
      {
        openai_runs( client, runs_command ).await;
      }
    }
  }

  /// OpenAI assistants.
  #[ derive ( Debug, Subcommand ) ]
  pub enum OpenAiAssistantsCommand
  {
    /// List OpenAI assistants.
    List
    {
      /// Show records as separate tables.
      #[arg(long, default_value_t = false)]
      show_records_as_tables : bool
    },
  }

  /// Execute OpenAI command related to assistants.
  pub async fn openai_assistants
  (
    client : &Client,
    command : OpenAiAssistantsCommand,
  )
  {
    match command
    {
        OpenAiAssistantsCommand::List{ show_records_as_tables } => 
        {
          openai_list_assistants( client, show_records_as_tables ).await;
        }
    }
  }

  /// List OpenAI assistants.
  pub async fn openai_list_assistants
  ( 
    client : &Client,
    show_records_as_tables : bool,
  )
  {
    let result = actions::openai_list_assistants( client, show_records_as_tables ).await;

    match result
    {
      Ok ( report ) => println!( "{}", report ),
      Err ( error ) => println!( "{}", error )
    }
  }

  /// OpenAI files.
  #[ derive ( Debug,Subcommand ) ]
  pub enum OpenAiFilesCommand
  {
    /// List OpenAI files.
    List
    {
      /// Show records as separate tables.
      #[arg(long, default_value_t = false)]
      show_records_as_tables : bool
    },
  }

  /// Execute OpenAI commands related to files.
  pub async fn openai_files
  (
    client : &Client,
    command : OpenAiFilesCommand,
  )
  {
    match command
    {
      OpenAiFilesCommand::List{ show_records_as_tables } => 
      {
        openai_list_files( client, show_records_as_tables ).await;
      }
    }
  }

  /// List files in your OpenAI API.
  pub async fn openai_list_files
  ( 
    client : &Client,
    show_records_as_tables : bool,
  )
  {
    let result = actions::openai_list_files( client, show_records_as_tables ).await;

    match result
    {
      Ok ( report ) => println!( "{}", report ),
      Err ( error ) => println!( "{}", error )
    }
  }

  /// OpenAI runs.
  #[ derive ( Debug,Subcommand ) ]
  pub enum OpenAiRunsCommand
  {
    /// List OpenAI runs in a thread.
    List
    {
      /// Thread ID.
      thread_id : String,

      /// Show records as separate tables.
      #[arg(long, default_value_t = false)]
      show_records_as_tables : bool
    },
  }

  /// Execute OpenAI commands related to runs.
  pub async fn openai_runs
  (
    client : &Client,
    command : OpenAiRunsCommand,
  )
  {
    match command
    {
      OpenAiRunsCommand::List { thread_id, show_records_as_tables } => 
      {
        openai_list_runs( client, thread_id, show_records_as_tables ).await;
      }
    }
  }

  /// List runs in the thread in OpenAI API.
  pub async fn openai_list_runs
  ( 
    client : &Client, 
    thread_id : String,
    show_records_as_tables : bool,
  )
  {
    let result = actions::openai_list_runs( client, thread_id, show_records_as_tables ).await;

    match result
    {
      Ok ( report ) => println!( "{}", report ),
      Err ( error ) => println!( "{}", error )
    }
  }

}

crate::mod_interface!
{
  exposed use
  {
    Cli,
    CliCommand,
    OpenAiCommand,
    OpenAiAssistantsCommand,
    OpenAiFilesCommand,
    OpenAiRunsCommand,
    openai,
    openai_assistants,
    openai_list_assistants,
    openai_files,
    openai_list_files,
    openai_runs,
    openai_list_runs,
  };
}
