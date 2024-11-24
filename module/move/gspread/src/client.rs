//!
//! Client of API.
//!

mod private
{

  use google_sheets4 as sheets4;
  use sheets4::Sheets;
  use sheets4::hyper_rustls;
  use sheets4::hyper_util;
  use sheets4::yup_oauth2::
  {
    self,
    ApplicationSecret
  };
  use hyper_util::client::legacy::connect::HttpConnector;

  pub use hyper_util::client::legacy::Client;

  use std::
  {
    error::Error,
  };

  use crate::*;
  use secret::Secret;

  pub type SheetsType = Sheets< hyper_rustls::HttpsConnector< HttpConnector > >;

  pub async fn hub( secrets: &Secret ) -> Result< SheetsType, Box< dyn Error > >
  {
    let secret: ApplicationSecret = ApplicationSecret
    {
      client_id : secrets.CLIENT_ID.clone(),
      auth_uri : secrets.AUTH_URI.clone(),
      token_uri : secrets.TOKEN_URI.clone(),
      client_secret : secrets.CLIENT_SECRET.clone(),
      .. Default::default()
    };

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder
    (
      secret,
      yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    let client = Client::builder
    (
      hyper_util::rt::TokioExecutor::new()
    )
    .build
    (
      hyper_rustls::HttpsConnectorBuilder::new()
      .with_native_roots()
      .unwrap()
      .https_or_http()
      .enable_http1()
      .build()
    );

    Ok( Sheets::new( client, auth ) )
  }


}

crate::mod_interface!
{
  exposed use
  {
    hub,
    Client,
    SheetsType
  };
}