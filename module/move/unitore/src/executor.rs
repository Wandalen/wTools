//! Execute plan.
use super::*;
use feed_config::SubscriptionConfig;
use gluesql::sled_storage::sled::Config;
use retriever::{ FeedClient, FeedFetch };
use feed_config::read_feed_config;
use storage::{ FeedStorage, FeedStore };
use report::{ Report, FieldsReport, FeedsReport, QueryReport, ConfigReport, UpdateReport, ListReport };
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
    .long_hint(
      concat!
      (
        "Download frames from feed sources provided in config files.\n",
        "    Example: .frames.download",
      )
    )
    .form(),
    wca::Command::former()
    .phrase( "fields.list" )
    .long_hint(
      concat!
      (
        "List all fields in frame table with explanation and type.\n",
        "    Example: .fields.list",
      )
    )
    .form(),
    wca::Command::former()
    .phrase( "feeds.list" )
    .long_hint(
      concat!
      (
        "List all feeds from storage.\n",
        "    Example: .feeds.list",
      )
    )
    .form(),
    wca::Command::former()
    .phrase( "frames.list" )
    .long_hint(
      concat!
      (
        "List all frames saved in storage.\n",
        "    Example: .frames.list",
      )
    )
    .form(),
    wca::Command::former()
    .phrase( "config.add" )
    .long_hint(
      concat!
      (
        "Add toml file with feeds configurations. Subject: path to config file.\n",
        "    File content: list of \n",
        "    Example: .config.add ./config/feeds.toml",
      )
    )
    .subject( "Link", wca::Type::Path, false )
    .form(),
    wca::Command::former()
    .phrase( "config.delete" )
    .long_hint(
      concat!
      (
        "Delete file with feeds configuraiton. Subject: path to config file.\n",
        "    Example: .config.delete ./config/feeds.toml",
      )
    )
    .subject( "Link", wca::Type::String, false )
    .form(),
    wca::Command::former()
    .phrase( "config.list" )
    .long_hint(
      concat!
      (
        "List all config files saved in storage.\n",
        "    Example: .config.list",
      )
    )
    .form(),
    wca::Command::former()
    .phrase( "tables.list" )
    .long_hint(
      concat!
      (
        "List all tables saved in storage.\n",
        "    Example: .tables.list",
      )
    )
    .form(),
    wca::Command::former()
    .phrase( "table.list" )
    .long_hint(
      concat!
      (
        "List fields of specified table.\n",
        "Subject: table name.\n",
        "    Example: .table.list feed",
      )
    )
    .subject( "Name", wca::Type::String, false )
    .form(),
    wca::Command::former()
    .phrase( "query.execute" )
    .long_hint
    ( 
      concat!
      (
        "Execute custom query. Subject: query string, with special characters escaped.\n",
        "    Example query:\n",
        "  - select all frames:\n",
        r#"    .query.execute \'SELECT \* FROM frame\'"#,
        "\n",
        "  - select title and link to the most recent frame:\n",
        r#"    .query.execute \'SELECT title, links, MIN\(published\) FROM frame\'"#,
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
      let report = update_feed();
      if report.is_ok()
      {
        report.unwrap().report();
      }
      else
      {
        println!( "{}", report.unwrap_err() );
      }

      Ok( () )
    } ) ),

    ( "fields.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_fields();
      if report.is_ok()
      {
        report.unwrap().report();
      }
      else
      {
        println!( "{}", report.unwrap_err() );
      }

      Ok( () )
    } ) ),

    ( "frames.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_frames();
      if report.is_ok()
      {
        report.unwrap().report();
      }
      else
      {
        println!( "{}", report.unwrap_err() );
      }

      Ok( () )
    } ) ),

    ( "feeds.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_feeds();
      if report.is_ok()
      {
        report.unwrap().report();
      }
      else
      {
        println!( "{}", report.unwrap_err() );
      }

      Ok( () )
    } ) ),

    ( "config.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_subscriptions();
      if report.is_ok()
      {
        report.unwrap().report();
      }
      else
      {
        println!( "{}", report.unwrap_err() );
      }

      Ok( () )
    } ) ),

    ( "config.add".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( path ) = args.get_owned::< wca::Value >( 0 )
      {
        let report = add_config( path.into() );
        if report.is_ok()
        {
          report.unwrap().report();
        }
        else
        {
          println!( "{}", report.unwrap_err() );
        }
      }

      Ok( () )
    } ) ),
    ( "config.delete".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( path ) = args.get_owned( 0 )
      {
        let report = remove_subscription( path );
        if report.is_ok()
        {
          report.unwrap().report();
        }
        else
        {
          println!( "{}", report.unwrap_err() );
        }
      }

      Ok( () )
    } ) ),
    ( "table.list".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( table_name ) = args.get_owned::< String >( 0 )
      {
        let report = list_columns( table_name );
        if report.is_ok()
        {
          report.unwrap().report();
        }
        else
        {
          println!( "{}", report.unwrap_err() );
        }
      }
      Ok( () )
    } ) ),
    ( "tables.list".to_owned(), wca::Routine::new( | ( _args, _props ) |
    {
      let report = list_tables();
      if report.is_ok()
      {
        report.unwrap().report();
      }
      else
      {
        println!( "{}", report.unwrap_err() );
      }

      Ok( () )
    } ) ),
    ( "query.execute".to_owned(), wca::Routine::new( | ( args, _props ) |
    {
      if let Some( query ) = args.get_owned::< Vec::< String > >( 0 )
      {
        let report = execute_query( query.join( " " ) );
        if report.is_ok()
        {
          report.unwrap().report();
        }
        else
        {
          let err = report.unwrap_err();
          println!( "Error while executing SQL query:" );
          println!( "{}", err );
        }
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
      feeds.push( ( feed, subscriptions[ i ].update_period.clone() ) );
    }
    self.storage.process_feeds( feeds ).await
  }

  /// Get all frames currently in storage.
  pub async fn get_all_frames( &mut self ) -> Result< ListReport, Box< dyn std::error::Error + Send + Sync > >
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

/// Update all feed from config files saved in storage.
pub fn update_feed() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) );

  let rt  = tokio::runtime::Runtime::new()?;
  let report = rt.block_on( async move
  {
    let config = Config::default()
    .path( path_to_storage )
    ;

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
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let config = Config::default()
    .path( path_to_storage )
    ;

    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.get_columns()
  } )
}

/// List all frames.
pub fn list_frames() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
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
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
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
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
  ;
  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.list_subscriptions().await
  } )
}

pub fn list_tables() -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
  ;

  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.list_tables().await
  } )
}

pub fn list_columns( table_name : String ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
  ;

  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.list_columns( table_name ).await
  } )
}

pub fn add_config( path : std::path::PathBuf ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
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

pub fn remove_subscription( path : String ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
  ;

  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.remove_subscription( path ).await
  } )
}

pub fn execute_query( query : String ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
  .unwrap_or( String::from( "./_data" ) )
  ;

  let config = Config::default()
  .path( path_to_storage )
  ;
  let rt  = tokio::runtime::Runtime::new()?;
  rt.block_on( async move
  {
    let feed_storage = FeedStorage::init_storage( config ).await?;

    let mut manager = FeedManager::new( feed_storage );
    manager.storage.execute_query( query ).await
  } )
}