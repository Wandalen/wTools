//!
//! Cells commands.
//! set command -> set specified values in specified columns in specified row.
//! 

mod private
{
  use clap::Subcommand;

  use crate::*;
  use actions::gspread::get_spreadsheet_id_from_url;

  #[ derive( Debug, Subcommand ) ]
  pub enum Commands
  {
    /// Command to set values range to a google sheet
    /// 
    /// Command example:
    /// 
    /// gspread cells set
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    /// --select-row-by-key "id"
    /// --json '{"id": "2", "A": "1", "B": "2"}'
    #[ command( name = "set" ) ]
    Set
    {
      #[ arg( long, help = "Identifier of a row. Available identifiers: id (row's unique identifier).\n\
      Example: --select_row_by_key \"id\"" ) ]
      select_row_by_key : String,
      
      #[ arg( long, help = "Value range. It must contain select_row_by_key.
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Every key and value must be a string.
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"id\": \"3\", \"A\": \"1\", \"B\": \"2\"}'\n\
      2. --json \"{\"id\": \"3\", \"A\": \"1\", \"B\": \"2\"}\"\n\
      3. --json '{\\\"id\\\": \\\"3\\\", \\\"A\\\": \\\"1\\\", \\\"B\\\": \\\"2\\\"}'\n\
      4. --json \"{\\\"id\\\": \\\"3\\\", \\\"A\\\": \\\"1\\\", \\\"B\\\": \\\"2\\\"}\" " ) ]
      json : String,

      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String
    }

  }

  pub async fn command
  (
    // hub : &SheetsType,
    commands : Commands
  )
  {
    match commands
    {
      Commands::Set { select_row_by_key, json, url, tab } =>
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
        
        match actions::gspread_cells_set::action
        (
          // &hub,
          select_row_by_key.as_str(),
          json.as_str(),
          spreadsheet_id,
          tab.as_str()
        )
        .await
        {
          Ok( val ) => println!( "{} cells were sucsessfully updated!", val ),
          Err( error ) => println!( "Error:\n{}", error )
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
    Commands
  };
}