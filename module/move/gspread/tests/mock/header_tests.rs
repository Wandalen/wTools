//!
//! Get header tests.
//! In these examples:
//!   - url is /v4/spreadsheets/{spreadsheet_id}}/values/{range}
//!   - everything is fake: spreadsheet_id, sheet's name, range and response json
//! 

use httpmock::prelude::*;
use reqwest;


#[ tokio::test ]
async fn test_get_header()
{
  let server = MockServer::start();
  let body = r#"{ "A1": "Name", "B1": "Surname", "C1": "Age" }"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab1!A1:Z1" );
    then.status( 200 )
      .header("Content-Type", "application/json" )
      .body( body );
  } );

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/tab1!A1:Z1"
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 )

}

#[ tokio::test ]
async fn test_get_header_with_spaces_with_mock()
{
  let server = MockServer::start();
  let body = r#"{ "A1": "Name", "B1": "", "C1": "Age" }"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab1!A1:Z1" );
    then.status( 200 )
      .header("Content-Type", "application/json" )
      .body( body );
  } );

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/tab1!A1:Z1"
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 )
}

#[ tokio::test ]
async fn test_get_header_empty_with_mock()
{
  let server = MockServer::start();

  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab1!A1:Z1" );
    then.status( 200 )
      .header("Content-Type", "application/json" )
      .body( r#"{}"# );
  } );

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/tab1!A1:Z1"
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 )
}

#[ tokio::test ]
async fn test_get_header_with_empty_end_with_mock()
{
  let server = MockServer::start();
  let body = r#"{ "A1": "Name", "B1": "Surname" }"#;
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/v4/spreadsheets/12345/values/tab1!A1:Z1" );
    then.status( 200 )
      .header("Content-Type", "application/json" )
      .body( body );
  } );

  let response = reqwest::get
  ( 
    server.url
    ( 
      "/v4/spreadsheets/12345/values/tab1!A1:Z1"
    ) 
  )
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 )
}