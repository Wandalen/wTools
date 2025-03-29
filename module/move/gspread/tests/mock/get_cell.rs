//!
//! Tests for `get_cell` function.
//! 

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::get_cell;
use gcore::ApplicationSecret;
use gcore::client::
{
  Client, 
  Dimension, 
  ValueRange
};

/// # What
/// We check that reading a specific cell from a Google Spreadsheet returns the expected result.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Send a GET request to "/{spreadsheet_id}/values/{range}".
/// 4. Check for correct results.
#[ tokio::test ]
async fn test_mock_get_cell_should_work() 
{
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2".to_string() ),
    values : Some( vec![ vec![ json!( "Steeve" ) ] ] )
  };

  // 1. Ceating a server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Creating a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Sending a PUT request.
  let result = get_cell( &client, "12345", "tab2", "A2" )
  .await
  .expect( "get_cell failed" );

  // 4. Checking results.
  mock.assert();

  assert_eq!( result, serde_json::Value::String( "Steeve".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_empty_cell_should_work() 
{
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2".to_string() ),
    values : Some( vec![ vec![ json!( "" ) ] ] )
  };

  // 1. Ceating a server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Creating a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Sending a PUT request.
  let result = get_cell( &client, "12345", "tab2", "A2" )
  .await
  .expect( "get_cell failed" );

  // 4. Checking results.
  mock.assert();

  assert_eq!( result, serde_json::Value::String( "".to_string() ) );
}

/// # What
/// We test that function has to return an error if invalid cellid was provideed.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Call `get_cell` and pass there a bad cell id. 
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_get_cell_with_bad_range_should_panic() 
{
  // 1. Ceating a server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!AAAA2" );
    then
      .status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ error: invalid range. }"# );
  } );

  // 2. Creating a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Sending a PUT request.
  let _result = get_cell( &client, "12345", "tab2", "AAAA2" )
  .await
  .expect( "get_cell failed" );
}