


mod private
{
  use crate::*;
  use gcore::Secret;
  use gcore::error::Result;
  use gcore::client::Client;
  use actions::gspread::update_rows_by_custom_row_key;
  use actions::utils::
  {
    parse_json, 
    parse_key_by, 
    parse_on_fail, 
    parse_on_find
  };


  pub async fn action< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : &str,
    json_str : &str,
    on_find : &str,
    on_fail : &str
  ) -> Result< u32 >
  {
    let key_by = match parse_key_by( key_by )
    {
      Ok( val ) => val,
      Err( error ) => return Err( error ),
    };

    let on_find = parse_on_find( on_find )?;
    let on_fail = parse_on_fail( on_fail )?;

    match parse_json( json_str )
    {
      Ok( parsed_json ) =>
      {
        match update_rows_by_custom_row_key
        ( 
          client, 
          spreadsheet_id, 
          sheet_name, 
          key_by, 
          parsed_json, 
          on_find, 
          on_fail 
        ).await
        {
          Ok( response ) => Ok
          ( 
            match response.responses
            {
              Some( _ ) => match response.total_updated_cells
              {
                Some( amount ) => amount,
                None => 0
              },
              None => 0,
            } 
          ),
          Err( error ) => Err( error )
        }
      },

      Err( error ) => Err( error )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    action
  };
}