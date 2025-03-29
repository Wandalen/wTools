//!
//! Tests for `append_row` function.
//! 

use gspread::gcore::client::BatchUpdateValuesResponse;
use httpmock::prelude::*;
use serde_json::json;
use std::collections::HashMap;

use gspread::*;
use actions::gspread::append_row;
use gcore::ApplicationSecret;
use gcore::client::Client;


/// # What 
/// We test appending a row at the and of a sheet.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `append_row` function wich sends a POST request to /{spreadshett_id}/values/{range}:append
/// 4. Check results.
#[tokio::test]
async fn test_mock_append_row_should_work() 
{
  let spreadsheet_id = "12345";
  let body_batch_update = BatchUpdateValuesResponse 
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 1 ),
    total_updated_columns : Some( 3 ),
    total_updated_cells : Some( 3 ),
    total_updated_sheets : Some( 1 ),
    responses : None,
  };

  let body_values_append = json!({
    "updates": {
      "updatedRange": "tab2!A5"
    }
  });

  let server = MockServer::start();

  let mock_append = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab2!A1:append" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body( body_values_append.clone() );
  } );

  let mock_batch_update = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body_batch_update );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  let mut row_key_val = HashMap::new();
  row_key_val.insert( "A".to_string(), json!( 1 ) );
  row_key_val.insert( "B".to_string(), json!( 2 ) );
  row_key_val.insert( "C".to_string(), json!( 3 ) );

  let response = append_row( &client, spreadsheet_id, "tab2", &row_key_val )
  .await
  .expect( "append_row failed." );

  mock_append.assert();
  mock_batch_update.assert();

  assert_eq!( response.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( response.total_updated_cells, Some( 3 ) );
}

#[ allow( non_snake_case ) ]
#[ tokio::test ]
async fn test_mock_append_row_begining_from_C_column_should_work()
{  
  let spreadsheet_id = "12345";
  let body_batch_update = BatchUpdateValuesResponse 
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 1 ),
    total_updated_columns : Some( 7 ),
    total_updated_cells : Some( 7 ),
    total_updated_sheets : Some( 1 ),
    responses : None,
  };
  let body_values_append = json!({
    "updates": {
      "updatedRange": "tab2!A5"
    }
  });

  // 1. Start a mock server.
  let server = MockServer::start();
  
  let mock_append = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab2!A1:append" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body( body_values_append.clone() );
  } );

  let mock_batch_update = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body_batch_update );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `append_row`.
  let mut row_key_val = HashMap::new();
  row_key_val.insert( "C".to_string(), json!( 1 ) );
  row_key_val.insert( "D".to_string(), json!( 2 ) );
  row_key_val.insert( "F".to_string(), json!( 3 ) );
  row_key_val.insert( "G".to_string(), json!( 4 ) );

  let response = append_row( &client, spreadsheet_id, "tab2", &row_key_val )
  .await
  .expect( "append_row failed." );

  // 4. Check results.
  mock_append.assert();
  mock_batch_update.assert();

  assert_eq!( response.spreadsheet_id, Some( spreadsheet_id.to_string() ) );
  assert_eq!( response.total_updated_cells, Some( 7 ) );
}


/// # What
/// We test that we can not pass a HashMap with invalid column index.
/// 
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `append_row` wich sends a POST request to /{spreadsheet_id}/values/{range}:append
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_append_row_with_bad_values_should_panic()
{  
  let spreadsheet_id = "12345";
  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab2!A1:append" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": "column index is invalid" }"# );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `append_row`.
  let mut row_key_val = HashMap::new();
  row_key_val.insert( "AAAAA".to_string(), json!( 1 ) );
  row_key_val.insert( "BBBBA".to_string(), json!( 2 ) );
  row_key_val.insert( "CCCCA".to_string(), json!( 3 ) );

  let _response = append_row( &client, spreadsheet_id, "tab2", &row_key_val )
  .await
  .expect( "append_row failed. Ok!" );
}

#[ tokio::test ]
#[ should_panic ]
async fn test_mock_append_row_with_bad_values2_should_panic()
{  
  let spreadsheet_id = "12345";
  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab2!A1:append" );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( r#"{ "error": "column name is invalid" }"# );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `append_row`.
  let mut row_key_val = HashMap::new();
  row_key_val.insert( "123".to_string(), json!( 1 ) );
  row_key_val.insert( "a".to_string(), json!( 2 ) );
  row_key_val.insert( "qdqwq".to_string(), json!( 3 ) );

  let _response = append_row( &client, spreadsheet_id, "tab2", &row_key_val )
  .await
  .expect( "append_row failed. Ok!" );
}