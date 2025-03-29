//!
//! Tests for `copy_to` function.
//!

use httpmock::prelude::*;
use serde_json::json;
use gspread::*;
use actions::gspread::copy_to;
use gcore::
{
  client::Client, 
  ApplicationSecret
};

/// # What
/// We test copying a sheet from one spreadsheet to another.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client pointing to that mock server.
/// 3. Mock a `POST` request to /{spreadsheet_id}/sheets/{sheet_id}:copyTo.
/// 4. Call `copy_to`.
/// 5. Verify the response (e.g. `sheetId` and `title`).
#[ tokio::test ]
async fn test_mock_copy_to_should_work() 
{
  let server = MockServer::start();
  let spreadsheet_id = "12345";
  let sheet_id = "67890";
  let destination_spreadsheet_id = "destination123";

  let body = json!
  (
    {
      "sheetId" : 999,
      "title" : "CopiedSheet",
      "index" : 2
    }
  );

  // 1. Mock the POST request for copying the sheet.
  let copy_mock = server.mock( | when, then | {
    when.method( POST )
      .path( format!( "/{}/sheets/{}:copyTo", spreadsheet_id, sheet_id ) );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .json_body( body.clone() );
  } );

  // 2. Create a client pointing to our mock server.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `copy_to`.
  let response = copy_to
  (
    &client, 
    spreadsheet_id, 
    sheet_id, 
    destination_spreadsheet_id
  )
  .await
  .expect( "copy_to failed" );

  // 4. Verify that the mock was indeed called.
  copy_mock.assert();

  // 5. Check the returned `SheetProperties`.
  assert_eq!( response.sheet_id, Some( 999 ), "Expected sheetId to be 999" );
  assert_eq!( response.title.as_deref(), Some( "CopiedSheet" ), "Expected title to be 'CopiedSheet'" );
  assert_eq!( response.index, Some( 2 ), "Expected index to be 2" );
}

/// # What
/// We test error handling for `copy_to` when the API responds with an error.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Mock a `POST` request that returns an error (400).
/// 4. Call `copy_to` and expect a panic (due to `.expect(...)`).
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_copy_to_should_panic() 
{
  let server = MockServer::start();
  let spreadsheet_id = "12345";
  let sheet_id = "67890";
  let destination_spreadsheet_id = "destination123";

  // 1. Mock a failing POST request.
  let _copy_mock = server.mock( | when, then | {
    when.method( POST )
      .path( format!( "/{}/sheets/{}:copyTo", spreadsheet_id, sheet_id ) );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .json_body
      ( 
        json!
        (
          {
            "error" : { "message" : "Invalid request or missing permissions." }
          }
        ) 
      );
  });

  // 2. Create a client pointing to our mock server.
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Call `copy_to`, which should panic because we `.expect(...)`.
  let response = copy_to
  (
    &client,
    spreadsheet_id,
    sheet_id,
    destination_spreadsheet_id
  )
  .await
  .expect( "copy_to failed. Ok!" );

  // We'll never reach here because of the panic.
  println!( "{:?}", response );
}
