//!
//! Command "rows"
//!

mod private
{

  use crate::*;
  use commands::gspread::CommonArgs;
  use client::SheetsType;
  use actions;
  use actions::gspread::get_sheetspread_id_from_url;
  use debug::table_wrapper::Table;

  pub async fn command
  (
    hub: &SheetsType,
    args: CommonArgs
  )
  {
    match args
    {
      CommonArgs { url, tab } =>
      {
        let sheetspread_id = get_sheetspread_id_from_url(url.as_str()).unwrap();

        let result = actions::gspread_get_rows::action
        (
          hub,
          sheetspread_id,
          tab.as_str()
        ).await;

        match result
        {
          Ok( ValueRange ) =>
          {
            let table = Table::new( ValueRange );
            table.display();
          },
          Err( error ) => println!( "Error: {}", error ),
        }
      }
    }
  }
}

pub use private::
{
  command,
};