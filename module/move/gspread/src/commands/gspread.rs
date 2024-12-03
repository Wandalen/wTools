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
    #[ arg( long ) ]
    pub url : String,

    #[ arg( long ) ]
    pub tab : String
  }

  #[ derive( Debug, Subcommand ) ]
  pub enum Command
  {

    #[ command ( name = "header" ) ]
    Header
    (
      CommonArgs
    ),

    #[ command( name = "rows" ) ]
    Rows
    (
      CommonArgs
    ),

    #[ command ( subcommand, name = "cell" ) ]
    Cell
    (
      gspread_cell::Commands
    ),

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
        gspread_cells::command( hub, cells_command ).await;
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