//!
//! Collection of Google Sheets API commands.
//!


mod private
{

  use clap::{ Subcommand, Parser };

  use crate::*;
  use client::SheetsType;

  use commands::
  {
    gspread_header,
    gspread_rows,
    gspread_cell,
    gspread_cells
  };

  #[ derive( Debug, Parser ) ]
  pub struct CommonArgs
  {
    #[ arg( long, help = "Full URL of Google Sheet.\n\
    It has to be inside of '' to avoid parse errors.\n\
    Example: 'https://docs.google.com/spreadsheets/d/your_spreadsheet_id/edit?gid=0#gid=0'" ) ]
    pub url : String,

    #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
    pub tab : String
  }

  #[ derive( Debug, Subcommand ) ]
  pub enum Command
  {
    
    /// Command to get header of a sheet. Header is a first raw.
    /// 
    /// Command example: 
    /// 
    /// gspread header
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    #[ command ( name = "header" ) ]
    Header
    (
      CommonArgs
    ),

    /// Command to get all raws of a sheet but not header.
    /// 
    /// Command example:
    /// 
    /// gspread rows
    /// --url 'https://docs.google.com/spreadsheets/d/1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU/edit?gid=0#gid=0'
    /// --tab tab1
    #[ command( name = "rows" ) ]
    Rows
    (
      CommonArgs
    ),

    /// Command to get or update a cell from a sheet.
    #[ command ( subcommand, name = "cell" ) ]
    Cell
    (
      gspread_cell::Commands
    ),

    /// Commands to set a new value to a cell or get a value from a cell.
    #[ command ( subcommand, name = "cells" ) ]
    Cells
    (
      gspread_cells::Commands
    )

  }

  pub async fn command
  (
    hub : &SheetsType,
    command : Command,
  )
  {
    match command
    {

      Command::Header( header_command ) =>
      {
        gspread_header::command( hub, header_command ).await;
      },

      Command::Rows( rows_command ) =>
      {
        gspread_rows::command( hub, rows_command ).await;
      },

      Command::Cell( cell_command ) =>
      {
        gspread_cell::command( hub, cell_command ).await;
      },

      Command::Cells( cells_command) =>
      {
        // hub
        gspread_cells::command( cells_command ).await;
      },

    }
  }

}

crate::mod_interface!
{
  own use
  {
    CommonArgs,
    Command,
    command,
  };
}