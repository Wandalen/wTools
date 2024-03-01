//! Execute plan.

use super::*;
use feed_config::SubscriptionConfig;
use gluesql::{ core::executor::Payload, sled_storage::sled::Config };
use retriever::{ FeedClient, FeedFetch };
use feed_config::read_feed_config;
use storage::{ FeedStorage, FeedStore };
// use wca::prelude::*;

/// Run feed updates.
pub fn execute() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let ca = wca::CommandsAggregator::former()
  .grammar
  ( [
    wca::Command::former()
    .phrase( "frames.download" )
    .hint( "Subscribe to feed from sources provided in config file" )
    .subject( "Source file", wca::Type::String, false )
    .form(),
    wca::Command::former()
    .phrase( "fields.list" )
    .hint( "List all fields in Frames table with explanation." )
    .form(),
    wca::Command::former()
    .phrase( "feeds.list" )
    .hint( "List all feeds from storage." )
    .form(),
    wca::Command::former()
    .phrase( "frames.list" )
    .hint( "List all frames saved in storage." )
    .form(),
  ] )
  .executor
  ( [
    ( "frames.download".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( path ) = args.get_owned( 0 )
      {
        let rt  = tokio::runtime::Runtime::new()?;
        rt.block_on( fetch_from_config( path ) ).unwrap();
      }

      Ok( () )
    } ) ),

    ( "fields.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let rt  = tokio::runtime::Runtime::new()?;
      rt.block_on( list_fields() ).unwrap();
      Ok( () )
    } ) ),

    ( "frames.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let rt  = tokio::runtime::Runtime::new()?;
      rt.block_on( list_frames() ).unwrap();
      Ok( () )
    } ) ),

    ( "feeds.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let rt  = tokio::runtime::Runtime::new()?;
      rt.block_on( list_feeds() ).unwrap();
      Ok( () )
    } ) ),
  ] )
  .help_variants( [ wca::HelpVariants::General, wca::HelpVariants::SubjectCommand ] )
  .build();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args.join( " " ) )?;

  Ok( () )
}

/// Manages feed subsriptions and updates.
pub struct FeedManager< C, S : FeedStore + Send >
{
  /// Subscription configuration with link and update period.
  pub config : Vec< SubscriptionConfig >,
  /// Storage for saving feed.
  pub storage : S,
  /// Client for fetching feed from links in FeedConfig.
  pub client : C,
}

impl< S : FeedStore + Send > FeedManager< FeedClient, S >
{
  /// Create new instance of FeedManager.
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
  /// Set configurations for subscriptions.
  pub fn set_config( &mut self, configs : Vec< SubscriptionConfig > )
  {
    self.config = configs;
  }

  /// Set client for fetching feed.
  pub fn set_client( &mut self, client : C )
  {
    self.client = client;
  }

  /// Update modified frames and save new items.
  pub async fn update_feed( &mut self ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let mut feeds = Vec::new();
    for i in  0..self.config.len()
    {
      let feed = self.client.fetch( self.config[ i ].link.clone() ).await?;
      feeds.push( feed );
    }
    self.storage.process_feeds( feeds ).await?;
    Ok( () )
  }

  /// Get all frames currently in storage.
  pub async fn get_all_frames( &mut self ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.get_all_frames().await
  }

  /// Get all feeds currently in storage.
  pub async fn get_all_feeds( &mut self ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.get_all_feeds().await
  }

  /// Execute custom query, print result.
  pub async fn execute_custom_query( &mut self, query : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.execute_query( query ).await
  }

  /// Get columns names of Frames table.
  pub fn get_columns( &mut self ) -> Result< Vec< [ &'static str; 3 ] >, Box< dyn std::error::Error + Send + Sync > >
  {
    Ok( self.storage.columns_titles() )
  }
}

/// Update all feed from subscriptions in file.
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

/// List all fields.
pub async fn list_fields() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;

  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  let fields = manager.get_columns()?;
  for field in fields
  {
    println!( "{}, type {} : {}\n", field[ 0 ], field[ 1 ], field[ 2 ] );
  }

  Ok( () )
}

/// List all frames.
pub async fn list_frames() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;

  let feed_storage = FeedStorage::init_storage( config ).await?;
  let mut manager = FeedManager::new( feed_storage );
  let frames = manager.get_all_frames().await?;
  println!( "{:#?}", frames );

  Ok( () )
}

/// List all feeds.
pub async fn list_feeds() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;

  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  let feeds = manager.get_all_feeds().await?;

  println!( "{:#?}", feeds );

  Ok( () )
}