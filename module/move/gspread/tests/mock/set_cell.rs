//!
//! Tests for `set_cell` function.
//! 

use gspread::gcore::ApplicationSecret;
use httpmock::prelude::*;

use serde_json::json;
use gspread::actions::gspread::set_cell; 
use gspread::gcore::client::
{
  Client, 
  Dimension, 
  ValueRange,
  UpdateValuesResponse 
};

/// # What
/// We check that setting a value in a specific cell of a Google Spreadsheet works correctly.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Send a PUT request to /{spreadsheet_id}/values/{range}?valueInputOption=RAW.
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_set_cell_should_work() 
{
  // 1. Start a mock server.
  let spreadsheet_id = "12345";
  let range = "tab2!A1";
  let value_range = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( range.to_string() ),
    values : Some( vec![ vec![ json!( "Val" ) ] ] )
  };

  let response_body = UpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    updated_cells : Some( 1 ),
    updated_columns : Some( 1 ),
    updated_range : Some( range.to_string() ),
    updated_rows : Some( 1 ),
    updated_data : Some( value_range )
  };

  let server = MockServer::start();

  let mock = server.mock( | when, then | {
    when.method( PUT )
      .path( "/12345/values/tab2!A1" )
      .query_param( "valueInputOption", "RAW" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );

  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a PUT request.
  let result = set_cell
  ( 
    &client, 
    spreadsheet_id, 
    "tab2", 
    "A1", 
    json!( "Val" ) 
  )
  .await
  .expect( "set_cell failed with mock" );

  // 4. Check results.
  mock.assert();

  assert_eq!( result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( result.updated_range.as_deref(), Some( range ) );
  assert_eq!( result.updated_rows, Some( 1 ) );
  assert_eq!( result.updated_columns, Some( 1 ) );
  assert_eq!( result.updated_cells, Some( 1 ) );

  if let Some( updated_data ) = &result.updated_data 
  {
    let values = updated_data.values.as_ref().unwrap();
    assert_eq!( values, &vec![ vec![ json!( "Val" ) ] ] );
  }
}

/// # What
/// We test that `set_cell` function will return error with bad cell_id.
///
/// # How
/// 1. Start a mock server.
/// 2. Send a PUT request to /{spreadsheet_id}/values/{range}?valueInputOption=RAW.
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_set_cell_bad_cell_id_should_panic() 
{
  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( PUT )
      .path( "/12345/values/tab2!AAAA1" )
      .query_param( "valueInputOption", "RAW" );
    then
      .status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ error: invalid range. }"# );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a PUT request.
  let _result = set_cell( &client, "12345", "tab2", "A1", json!( "Val" ) )
  .await
  .expect( "set_cell failed with mock. Ok." );
}