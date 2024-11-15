//!
//! Action for command "cell get"
//!
//! It returns a selected cell
//!

mod private
{
  use crate::*;
  use actions::gspread::{ Value, Result };
  use client::SheetsType;

  pub async fn action
  (
    hub: &SheetsType,
    spreadsheet_id: &str,
    table_name: &str,
    cell_id: &str,
  ) -> Result< Value >
  {
    let result = hub
      .spreadsheets()
      .values_get(spreadsheet_id, format!("{}!{}", table_name, cell_id).as_str())
      .doit()
      .await?;

    Ok( result.1.values.unwrap() )
  }
}

pub use private::
{
  action,
};
