//!
//! Tests for `update_row` function.
//! 

use httpmock::prelude::*;

use serde_json::json;
use gspread::*;
use actions::gspread::update_row;
use gcore::ApplicationSecret;
use gcore::client::
{
  BatchUpdateValuesResponse, 
  Client, 
  Dimension, 
  ValueRange
};

/// # What
/// We check that updating a row in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `update_row()`, passing the necessary parameters.
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_update_row_should_work() 
{
  let spreadsheet_id = "12345";
  let value_ranges = vec!
  [
    ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( "tab2!A5".to_string() ),
      values : Some( vec![ vec![ json!( "Hello" ) ] ] )
    },

    ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( "tab2!A7".to_string() ),
      values : Some( vec![ vec![ json!( 123 ) ] ] )
    },
  ];

  let response_body = BatchUpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 2 ),
    total_updated_columns : Some( 1 ),
    total_updated_cells : Some( 2 ),
    total_updated_sheets : Some( 1 ),
    responses : Some( value_ranges )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `update_row` function.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_row
  ( 
    &client, 
    spreadsheet_id, 
    "tab2", 
    json!( "5" ), 
    row_key_val
  )
  .await
  .expect( "update_row failed in mock test" );

  // 4. Check results.
  mock.assert();

  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( batch_result.total_updated_cells, Some( 2 ) );
  assert_eq!( batch_result.total_updated_rows, Some( 2 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 1 ) );

  if let Some( responses ) = &batch_result.responses 
  {
    assert_eq!( responses.len(), 2 );
  }
}

#[ tokio::test ]
async fn test_mock_update_row_with_empty_values_should_work() 
{
  let spreadsheet_id = "12345";
  let response_body = BatchUpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : None,
    total_updated_columns : None,
    total_updated_cells : None,
    total_updated_sheets : None,
    responses : None
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `update_row` function.
  let row_key_val = std::collections::HashMap::new();

  let batch_result = update_row
  ( 
    &client, 
    spreadsheet_id, 
    "tab2", 
    json!( "5" ), 
    row_key_val
  )
  .await
  .expect( "update_row failed in mock test" );

  // 4. Check results.
  mock.assert();

  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( spreadsheet_id ) );
  assert_eq!( batch_result.total_updated_cells, None );
  assert_eq!( batch_result.total_updated_rows, None );
  assert_eq!( batch_result.total_updated_columns, None );
}

/// # What
/// We test that function will return an error if invalid paramentrs were passed.
/// 
/// # How 
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `update_row` which sends a POST request to /{spreadsheet_id}/values:batchUpdate
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_update_row_with_invalid_row_key_should_panic() 
{
  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( "{ error: invalid row_key }" );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `update_row` function.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_row
  ( 
    &client, 
    "12345", 
    "tab2", 
    json!( "Invalid row_key" ), 
    row_key_val
  )
  .await
  .expect( "update_row failed in mock test. Ok!" );
}

#[ tokio::test ]
#[ should_panic ]
async fn test_mock_update_row_with_invalid_row_key_val_should_panic() 
{
  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( "{ error: invalid column index }" );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `update_row` function.
  let mut row_key_val = std::collections::HashMap::new();
  // It is invalid. Allowed range: A -> ZZZ
  row_key_val.insert( "AAAAAA".to_string(), json!( "Hello" ) );
  // It is also ionvalid
  row_key_val.insert( "12".to_string(), json!( 123 ) );

  let _batch_result = update_row
  ( 
    &client, 
    "12345", 
    "tab2", 
    json!( "Invalid row_key" ), 
    row_key_val
  )
  .await
  .expect( "update_row failed in mock test. Ok!" );
}