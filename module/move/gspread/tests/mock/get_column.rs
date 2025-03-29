//!
//! Tests for `get_column` function.
//! 

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::get_column;
use gcore::ApplicationSecret;
use gcore::client::
{
  Client, 
  Dimension, 
  ValueRange, 
};

/// # What 
/// We test retrieving a single column from a sheet by its column ID.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_column` function which sends a GET request to /{spreadsheet_id}/values/{sheet_name}!{column_id}:{column_id}
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_get_column_should_work()
{  
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "A";

  let mock_response_values = vec![ vec![ json!( "Value1" ), json!( "Value2" ), json!( "Value3" ) ] ];

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A:A" )
      .query_param( "majorDimension", "COLUMNS" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj
      ( 
        &ValueRange
        {
          range : Some( "tab2!A:A".to_string() ),
          major_dimension : Some( Dimension::Column ),
          values : Some( mock_response_values.clone() ),
        } 
      );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_column`.
  let column = get_column( &client, spreadsheet_id, sheet_name, column_id )
  .await
  .expect( "get_column failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( column.len(), 3 );
  assert_eq!( column[0], json!( "Value1" ) );
  assert_eq!( column[1], json!( "Value2" ) );
  assert_eq!( column[2], json!( "Value3" ) );
}


/// # What 
/// We test retrieving a column when no data exists for the given column ID.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_column` function which sends a GET request to /{spreadsheet_id}/values/{sheet_name}!{column_id}:{column_id}
/// 4. Check results (an empty array is returned).
#[ tokio::test ]
async fn test_mock_get_empty_column_should_work()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "Z";
  let  response_body = ValueRange
  {
    range : Some( "tab2!Z:Z".to_string() ),
    major_dimension : Some( Dimension::Column ),
    ..Default::default()
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!Z:Z" )
      .query_param( "majorDimension", "COLUMNS" )
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

  // 3. Call `get_column`.
  let column = get_column( &client, spreadsheet_id, sheet_name, column_id )
  .await
  .expect( "get_column failed." );

  // 4. Check results.
  mock.assert();

  assert_eq!( column.len(), 0 );
}


/// # What
/// We test error handling if the server responds with an error.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_column` with a column ID that triggers an error.
/// 4. We expect a panic (since the function returns an error and we `.expect()`).
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_get_column_with_error_should_panic()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "INVALID";

  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!INVALID:INVALID" )
      .query_param( "majorDimension", "COLUMNS" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": { "message": "Invalid column ID" } }"# );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_column`.
  let column = get_column( &client, spreadsheet_id, sheet_name, column_id )
  .await
  .expect( "get_column failed. Ok!" );

  println!( "{:?}", column );
}
