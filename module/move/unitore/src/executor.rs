//! Execute plan.

use super::*;
use feed_config::SubscriptionConfig;
use gluesql::{ core::executor::Payload, sled_storage::sled::Config, prelude::Value };
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
    wca::Command::former()
    .phrase( "config.add" )
    .hint( "Add subscription configuration." )
    .subject( "Link", wca::Type::String, false )
    .form(),
    wca::Command::former()
    .phrase( "config.delete" )
    .hint( "Delete subscription configuraiton." )
    .subject( "Link", wca::Type::String, false )
    .form(),
    wca::Command::former()
    .phrase( "config.list" )
    .hint( "List all subscription configurations saved in storage." )
    .form(),
    wca::Command::former()
    .phrase( "query.execute" )
    .hint( "Execute custom query." )
    .subject( "Query", wca::Type::List( Box::new( wca::Type::String ), ',' ), false )
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


    ( "config.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let rt  = tokio::runtime::Runtime::new()?;
      rt.block_on( list_subscriptions() ).unwrap();
      Ok( () )
    } ) ),

    ( "config.add".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( link ) = args.get_owned( 0 )
      {
        let config = SubscriptionConfig
        {
          link,
          period : std::time::Duration::from_secs( 1000 ),
        };
        let rt  = tokio::runtime::Runtime::new()?;
        rt.block_on( add_subscription( config ) ).unwrap();
      }

      Ok( () )
    } ) ),

    ( "config.delete".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( link ) = args.get_owned( 0 )
      {
        let rt  = tokio::runtime::Runtime::new()?;
        rt.block_on( remove_subscription( link ) ).unwrap();
      }

      Ok( () )
    } ) ),
    ( "query.execute".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      println!( "{:?}", args );
      if let Some( query ) = args.get_owned::<Vec<String>>( 0 )
      {
        println!( "{:?}", query );
        let rt  = tokio::runtime::Runtime::new()?;
        rt.block_on( execute_query( query.join( " " ) ) ).unwrap();
      }

      Ok( () )
    } ) ),
  ] )
  .help_variants( [ wca::HelpVariants::General, wca::HelpVariants::SubjectCommand ] )
  .build();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args.join( " " ) )?;

  Ok( () )
}

pub struct FramesReport
{
  pub updated_frames : usize,
  pub new_frames : usize,
}

impl FramesReport
{
  pub fn new() -> Self
  {
    Self
    {
      updated_frames : 0,
      new_frames : 0,
    }
  }
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

  pub async fn list_subscriptions( &mut self ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.list_subscriptions().await
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

pub async fn list_subscriptions() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;
  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  let res = manager.list_subscriptions().await?;
  println!( "{:?}", res );

  Ok( () )
}

pub async fn add_subscription( sub_config : SubscriptionConfig ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;
  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  manager.storage.add_subscription( sub_config ).await?;

  Ok( () )
}

pub async fn remove_subscription( link : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;
  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  manager.storage.remove_subscription( link ).await?;

  Ok( () )
}

pub async fn execute_query( query : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "data/temp".to_owned() )
  ;
  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  manager.storage.execute_query( query ).await?;

  Ok( () )
}