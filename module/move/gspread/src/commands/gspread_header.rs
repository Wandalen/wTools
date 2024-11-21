//!
//! Command "header"
//!

mod private
{

  use crate::*;
  use commands::gspread::CommonArgs;
  use client::SheetsType;
  use actions;
  use actions::gspread::get_sheetspread_id_from_url;

  pub async fn command
  (
    hub : &SheetsType,
    args : CommonArgs,
  )
  {
    match args
    {
      CommonArgs { url, tab } =>
      {
        let sheetspread_id = get_sheetspread_id_from_url( url.as_str() ).unwrap();
        let result = actions::gspread_get_header::action
          (
            hub,
            sheetspread_id,
            tab.as_str()
          ).await;

        match result
        {
          Ok( header ) => println!( "Header: \n {}", header ),
          Err( error ) => println!( "Error: {}", error ),
        }
      }
    }
  }
}

crate::mod_interface!
{
  own use
  {
    command
  };
}

