//!
//! Google Sheets API actions.
//!
//! This module also contains the definition of Google Sheets Error.
//!

mod private
{
  use regex::Regex;
  use error_tools::typed::Error;
  use derive_tools::AsRefStr;

  use crate::*;
  use ser::
  {
    JsonValue,
    DisplayFromStr
  };

  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    #[ error( "Google Sheets returned error:\n{0}" ) ]
    ApiError
    (
      #[ from ]
      #[ serde_as( as = "DisplayFromStr" ) ]
      google_sheets4::Error
    )
  }

  pub fn get_sheetspread_id_from_url
  (
    url: &str
  ) -> Option< &str >
  {

    let re = Regex::new( r"d/([^/]+)/edit" ).unwrap();
    if let Some( captures ) = re.captures( url )
    {
      if let Some( id ) = captures.get( 1 )
      {
        return Some( id.as_str() );
      }
    }

    None
  }

  pub type Result< T > = core::result::Result< T, Error >;
  pub type Value = Vec< Vec< JsonValue > >;
}

pub use private::
{
  Result,
  Value,
  get_sheetspread_id_from_url
};