//! Execute plan.
use super::*;
use feed_config::SubscriptionConfig;
use gluesql::sled_storage::sled::Config;
use retriever::{ FeedClient, FeedFetch };
use feed_config::read_feed_config;
use storage::{ FeedStorage, FeedStore };
use report::{ Report, FramesReport, FieldsReport, FeedsReport, QueryReport, ConfigReport, UpdateReport };
// use wca::prelude::*;

/// Run feed updates.
pub fn execute() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let ca = wca::CommandsAggregator::former()
  .grammar
  ( [
    wca::Command::former()
    .phrase( "frames.download" )
    .hint( "Download frames from feed sources provided in config files." )
    .form(),
    wca::Command::former()
    .phrase( "fields.list" )
    .hint( "List all fields in Frames table with explanation and type." )
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
    .hint( "Add subscription configuration. Subject: link to feed source." )
    .subject( "Link", wca::Type::Path, false )
    .form(),
    wca::Command::former()
    .phrase( "config.delete" )
    .hint( "Delete subscription configuraiton. Subject: link to feed source." )
    .subject( "Link", wca::Type::String, false )
    .form(),
    wca::Command::former()
    .phrase( "config.list" )
    .hint( "List all subscription configurations saved in storage." )
    .form(),
    wca::Command::former()
    .phrase( "query.execute" )
    .hint
    (
      concat!
      (
        "Execute custom query. Subject: query string, with special characters escaped.\n",
        "Example query:\n  - select all frames:\n",
        r#"  .query.execute \'SELECT \* FROM Frames\'"#,
        "\n",
        "  - select title and link to the most recent frame:\n",
        r#"  .query.execute \'SELECT title, links, MIN\(published\) FROM Frames\'"#,
        "\n\n",
      )
    )
    .subject( "Query", wca::Type::List( Box::new( wca::Type::String ), ' ' ), false )
    .form(),
  ] )
  .executor
  ( [
    ( "frames.download".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = update_feed().unwrap();
      report.report();

      Ok( () )
    } ) ),

    ( "fields.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_fields().unwrap();
      report.report();

      Ok( () )
    } ) ),

    ( "frames.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_frames().unwrap();
      report.report();

      Ok( () )
    } ) ),

    ( "feeds.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_feeds().unwrap();
      report.report();

      Ok( () )
    } ) ),

    ( "config.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_subscriptions().unwrap();
      report.report();

      Ok( () )
    } ) ),

    ( "config.add".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( path ) = args.get_owned::< wca::Value >( 0 )
      {
        let report = add_config( path.into() ).unwrap();
        report.report();
      }

      Ok( () )
    } ) ),

    ( "config.delete".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( link ) = args.get_owned( 0 )
      {
        let report = remove_subscription( link ).unwrap();
        report.report();
      }

      Ok( () )
    } ) ),
    ( "query.execute".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( query ) = args.get_owned::< Vec::< String > >( 0 )
      {
        let report = execute_query( query.join( " " ) ).unwrap();
        report.report();
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
  pub async fn update_feed( &mut self, subscriptions : Vec< SubscriptionConfig > ) -> Result< UpdateReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let mut feeds = Vec::new();
    for i in  0..subscriptions.len()
    {
      let feed = self.client.fetch( subscriptions[ i ].link.clone() ).await?;
      feeds.push( ( feed, subscriptions[ i ].period.clone() ) );
    }
    self.storage.process_feeds( feeds ).await
  }

  /// Get all frames currently in storage.
  pub async fn get_all_frames( &mut self ) -> Result< UpdateReport, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.get_all_frames().await
  }

  /// Get all feeds currently in storage.
  pub async fn get_all_feeds( &mut self ) -> Result< FeedsReport, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.get_all_feeds().await
  }

  /// Execute custom query, print result.
  pub async fn execute_custom_query( &mut self, query : String ) -> Result< QueryReport, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.execute_query( query ).await
  }

  /// Get columns names of Frames table.
  pub fn get_columns( &mut self ) -> Result< FieldsReport, Box< dyn std::error::Error + Send + Sync > >
  {
    Ok( self.storage.columns_titles() )
  }

  pub async fn list_subscriptions( &mut self ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.list_subscriptions().await
  }
}

// /// Update all feed from subscriptions in file.
// pub fn fetch_from_file( file_path : String ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
// {
//   let rt  = tokio::runtime::Runtime::new()?;
//   let report = rt.block_on( async move 
//   {
//     let config = Config::default()
//     .path( "data/temp".to_owned() )
//     ;
//     let feed_configs = read_feed_config( file_path ).unwrap();
//     let feed_storage = FeedStorage::init_storage( config ).await?;
  
//     let mut manager = FeedManager::new( feed_storage );
//     manager.set_config( feed_configs );
//     manager.update_feed().await

//   } );

//   report
// }

/// Update all feed from config files saved in storage.
pub fn update_feed() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let rt  = tokio::runtime::Runtime::new()?;
  let report = rt.block_on( async move
  {
    let config = Config::default()
    .path( "_data/temp".to_owned() )
    ;

    //let feed_configs = read_feed_config( file_path ).unwrap();
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    let configs = manager.list_subscriptions().await?.configs();

    let mut subscriptions = Vec::new();
    for config in configs
    {
      
      let sub_vec = read_feed_config( config )?;
      subscriptions.extend( sub_vec );
    }
    manager.update_feed( subscriptions ).await

  } );

  report
}

/// List all fields.
pub fn list_fields() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let config = Config::default()
    .path( "_data/temp".to_owned() )
    ;

    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.get_columns()
  } )
}

/// List all frames.
pub fn list_frames() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "_data/temp".to_owned() )
  ;
  let rt  = tokio::runtime::Runtime::new()?;

  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;
    let mut manager = FeedManager::new( feed_storage );
    manager.get_all_frames().await
  } )
}

/// List all feeds.
pub fn list_feeds() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "_data/temp".to_owned() )
  ;

  let rt  = tokio::runtime::Runtime::new()?;
  let report = rt.block_on( async move
    {
      let feed_storage = FeedStorage::init_storage( config ).await?;

      let mut manager = FeedManager::new( feed_storage );
      manager.get_all_feeds().await
    } )?;

  Ok( report )

}

pub fn list_subscriptions() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "_data/temp".to_owned() )
  ;
  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.list_subscriptions().await
  } )
}

pub fn add_config( path : std::path::PathBuf ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "_data/temp".to_owned() )
  ;

  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;
    let path = path.canonicalize().expect( "Invalid path" );


    let mut manager = FeedManager::new( feed_storage );
    manager.storage.add_config( path.to_string_lossy().to_string() ).await
  } )
}

pub fn remove_subscription( link : String ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "_data/temp".to_owned() )
  ;
  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.remove_subscription( link ).await
  } )
}

pub fn execute_query( query : String ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let config = Config::default()
  .path( "_data/temp".to_owned() )
  ;
  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.execute_query( query ).await
  } )
}