//!
//! Set cells tests.
//! In these examples:
//!   - url is /v4/spreadsheets/{spreadsheet_id}}/values/{range}
//!   - everything is fake: spreadsheet_id, sheet's name, range and response json
//! 

use httpmock::prelude::*;
use reqwest;

#[ tokio::test ]
async fn test_set_cells_with_mock()
{
  let server = MockServer::start();
  let body = r#"{ "id": "2", "A": "new_val1", "B": "new_val2"}"#;
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/v4/spreadsheets/12345/values/tab3!A2:B2" )
      .header( "Content-Type", "application/json" )
      .body( body );
    // returns amount of updated cells
    then.status( 201 )
      .header( "Content-Type", "application/json" )
      .body( "2" );
  } );

  let response = reqwest::Client::new()
  .post( server.url( "/v4/spreadsheets/12345/values/tab3!A2:B2" ) )
  .header( "Content-Type", "application/json" )
  .body( body )
  .send()
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 201 );
}

#[ tokio::test ]
async fn test_set_cells_wrong_row_with_mock()
{
  let server = MockServer::start();
  let body = r#"{ "id": "a", "A": "new_val1", "B": "new_val2"}"#;
  let response_body = r#"{"error":{"code":400,"message":"Invalid data[0]: Unable to parse range: tab3!Aa","status":"INVALID_ARGUMENT"}}"#;
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/v4/spreadsheets/12345/values/tab3!Aa:Ba" )
      .header( "Content-Type", "application/json" )
      .body( body );
    then.status( 400 )
      .header( "Content-Type", "application/json" )
      .body( response_body );
  } );

  let response = reqwest::Client::new()
  .post( server.url( "/v4/spreadsheets/12345/values/tab3!Aa:Ba" ) )
  .header( "Content-Type", "application/json" )
  .body( body )
  .send()
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 400 );
}