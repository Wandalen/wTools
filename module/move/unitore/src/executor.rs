//! Execute plan.

use super::*;
use feed_config::FeedConfig;
use gluesql::sled_storage::sled::Config;
use retriever::{ FeedClient, FeedFetch };
use feed_config::read_feed_config;
use storage::{ FeedStorage, FeedStore };
// use wca::prelude::*;

pub fn execute() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let ca = wca::CommandsAggregator::former()
  .grammar
  ( [
    wca::Command::former()
    .phrase( "subscribe" )
    .hint( "Subscribe to feed from sources provided in config file" )
    .subject( "Source file", wca::Type::String, false )
    .form(),
  ] )
  .executor
  ( [
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

pub struct FeedManager< C, S : FeedStore + Send >
{
  pub config : Vec< FeedConfig >,
  pub storage : S,
  pub client : C,
}

impl< S : FeedStore + Send > FeedManager< FeedClient, S >
{
  pub fn new( storage : S ) -> FeedManager< FeedClient, S >
  {
    Self
    { 
      storage,
      config : Vec::new(),
      client : FeedClient,
    }
  }
}

impl< C : FeedFetch, S : FeedStore + Send > FeedManager< C, S >
{
  pub fn set_config( &mut self, configs : Vec< FeedConfig > )
  {
    self.config = configs;
  }

  pub fn set_client( &mut self, client : C )
  {
    self.client = client;
  }

  pub async fn update_feed( &mut self ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    for i in  0..self.config.len()
    {
      let feed = self.client.fetch( self.config[ i ].link.clone() ).await?;
      self.storage.save_feed( feed.entries ).await.unwrap();
    }

    Ok( () )
  }
}

pub async fn fetch_from_config( file_path : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;
  let feed_configs = read_feed_config( file_path ).unwrap();
  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  manager.set_config( feed_configs );
  manager.update_feed().await?;

  Ok( () )
}
