//!
//! Get rows tests.
//! In these examples:
//!   - url is /v4/spreadsheets/{spreadsheet_id}}/values/{range}
//!   - everything is fake: spreadsheet_id, sheet's name, range and response json
//! 

use httpmock::prelude::*;
use reqwest;

#[ tokio::test ]
async fn test_get_rows_with_mock()
{
  let server = MockServer::start();
  let body = r#"{"A2" : "Steeve", "B2": "John", "A3": "Seva", "B3": "Oleg" }"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/A2:B3" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .body( body );
  });

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/A2:B3" 
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}

#[ tokio::test ]
async fn test_get_rows_with_spaces_with_mock()
{
  let server = MockServer::start();
  let body = r#"{"A2" : "Steeve", "B2": "", "A3": "Seva", "B3": "Oleg" }"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/A2:B3" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .body( body );
  });

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/A2:B3" 
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}

#[ tokio::test ]
async fn test_get_rows_empty_with_mock()
{
  let server = MockServer::start();
  let body = r#"{}"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/A2:B3" );
    then.status( 200 )
      .header( "Content-Type", "application/json" )
      .body( body );
  });

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/A2:B3" 
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}