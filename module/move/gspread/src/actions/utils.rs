
mod private
{
  use regex::Regex;
  use std::collections::HashMap;

  use crate::*;
  use gcore::error::
  { 
    Error, Result 
  };
  use actions::gspread::
  { 
    OnFail, 
    OnFind 
  };

  /// # parse_key_by
  /// 
  /// Parse a provided string to ( &str, serde_json::Value )
  /// 
  /// ## Errors
  /// 
  /// Can occur if passed string is not valid.
  pub fn parse_key_by( s : &str ) -> Result< ( &str, serde_json::Value ) >
  {
    let result : ( &str, serde_json::Value ) = serde_json::from_str( s )
    .map_err( | err | Error::ParseError( format!( "Failed to parse key_by. {}", err ) ) )?;
    
    Ok( result )
  }

  /// # parse_on_find
  /// 
  /// Parse provided string to OnFind's variant.
  /// 
  /// ## Errors
  /// 
  /// Can occur if variant is not allowed.
  pub fn parse_on_find( on_find : &str ) -> Result< OnFind >
  {
    check_variant( on_find, vec![ "first", "last", "all" ] )?;
    match on_find
    {
      "first" => Ok( OnFind::FirstMatchedRow ),
      "last" => Ok( OnFind::LastMatchedRow ),
      "all" => Ok( OnFind::AllMatchedRow ),
      &_ => Err( Error::ParseError( format!( "OnFind prase error." ) ) )
    }
  }

  /// # parse_on_fail
  /// 
  /// Parse provided string to OnFail's variant.
  /// 
  /// ## Errors
  /// 
  /// Can occur if variant is not allowed.
  pub fn parse_on_fail( on_fail : &str ) -> Result< OnFail >
  {
    check_variant( on_fail, vec![ "none", "error", "append" ] )?;
    match on_fail
    {
      "none" => Ok( OnFail::Nothing ),
      "error" => Ok( OnFail::Error ),
      "append" => Ok( OnFail::AppendRow ),
      &_ => Err( Error::ParseError( format!( "OnFail parse error." ) ) )
    }
  }

  /// # check_variant
  /// 
  /// Checks if passed variant is correct.
  /// 
  /// ## Returns:
  ///  - `Result< () >`
  /// 
  /// ## Errors:
  /// 
  /// Can occur if passed varaint is not alllowed.
  pub fn check_variant
  ( 
    variant : &str,
    allowed : Vec< &str > 
  ) -> Result< () >
  {
    if allowed.contains( &variant )
    {
      Ok( () )
    }
    else
    {
      Err
      ( 
        Error::ParseError( format!( "Not suchvariant: {}. Allowed: {:?}", variant, allowed ) )
      )
    }
  }

  /// # parse_json
  /// 
  /// Parse passed json to HashMap< String, serde_json::Value >
  /// 
  /// ## Returns
  ///  - `Result< HashMap< String, serde_json::Value > >`
  /// 
  /// ## Errors
  /// 
  /// Can occur if the passed json is not valid.
  pub fn parse_json
  ( 
    json_str : &str 
  ) -> Result< HashMap< String, serde_json::Value > >
  {
    let parsed_json : HashMap< String, serde_json::Value > = serde_json::from_str( json_str )
    .map_err( | error | Error::InvalidJSON( format!( "Failed to parse JSON: {}", error ) ) )?;

    Ok( parsed_json )
  }
  
  /// # `get_spreadsheet_id_from_url`
  ///
  /// Retrieves the spreadsheet ID from the provided Google Sheets URL.
  ///
  /// ## Parameters:
  /// - `url`:  
  ///   A `&str` containing the full URL of the Google spreadsheet.  
  ///
  /// ## Returns:
  /// - `Result< &str >`
  ///
  /// ## Errors:
  /// - `Error::InvalidUrl`:  
  ///   Occurs when the URL does not match the expected format.  
  ///   Suggests copying the entire URL directly from the browser.
  pub fn get_spreadsheet_id_from_url
  (
    url : &str
  ) -> Result< &str >
  {

    let re = Regex::new( r"d/([^/]+)/edit" ).unwrap();
    if let Some( captures ) = re.captures( url )
    {
      if let Some( id ) = captures.get( 1 )
      {
        return Ok( id.as_str() );
      }
    }

    Err
    ( 
      Error::InvalidUrl( "Wrong url format.\nFix: copy sheet's the whole url from your browser. Usage: --url '<your copied url>'".to_string() ) 
    )
  }
}

crate::mod_interface!
{
  own use
  {
    parse_json,
    parse_key_by,
    parse_on_find,
    parse_on_fail,
    check_variant,
    get_spreadsheet_id_from_url
  };
}