//!
//! Tests for `get_row` function.
//! 

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::get_row;
use gcore::ApplicationSecret;
use gcore::client::
{
  Client, 
  ValueRange
};


/// # What 
/// We test retrieving a single row from a sheet by its key.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_row` function which sends a GET request to /{spreadsheet_id}/values/{sheet_name}!A{row_key}:ZZZ{row_key}
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_get_row_should_work()
{  
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let row_key = json!( 10 );

  let mock_response_values = vec![ vec![ json!( "Hello" ), json!( "World" ) ] ];

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A10:ZZZ10" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj
      ( 
        &ValueRange
        {
          range : Some( "tab2!A10:ZZZ10".to_string() ),
          major_dimension : None,
          values : Some( mock_response_values.clone() ),
        } 
      );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_row`.
  let row = get_row( &client, spreadsheet_id, sheet_name, row_key )
  .await
  .expect( "get_row failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( row.len(), 2 );
  assert_eq!( row[ 0 ], json!( "Hello" ) );
  assert_eq!( row[ 1 ], json!( "World" ) );
}


/// # What 
/// We test retrieving a row when no data exists for the given row key.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_row` function which sends a GET request to /{spreadsheet_id}/values/{sheet_name}!A999:ZZZ999
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_get_row_no_data_should_work()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let row_key = json!( 999 );
  let  response_body = ValueRange
  {
    range : Some( "tab2!A999:ZZZ999".to_string() ),
    ..Default::default()
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A999:ZZZ999" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_row`.
  let row = get_row( &client, spreadsheet_id, sheet_name, row_key )
  .await
  .expect( "get_row failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( row.len(), 0 );
}


/// # What
/// We test error handling if the server responds with an error.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_row` with a row key that triggers an error (e.g. row key out of range).
/// 4. We expect a panic (since the function returns an error and we `.expect()`).
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_get_row_with_error_should_panic()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let row_key = json!( "bad_key" );

  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!Abad_key:ZZZbad_key" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": { "message": "Invalid row key" } }"# );
  } );

  let endpoint = server.url( "" );
  let client: Client<'_, ApplicationSecret> = Client::former()
  .endpoint( &*endpoint )
  .form();

  let row = get_row( &client, spreadsheet_id, sheet_name, row_key )
  .await
  .expect( "get_row failed. Ok!" );

  println!( "{:?}", row );
}
