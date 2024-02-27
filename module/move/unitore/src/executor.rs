//! Execute plan.

use super::*;
use retriever::FeedClient;
use feed_config::read_feed_config;
// use wca::prelude::*;

pub fn execute() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{

  let ca = wca::CommandsAggregator::former()
  .grammar
  ([
    wca::Command::former()
    .phrase( "subscribe" )
    .hint( "Subscribe to feed from sources provided in config file" )
    .subject( "Source file", wca::Type::String, false )
    .form(),
  ])
  .executor
  ([
    ( "subscribe".to_owned(), wca::Routine::new( | ( args, props ) |
    {
      println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );

      if let Some( path ) = args.get_owned( 0 )
      {
        let rt  = tokio::runtime::Runtime::new()?;
        rt.block_on( fetch_from_config( path ) ).unwrap();
      }

      Ok( () )
    } ) ),
  ] )
  .build();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args.join( " " ) )?;

  Ok( () )
}

pub async fn fetch_from_config( file_path : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let client = FeedClient;

  let feed_configs = read_feed_config( file_path ).unwrap();

  for config in feed_configs
  {
    client.fetch( config.link ).await?;
  }
  Ok( () )
}
