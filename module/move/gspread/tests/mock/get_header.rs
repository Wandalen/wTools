//!
//! Tests for `get_header()` function.
//! It can return only one of the common errors.
//! 

use gspread::gcore::ApplicationSecret;
use httpmock::prelude::*;

use serde_json::json;
use gspread::actions::gspread::get_header;
use gspread::gcore::client::
{
  Client, 
  Dimension, 
  ValueRange
};

/// # What
/// We check that requesting the header row (first row) of a sheet in a Google Spreadsheet
/// returns the correct set of column values.
/// 
/// It works:
///  - With the whole header, 
///  - With empty columns between columns,
///  - With empty column at the start.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_header()` function wich sends a GET request to /{spreadshett_id}/values/{range}.
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_get_header_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A1:ZZZ1".to_string() ),
    values : Some( vec![ vec![ json!( "ID" ), json!( "Name" ), json!( "Email" ) ] ] )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A1:ZZZ1" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client. 
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a GET request 
  let header = get_header( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_header failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( header.len(), 3, "Header row should have 3 columns" );

  assert_eq!( header[0], serde_json::Value::String( "ID".to_string() ) );
  assert_eq!( header[1], serde_json::Value::String( "Name".to_string() ) );
  assert_eq!( header[2], serde_json::Value::String( "Email".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_header_with_empty_column_betwee_columns_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A1:ZZZ1".to_string() ),
    values : Some( vec![ vec![ json!( "ID" ), json!( "" ), json!( "Email" ) ] ] )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A1:ZZZ1" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client. 
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a GET request 
  let header = get_header( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_header failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( header.len(), 3, "Header row should have 3 columns" );

  assert_eq!( header[0], serde_json::Value::String( "ID".to_string() ) );
  assert_eq!( header[1], serde_json::Value::String( "".to_string() ) );
  assert_eq!( header[2], serde_json::Value::String( "Email".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_header_with_empty_first_column_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A1:ZZZ1".to_string() ),
    values : Some( vec![ vec![ json!( "" ), json!( "Name" ), json!( "Email" ) ] ] )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A1:ZZZ1" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client. 
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a GET request 
  let header = get_header( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_header failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( header.len(), 3, "Header row should have 3 columns" );

  assert_eq!( header[0], serde_json::Value::String( "".to_string() ) );
  assert_eq!( header[1], serde_json::Value::String( "Name".to_string() ) );
  assert_eq!( header[2], serde_json::Value::String( "Email".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_header_with_empty_column_columns_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A1:ZZZ1".to_string() ),
    values : Some( vec![ vec![] ] )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A1:ZZZ1" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client. 
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send a GET request 
  let header = get_header( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_header failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( header.len(), 0, "Header row should have 0 columns" );
}