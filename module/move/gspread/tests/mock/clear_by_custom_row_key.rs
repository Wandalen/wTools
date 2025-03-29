//!
//! Tests for `clear_by_custom_row_key` function.
//!

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::
{
  clear_by_custom_row_key, 
  OnFind
};
use gcore::ApplicationSecret;
use gcore::client::
{
  BatchClearValuesResponse, 
  Client, 
  Dimension, 
  ValueRange 
};


/// # What 
/// We test clearing matched rows by a custom key in a specific column.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Mock the first request to get the column (GET).
/// 3. Mock the second request to batch clear matched rows (POST).
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_clear_by_custom_row_key_should_work()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "A";
  let on_find = OnFind::FirstMatchedRow;
  let key_value = json!( "B" );
  let column_values = vec![ vec![ json!( "A" ), json!( "B" ), json!( "C" ) ] ];
  let response_body = ValueRange
  {
    range : Some( "tab2!A:A".to_string() ),
    major_dimension : Some( Dimension::Column ),
    values : Some( column_values.clone() ),
  };

  // 1. Start a mock server.
  let server = MockServer::start();

  // 2. Mock the GET request for the column.
  let get_mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A:A" )
      .query_param( "majorDimension", "COLUMNS" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 3. Mock the POST request to batch clear.
  let response_body = BatchClearValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    cleared_ranges : Some( vec![ "tab2!A2:ZZZ2".to_string() ] )
  };

  let post_mock = server.mock( | when, then |
  {
    when.method( POST )
      .path( "/12345/values:batchClear" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 4. Call `clear_by_custom_row_key`.
  let result = clear_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    sheet_name,
    ( column_id, key_value ),
    on_find
  )
  .await
  .expect( "clear_by_custom_row_key failed." );

  get_mock.assert();
  post_mock.assert();

  assert_eq!( result.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( result.cleared_ranges, Some( vec![ "tab2!A2:ZZZ2".to_string() ] ) );
}


/// # What
/// We test clearing rows when column is empty or no rows match. 
/// 
/// # How
/// 1. Start a mock server.
/// 2. Mock the GET request that returns no values in the column.
/// 3. Check that the function returns a default response without calling batch clear.
#[ tokio::test ]
async fn test_mock_clear_by_custom_row_key_no_matches_should_return_default()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "A";
  let on_find = OnFind::FirstMatchedRow; 
  let key_value = json!( "whatever" );
  let response_body = ValueRange
  {
    range : Some( String::from( "tab2!A:A" ) ),
    major_dimension : Some( Dimension::Column ),
    values : None
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  // 2. Mock the GET request - returning no column data.
  let get_mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A:A" )
      .query_param( "majorDimension", "COLUMNS" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // Call `clear_by_custom_row_key`.
  let result = clear_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    sheet_name,
    ( column_id, key_value ),
    on_find
  )
  .await
  .expect( "clear_by_custom_row_key failed." );

  get_mock.assert();

  assert_eq!( result.spreadsheet_id, None );
  assert_eq!( result.cleared_ranges, None );
}


/// # What
/// We test error handling when the first request (get_column) fails.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Mock the GET request with an error (e.g., 400).
/// 3. We expect the function to return an error (here we `.expect()` => panic).
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_clear_by_custom_row_key_error_in_get_column_should_panic()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "1234";
  let on_find = OnFind::FirstMatchedRow; 
  let key_value = json!( "B" );

  let server = MockServer::start();
  let _get_mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A:A" )
      .query_param( "majorDimension", "COLUMNS" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": "Invalid column ID" }"# );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // This call should fail and panic because we `.expect(...)`.
  let result = clear_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    sheet_name,
    ( column_id, key_value ),
    on_find
  )
  .await
  .expect( "clear_by_custom_row_key failed. Ok!" );

  println!( "{:?}", result );
}


/// # What
/// We test error handling when batch clear fails.
/// 
/// 1. The function successfully retrieves column data.
/// 2. The function attempts to clear batch, but that request fails.
/// 3. The function should bubble up the error (here we `.expect()` => panic).
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_clear_by_custom_row_key_error_in_batch_clear_should_panic()
{
  let spreadsheet_id = "12345";
  let sheet_name = "tab2";
  let column_id = "A";
  let on_find = OnFind::FirstMatchedRow; 
  let key_value = json!( "B" );
  let column_values = vec![ vec![ json!( "A" ), json!( "B" ), json!( "C" ) ] ]; 
  let response_body = ValueRange
  {
    range : Some( "tab2!A:A".to_string() ),
    major_dimension : Some( Dimension::Column ),
    values : Some( column_values.clone() ),
  };

  let server = MockServer::start();
  let _get_mock = server.mock( | when, then |
  {
    when.method( GET )
      .path( "/12345/values/tab2!A:A" )
      .query_param( "majorDimension", "COLUMNS" )
      .query_param( "valueRenderOption", "UNFORMATTED_VALUE" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // Mock POST for batchClear - will fail.
  let _post_mock = server.mock( | when, then |
  {
    when.method( POST )
      .path( "/12345/values:batchClear" );
    then.status( 500 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": { "message": "Internal Server Error" } }"# );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // This call should fail and panic because the second request returns 500.
  let result = clear_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    sheet_name,
    ( column_id, key_value ),
    on_find
  )
  .await
  .expect( "clear_by_custom_row_key failed. Ok!" );

  println!( "{:?}", result );
}
