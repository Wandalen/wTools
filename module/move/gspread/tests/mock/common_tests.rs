//!
//! Common tests for every function.
//! 

use httpmock::prelude::*;
use gspread::*;
use actions::gspread::get_cell;
use gcore::
{
  client::Client, 
  ApplicationSecret
};


/// # What
/// We check that any function will panic with wrong `spreadsheet_id`.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Send a HTTP request.
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_wrong_spreadsheet_id_should_panic() 
{
  // 1. Start server.
  let server = MockServer::start();
  let _ = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .body( r#""# );
  } );

  // 2. Create a client. 
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send HTTP request.
  let _ = get_cell( &client, "", "tab2", "A2" )
  .await
  .expect( "get_cell failed" );
}

/// # What
/// We check that any function will panic with wrong `sheet_name`.
///
/// # How
/// 1. Start a mock server.
/// 2. Create a client.
/// 3. Send a HTTP request.
#[ tokio::test ]
#[ should_panic ]
async fn test_mock_wrong_sheet_name_should_panic() 
{
  // 1. Start server.
  let server = MockServer::start();
  let _ = server.mock( | when, then | {
    when.method( GET )
      .path( "/12345/values/tab2!A2" );
    then
      .status( 200 )
      .header( "Content-Type", "application/json" )
      .body( r#""# );
  } );

  // 2. Create a client. 
  let endpoint = server.url( "" );
  let client : Client< '_, ApplicationSecret > = Client::former()
  .endpoint( &*endpoint )
  .form();

  // 3. Send HTTP request.
  let _ = get_cell( &client, "12345", "wrong_name", "A2" )
  .await
  .expect( "get_cell failed" );
}