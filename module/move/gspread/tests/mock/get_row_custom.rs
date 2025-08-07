//!
//! Tests for `get_row_by_custom_row_key`.
//!.

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::
{
  get_row_by_custom_row_key, 
  OnFind
};
use gcore::
{
  client::Client, 
  ApplicationSecret
};

/// # What
/// This test checks that `get_row_by_custom_row_key` returns an empty vector
/// when the specified key value does not exist in the given column.  
///
/// # How
/// 1. Start a mock server.
/// 2. Create a `Client` pointing to that mock server.
/// 3. Mock a `GET` request to return no matching values in the desired column.
/// 4. Mock the `values:batchGet` request but expect it to be called **0 times**.
/// 5. Call `get_row_by_custom_row_key`.
/// 6. Assert that an empty `Vec` is returned, and `batchGet` was never triggered.
#[ tokio::test ]
async fn test_mock_get_row_by_custom_row_key_no_matches() 
{
  let server = MockServer::start();
  let spreadsheet_id = "12345";

  let get_mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab1!E:E" );
    then.status( 200 )
      .header( "Content-Type", "application/json")
      .json_body
      ( 
        json!
        ( 
          {
            "range" : "tab1!E:E",
            "majorDimension" : "COLUMNS",
            "values" : [ [ "111", "111", "111" ] ]
          } 
        ) 
      );
  } );

  let batch_get_mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values:batchGet" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body
      ( 
        json!
        ( 
          {
            "spreadsheetId" : "12345",
            "valueRanges" : []
          } 
        ) 
      );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  let fetched_rows = get_row_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( "targetVal" ) ),
    OnFind::AllMatchedRow,
  )
  .await
  .expect( "get_row_by_custom_row_key failed" );

  assert!( fetched_rows.is_empty(), "Expected no matched rows." );
  
  get_mock.assert();
  batch_get_mock.assert();
}


/// # What
/// This test checks `get_row_by_custom_row_key` when multiple rows match the key,
/// but we only want the **last** matched row (`OnFind::LastMatchedRow`).
///
/// # How
/// 1. Start a mock server.
/// 2. Create a `Client`.
/// 3. Mock the GET request, simulating multiple matches.
/// 4. Mock the batchGet request for the last matching row (say row 5).
/// 5. Call `get_row_by_custom_row_key` with `OnFind::LastMatchedRow`.
/// 6. Verify only row #5's data is returned.
#[ tokio::test ]
async fn test_mock_get_row_by_custom_row_key_multiple_matches_last() 
{
  let server = MockServer::start();
  let spreadsheet_id = "12345";

  let get_mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab1!E:E" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body
      ( 
        json!
        ( 
          {
            "range" : "tab1!E:E",
            "majorDimension" : "COLUMNS",
            "values" : [ [ "foo", "targetVal", "bar", "targetVal" ] ]
          } 
        ) 
      );
  } );

  let batch_get_mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values:batchGet" )
      .query_param( "ranges", "tab1!A4:ZZZ4" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body
      ( 
        json!
        ( 
          {
            "spreadsheetId" : "12345",
            "valueRanges" : [ 
              {
                "range" : "tab1!A4:ZZZ4",
                "majorDimension" : "ROWS",
                "values" : [ [ "Charlie", "X", "targetVal" ] ]
              } 
            ]
          }
        )
      );
  } );

  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  let fetched_rows = get_row_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( "targetVal" ) ),
    OnFind::LastMatchedRow,
  )
  .await
  .expect( "get_row_by_custom_row_key failed" );

  assert_eq!( fetched_rows.len(), 1 );
  assert_eq!( fetched_rows[ 0 ].len(), 3 );
  assert_eq!( fetched_rows[ 0 ][ 0 ], json!( "Charlie" ) );
  assert_eq!( fetched_rows[ 0 ][ 2 ], json!( "targetVal" ) );

  get_mock.assert();
  batch_get_mock.assert();
}
