//!
//! Tests to update
//! 

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::
{
  update_rows_by_custom_row_key, 
  OnFail, 
  OnFind
};
use gcore::ApplicationSecret;
use gcore::client::
{
  BatchUpdateValuesResponse, 
  Client, 
  Dimension, 
  ValueRange
};


/// # What
/// We check that updating rows in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client
/// 3. Call `update_rows_by_custom_row_key`.
/// 4. Check results.
#[ tokio::test ]
async fn test_mock_update_rows_by_custom_row_key_on_fail_nothing_should_work() 
{
  // 1. Start a mock server.
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
            "values" : [ [ "12", "12", "12", "12" ] ]
          } 
        ) 
      );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call update_rows_by_custom_row_key.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( 122 ) ),
    row_key_val,
    OnFind::FirstMatchedRow,
    OnFail::Nothing
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );

  assert_eq!( batch_result.spreadsheet_id.as_deref(), None );
  assert_eq!( batch_result.total_updated_cells, None );
  assert_eq!( batch_result.total_updated_rows, None );
  assert_eq!( batch_result.total_updated_columns, None );

  get_mock.assert();
}

/// # What
/// We check that updating rows in a Google Spreadsheet returns the correct response.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Call `update_rows_by_custom_row_key`.
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_update_rows_by_custom_row_key_on_fail_error_should_panic() 
{
  // Start a mock server.
  let server = MockServer::start();
  let spreadsheet_id = "12345";

  let _get_mock = server.mock( | when, then | {
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
            "values" : [ [ "12", "12", "12", "12" ] ]
          } 
        ) 
      );
  } );

  // 2. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call update_rows_by_custom_row_key
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let _batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( 122 ) ),
    row_key_val,
    OnFind::FirstMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );
}

/// # What
/// We test that in case where we didn't find passed cell, OnFail::AppendRow in works correct.
///
/// # How
/// 1. Start a mock server for getting our tabel.
/// 2. Start a mock server for adding a row.
/// 3. Create a client
/// 4. Call `update_rows_by_custom_row_key`.
/// 5. Check resaults.
#[ tokio::test ]
async fn test_mock_update_rows_by_custom_row_key_on_find_append_row_should_work() 
{
  // 1. Start get_mock.
  let server = MockServer::start();
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
            "values" : [ [ "12", "12", "12", "12" ] ]
          } 
        ) 
      );
  } );

  // 2. Start append_row_mock.
  let append_row_mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values/tab1!A1:append" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body_values_append );
  } );

  let mock_batch_update = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &body_batch_update );
  } );

  // 3. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 4. Call update_rows_by_custom_row_key.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( 122 ) ),
    row_key_val,
    OnFind::FirstMatchedRow,
    OnFail::AppendRow
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );

  // 5. Check results.
  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( "12345" ) );
  assert_eq!( batch_result.total_updated_rows, Some( 1 ) );

  get_mock.assert();
  append_row_mock.assert();
  mock_batch_update.assert();
}

/// # What
/// We test that in case where we didn't find passed cell, OnFail::AppendRow in works correct.
///
/// # How
/// 1. Start a mock server for getting our tabel.
/// 2. Start a mock server for adding a row.
/// 3. Create a client
/// 4. Call `update_rows_by_custom_row_key`.
/// 5. Check resaults.
#[ tokio::test ]
async fn test_mock_update_rows_by_custom_row_key_on_find_update_first_row_should_work() 
{
  // 1. Start get_mock.
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
            "values" : [ [ "12", "12", "12", "12" ] ]
          } 
        ) 
      );
  } );

  let mocked_value_ranges = vec!
  [
    ValueRange 
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( format!( "tab1!A2" ) ),
      values : Some( vec![ vec![ json!( "Hello" ) ] ] ),
    },

    ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( format!( "tab1!B2" ) ),
      values : Some( vec![ vec![ json!( 123 ) ] ] ),
    }
  ];

  let response_body = BatchUpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 1 ),
    total_updated_sheets : Some( 1 ),
    total_updated_cells : Some( 2 ),
    total_updated_columns : Some( 2 ),
    responses : Some( mocked_value_ranges )
  };

  // 2. Start update_mock.
  let update_mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 3. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 4. Call update_rows_by_custom_row_key.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( "12" ) ),
    row_key_val,
    OnFind::FirstMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );

  // 5. Check results.
  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( "12345" ) );
  assert_eq!( batch_result.total_updated_cells, Some( 2 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 2 ) );
  assert_eq!( batch_result.total_updated_rows, Some( 1 ) );
  assert_eq!( batch_result.total_updated_sheets, Some( 1 ) );
  
  let responses = batch_result
  .responses
  .expect( "No responses found in BatchUpdateValuesResponse" );
  assert_eq!( responses.len(), 2 );

  get_mock.assert();
  update_mock.assert();
}

/// # What
/// We test that in case where we didn't find passed cell, OnFail::UpdateAllMatchesRows in works correct.
///
/// # How
/// 1. Start a mock server for getting our tabel.
/// 2. Start a mock server for update rows.
/// 3. Create a client
/// 4. Call `update_rows_by_custom_row_key`.
/// 5. Check resaults.
#[ tokio::test ]
async fn test_mock_update_rows_by_custom_row_key_on_find_update_all_rows_should_work() 
{
  // 1. Start get_mock.
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
            "values" : [ [ "12", "12", "12", "12" ] ]
          } 
        ) 
      );
  } );

  let mut mocked_value_ranges = vec![];
  for i in 1..=4 
  {
    mocked_value_ranges.push
    (
      ValueRange 
      {
        major_dimension : Some( Dimension::Row ),
        range : Some( format!( "tab1!A{}", i ) ),
        values : Some( vec![ vec![ json!( "Hello" ) ] ] ),
      }
    );
    mocked_value_ranges.push
    (
      ValueRange
      {
        major_dimension : Some( Dimension::Row ),
        range : Some( format!( "tab1!B{}", i ) ),
        values : Some( vec![ vec![ json!( 123 ) ] ] ),
      }
    );
  }

  let response_body = BatchUpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 4 ),
    total_updated_sheets : Some( 1 ),
    total_updated_cells : Some( 8 ),
    total_updated_columns : Some( 2 ),
    responses : Some( mocked_value_ranges )
  };

  // 2. Start update_mock.
  let update_mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 3. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 4. Call update_rows_by_custom_row_key.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( "12" ) ),
    row_key_val,
    OnFind::AllMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );

  println!( "{:?}", batch_result );

  // 5. Check results.
  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( "12345" ) );
  assert_eq!( batch_result.total_updated_cells, Some( 8 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 2 ) );
  assert_eq!( batch_result.total_updated_rows, Some( 4 ) );
  assert_eq!( batch_result.total_updated_sheets, Some( 1 ) );
  
  let responses = batch_result
  .responses
  .expect( "No responses found in BatchUpdateValuesResponse" );
  assert_eq!( responses.len(), 8 );

  get_mock.assert();
  update_mock.assert();
}

/// # What
/// We test that in case where we find passed cell, OnFail::UpdateLastMatchedRow in works correct.
///
/// # How
/// 1. Start a mock server for getting our tabel.
/// 2. Start a mock server for update a row.
/// 3. Create a client
/// 4. Call `update_rows_by_custom_row_key`.
/// 5. Check resaults.
#[ tokio::test ]
async fn test_mock_update_rows_by_custom_row_key_on_find_update_last_row_should_work() 
{
  // 1. Start get_mock.
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
            "values" : [ [ "12", "12", "12", "12" ] ]
          } 
        ) 
      );
  } );

  let mocked_value_ranges = vec!
  [
    ValueRange 
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( format!( "tab1!A2" ) ),
      values : Some( vec![ vec![ json!( "Hello" ) ] ] ),
    },
    ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      range : Some( format!( "tab1!B2" ) ),
      values : Some( vec![ vec![ json!( 123 ) ] ] ),
    }
  ];

  let response_body = BatchUpdateValuesResponse
  {
    spreadsheet_id : Some( spreadsheet_id.to_string() ),
    total_updated_rows : Some( 1 ),
    total_updated_sheets : Some( 1 ),
    total_updated_cells : Some( 2 ),
    total_updated_columns : Some( 2 ),
    responses : Some( mocked_value_ranges )
  };

  // 2. Start update_mock.
  let update_mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/12345/values:batchUpdate" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body_obj( &response_body );
  } );

  // 3. Create a client.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 4. Call update_rows_by_custom_row_key.
  let mut row_key_val = std::collections::HashMap::new();
  row_key_val.insert( "A".to_string(), json!( "Hello" ) );
  row_key_val.insert( "B".to_string(), json!( 123 ) );

  let batch_result = update_rows_by_custom_row_key
  (
    &client,
    spreadsheet_id,
    "tab1",
    ( "E", json!( "12" ) ),
    row_key_val,
    OnFind::LastMatchedRow,
    OnFail::Error
  )
  .await
  .expect( "update_rows_by_custom_row_key failed" );

  // 5. Check results.
  assert_eq!( batch_result.spreadsheet_id.as_deref(), Some( "12345" ) );
  assert_eq!( batch_result.total_updated_rows, Some( 1 ) );
  assert_eq!( batch_result.total_updated_sheets, Some( 1 ) );
  assert_eq!( batch_result.total_updated_cells, Some( 2 ) );
  assert_eq!( batch_result.total_updated_columns, Some( 2 ) );
  
  let responses = batch_result
  .responses
  .expect( "No responses found in BatchUpdateValuesResponse" );
  assert_eq!( responses.len(), 2);

  get_mock.assert();
  update_mock.assert();
}