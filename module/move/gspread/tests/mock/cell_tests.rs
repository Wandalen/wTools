//!
//! Get and set cell tests.
//! In these examples:
//!   - url is /v4/spreadsheets/{spreadsheet_id}}/values/{range}
//!   - everything is fake: spreadsheet_id, sheet's name, range and response json
//! 

use httpmock::prelude::*;
use reqwest;

#[ tokio::test ]
async fn test_get_cell_with_mock()
{
  let server = MockServer::start();
  let body = r#"{ "A2": "Steeve" }"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab2!R2C1" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .body( body );
  } );

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/tab2!R2C1"
    )
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}

#[ tokio::test ]
async fn test_get_cell_empty_with_mock()
{
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab2!R2C1" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .body( r#"{}"# );
  } );

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/tab2!R2C1"
    )
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}

#[ tokio::test ]
async fn test_set_cell_with_mock()
{
  let server = MockServer::start();
  let body = r#"A2": "Some value"#;
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/v4/spreadsheets/12345/values/tab2!R2C1" )
      .header("content-type", "application/json")
      .body( body );
    // returns amount of updated cells
    then.status( 201 )
      .header( "Content-Type", "application/json" )
      .body( "1" );
  } );

  let response = reqwest::Client::new()
  .post( server.url( "/v4/spreadsheets/12345/values/tab2!R2C1" ) )
  .header( "Content-Type", "application/json" )
  .body( body )
  .send()
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 201 );
}