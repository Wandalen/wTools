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

/// Feed client
#[ derive( Debug ) ]
pub struct FeedClient;

impl FeedClient
{
  /// Fetch feed.
  pub async fn fetch( &self, source: String ) -> Result< feed_rs::model::Feed, Box< dyn std::error::Error + Send + Sync > >
  {
    let https = HttpsConnector::new();
    let client = Client::builder( TokioExecutor::new() ).build::< _, Empty< Bytes > >( https );
    let mut res = client.get( source.parse()? ).await?;

    // println!( "Response status: {:?}", res.status() );
    // println!( "Response headers: {:?}", res.headers() );

    let mut feed = Vec::new();
    while let Some( next ) = res.frame().await
    {
      let frame = next?;
      if let Some( chunk ) = frame.data_ref()
      {
        feed.extend( chunk.to_vec() );
      }
    }
    let feed = feed_parser::parse( feed.as_slice() )?;
    println!("Feed | id::{:?} | published::{:?} | ttl::{:?} | entries::{:?}", feed.id, feed.published, feed.ttl, feed.entries.len() );

    for e in &feed.entries
    {
      println!("  Entry | id::{:?} | updated::{:?}", e.id, e.updated );
      println!("    summary::{:20?}", e.summary );
    }

    Ok( feed )
  }
}
