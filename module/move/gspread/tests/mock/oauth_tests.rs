//!
//! OAuth2 tests.
//! 

use httpmock::prelude::*;
use reqwest;
use serde_json::json;

#[ tokio::test ]
async fn oauth2_first_endpoint_with_mock()
{
  let server = MockServer::start();
  let mock = server.mock( | when, then | {
    when.method( GET )
      .path( "/o/oauth2/auth" )
      .query_param( "scope", "https://www.googleapis.com/auth/drive.readonly" )
      .query_param( "access_type", "offline" )
      .query_param( "redirect_uri", "http://localhost:44444" )
      .query_param( "response_type", "code" )
      .query_param( "client_id", "YOUR_CLIENT_ID" );
    then.status( 302 );
  });

  let response = reqwest::get
  ( 
    server.url
    ( 
    "/o/oauth2/auth?\
    scope=https://www.googleapis.com/auth/drive.readonly&\
    access_type=offline&\
    redirect_uri=http://localhost:44444&\
    response_type=code&\
    client_id=YOUR_CLIENT_ID"
    ) 
  )
  .await
  .unwrap();
  
  mock.assert();

  assert_eq!( response.status(), 302 );
}


#[ tokio::test ]
async fn oauth2_second_endpoint_with_mock()
{
  let server = MockServer::start();

  // url == first endpoint
  let mock = server.mock( | when, then | {
    when.path( "/o/oauth2/auth" )
      .query_param( "scope", "https://..." );
    then.status( 302 );
  } );

  // in real program at that point we have to open generated url and give access to our program from browser
  let response = reqwest::get( server.url( "/o/oauth2/auth?scope=https://..." ) ).await.unwrap();

  mock.assert();

  assert_eq!( response.status(), 302 );
}

#[ tokio::test ]
async fn oauth2_third_endpoint_with_mock()
{
  let server = MockServer::start();
  let body = r#"code=AUTHORIZATION_CODE&client_secret=YOUR_CLIENT_SECRET&"#;
  let mock = server.mock( | when, then | {
    when.method( POST )
      .path( "/token" )
      .header("Content-Type", "application/json" )
      .body( body );
    then.status( 200 )
      .header("Content-Type", "application/json" )
      .json_body
      ( 
        json!
        ( 
          {
            "access_token" : "access_token",
            "token_type" : "Bearer",
            "expires_in" : "3600",
            "scope" : "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email openid"
          }
        ) 
      );
  });

  let response = reqwest::Client::new()
  .post( server.url( "/token" ) )
  .header( "Content-Type", "application/json" )
  .body( body )
  .send()
  .await
  .unwrap();

  mock.assert();

  assert_eq!( response.status(), 200 );
}