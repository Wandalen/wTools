//!
//! Collection of subcommands fo command "cell"
//!

mod private
{

  use clap::Subcommand;

  use crate::*;
  use actions;
  use actions::gspread::get_spreadsheet_id_from_url;
  use client::SheetsType;

  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    /// Command to get a value from a sheet's cell
    /// 
    /// Command example:
    /// 
    /// gspread cell get
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    /// --cell A1
    #[ command( name = "get" ) ]
    Get
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Cell id. You can set it in format:\n \
      - A1, where A is column name and 1 is row number\n\
      Example: --cell A4" ) ]
      cell : String,
    },

    /// Command to set a new value to a sheet's cell.
    /// 
    /// Command example:
    /// 
    /// gspread cell set
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    /// --cell A1
    /// --val 13
    #[ command( name = "set" ) ]
    Set
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Cell id. You can set it in format:\n \
      - A1, where A is column name and 1 is row number\n\
      Example: --cell A4" ) ]
      cell : String,

      #[ arg( long, help = "Value you want to set. It can be written on any language.\nExample: --val hello" ) ]
      val : String
    }
  }

  pub async fn command
  (
    hub : &SheetsType,
    commands : Commands
  )
  {
    match commands
    {
      Commands::Get { url, tab, cell } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( url.as_str() ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_cell_get::action
        (
          hub,
          spreadsheet_id,
          tab.as_str(),
          cell.as_str()
        )
        .await
        {
          Ok( value ) => println!( "Value: {}", value ),
          Err( error ) => println!( "Error:\n{}", error ),
        }
      },

      Commands::Set { url, tab, cell, val } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( url.as_str() ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_cell_set::action
        (
          hub,
          spreadsheet_id,
          tab.as_str(),
          cell.as_str(),
          val.as_str()
        )
        .await
        {
          Ok( number ) => println!( "You successfully update {} cell!", number ),
          Err( error ) => println!( "Error:\n{}", error ),
        }
      }

    }
  }
}

crate::mod_interface!
{
  own use
  {
    command,
    Commands,
  };
}