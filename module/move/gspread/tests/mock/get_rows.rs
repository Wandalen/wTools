//!
//! Tests for `get_rows` function.
//! 

use httpmock::prelude::*;

use serde_json::json;
use gspread::actions::gspread::get_rows;
use gspread::gcore::client::
{
  Client, 
  Dimension, 
  ValueRange
};

/// # What
/// We check that requesting all rows from the second row onward (below the header)
/// correctly parses the response and returns the expected result.
/// 
/// It works:
///  - With the whole rows.
///  - With rows with empty columns.
///  - With empty rows in the middle.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `get_rows` which sends a GET request to "/{spreadsheet_id}/values/{range}".
/// 4. Check results.
#[tokio::test]
async fn test_mock_get_rows_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2:ZZZ".to_string() ),
    values : Some
    ( 
      vec!
      [ 
        vec![ json!( "Row2Col1" ), json!( "Row2Col2" ) ], 
        vec![ json!( "Row3Col1" ), json!( "Row3Col2" ) ] 
      ] 
    )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2:ZZZ" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url("" );
  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_rows`
  let rows = get_rows( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_rows failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[0].len(), 2 );
  assert_eq!( rows[0][0], serde_json::Value::String( "Row2Col1".to_string() ) );
  assert_eq!( rows[0][1], serde_json::Value::String( "Row2Col2".to_string() ) );

  assert_eq!( rows[1].len(), 2);
  assert_eq!( rows[1][0], serde_json::Value::String( "Row3Col1".to_string() ) );
  assert_eq!( rows[1][1], serde_json::Value::String( "Row3Col2".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_rows_with_empty_columns() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2:ZZZ".to_string() ),
    values : Some
    ( 
      vec!
      [ 
        vec![ json!( "Row2Col1" ), json!( "" ), json!( "Row2Col3" ) ], 
        vec![ json!( "Row3Col1" ), json!( "" ), json!( "Row3Col3" ) ] 
      ] 
    )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2:ZZZ" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url("" );
  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_rows`
  let rows = get_rows( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_rows failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[0].len(), 3 );
  assert_eq!( rows[0][0], serde_json::Value::String( "Row2Col1".to_string() ) );
  assert_eq!( rows[0][1], serde_json::Value::String( "".to_string() ) );
  assert_eq!( rows[0][2], serde_json::Value::String( "Row2Col3".to_string() ) );

  assert_eq!( rows[1].len(), 3);
  assert_eq!( rows[1][0], serde_json::Value::String( "Row3Col1".to_string() ) );  
  assert_eq!( rows[1][1], serde_json::Value::String( "".to_string() ) );
  assert_eq!( rows[1][2], serde_json::Value::String( "Row3Col3".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_rows_with_empty_row_in_the_middle() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2:ZZZ".to_string() ),
    values : Some
    ( 
      vec!
      [ 
        vec![ json!( "Row2Col1" ), json!( "Row2Col2" ), json!( "Row2Col3" ) ], 
        vec![ json!( "" ), json!( "" ), json!( "" ) ],
        vec![ json!( "Row3Col1" ), json!( "Row3Col2" ), json!( "Row3Col3" ) ], 
      ] 
    )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2:ZZZ" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url("" );
  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `get_rows`
  let rows = get_rows( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_rows failed" );

  // 4. Check results.
  mock.assert();

  assert_eq!( rows.len(), 3 );
  assert_eq!( rows[0].len(), 3 );
  assert_eq!( rows[0][0], serde_json::Value::String( "Row2Col1".to_string() ) );
  assert_eq!( rows[0][1], serde_json::Value::String( "Row2Col2".to_string() ) );
  assert_eq!( rows[0][2], serde_json::Value::String( "Row2Col3".to_string() ) );

  assert_eq!( rows[1].len(), 3);
  assert_eq!( rows[1][0], serde_json::Value::String( "".to_string() ) );  
  assert_eq!( rows[1][1], serde_json::Value::String( "".to_string() ) );
  assert_eq!( rows[1][2], serde_json::Value::String( "".to_string() ) );

  assert_eq!( rows[2].len(), 3);
  assert_eq!( rows[2][0], serde_json::Value::String( "Row3Col1".to_string() ) );  
  assert_eq!( rows[2][1], serde_json::Value::String( "Row3Col2".to_string() ) );
  assert_eq!( rows[2][2], serde_json::Value::String( "Row3Col3".to_string() ) );
}

#[ tokio::test ]
async fn test_mock_get_rows_empty_should_work() 
{
  let spreadsheet_id = "12345";
  let body = ValueRange
  {
    major_dimension : Some( Dimension::Row ),
    range : Some( "tab2!A2:ZZZ".to_string() ),
    values : Some( vec![] )
  };

  // 1. Start a mock server.
  let server = MockServer::start();
  let _mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2:ZZZ" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body );
  } );

  // 2. Create a client.
  let endpoint = server.url("" );
  let client = Client::former()
  .endpoint( &*endpoint )
  .form();

  let rows = get_rows( &client, spreadsheet_id, "tab2" )
  .await
  .expect( "get_rows failed" );

  assert_eq!( rows.len(), 0 );
}