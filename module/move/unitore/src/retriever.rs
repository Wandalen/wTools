//! Feed client

// use super::*;
use hyper_tls::HttpsConnector;
use hyper_util::
{
  client::legacy::Client,
  rt::TokioExecutor,
};
use http_body_util::{ Empty, BodyExt };
use hyper::body::Bytes;
use feed_rs::parser as feed_parser;
use error_tools::{ Result, for_app::Context };

/// Fetch feed from provided source link.
#[ async_trait::async_trait ]
pub trait FeedFetch
{
  async fn fetch( &self, source : String ) -> Result< feed_rs::model::Feed >;
}

/// Feed client for fetching feed.
#[ derive( Debug ) ]
pub struct FeedClient;

#[ async_trait::async_trait ]
impl FeedFetch for FeedClient
{
  async fn fetch( &self, source : String ) -> Result< feed_rs::model::Feed >
  {
    let https = HttpsConnector::new();
    let client = Client::builder( TokioExecutor::new() ).build::< _, Empty< Bytes > >( https );
    let link = source.parse().context( "Failed to parse source link to download frames" )?;
    let mut res = client.get( link ).await?;

    let mut feed = Vec::new();
    while let Some( next ) = res.frame().await
    {
      let frame = next?;
      if let Some( chunk ) = frame.data_ref()
      {
        feed.extend( chunk.to_vec() );
      }
    }

    let feed = feed_parser::parse( feed.as_slice() ).context( "Failed to parse retrieved feeds." )?;

   ..println!( "{:#?}", feed.links );

    Ok( feed )
  }
}
