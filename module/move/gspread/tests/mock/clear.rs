//!
//! Tests for `clear` function.
//!

use httpmock::prelude::*;
use gspread::*;
use actions::gspread::clear;
use gcore::ApplicationSecret;
use gcore::client::
{
  Client,
  ValuesClearResponse
};


/// # What 
/// We test clearing a sheet by specifying its name.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `clear` function which sends a POST request to /{spreadsheet_id}/values/{sheet_name}!A:ZZZ:clear
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_clear_should_work()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";

  let body = ValuesClearResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    cleared_range : Some( "tab2!A:ZZZ".to_string() )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then |
  {
    when.method( POST )
      .path( "/12345/values/tab2!A:ZZZ:clear" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `clear`.
  let response = clear( &client, spreadsheet_id, sheet_name )
  .await
  .expect( "clear failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( response.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( response.cleared_range, Some( "tab2!A:ZZZ".to_string() ) );
}


/// # What
/// We test clearing a sheet when there is no data to clear.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `clear` which sends a POST request to /{spreadsheet_id}/values/{sheet_name}!A:ZZZ:clear
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_clear_empty_result_should_work()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let body = ValuesClearResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    cleared_range : None
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then |
  {
    when.method( POST )
      .path( "/12345/values/tab2!A:ZZZ:clear" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `clear`.
  let response = clear( &client, spreadsheet_id, sheet_name )
  .await
  .expect( "clear failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( response.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( response.cleared_range, None );
}


/// # What
/// We test error handling if the server responds with an error.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `clear` with invalid parameters or server error.
/// 4. We expect a panic.
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_clear_with_error_should_panic()
{
  let spreadsheet_id = "12345";
  let sheet_name = "invalid_sheet";

  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then |
  {
    when.method( POST )
      .path( "/12345/values/invalid_sheet!A:ZZZ:clear" );
    then.status( 404 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": { "message": "Sheet not found" } }"# );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
    .endpoint( &*endpoint )
    .form();

  // 3. Call `clear`.
  let response = clear( &client, spreadsheet_id, sheet_name )
  .await
  .expect( "clear failed. Ok!" );

  println!( "{:?}", response );
}
