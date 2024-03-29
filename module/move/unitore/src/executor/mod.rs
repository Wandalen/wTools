//! Execute plan.

use super::*;
use feed_config::SubscriptionConfig;
use gluesql::sled_storage::{ sled::Config, SledStorage };
use retriever::{ FeedClient, FeedFetch };
use storage::{ Store, FeedStorage, feed::FeedStore, config::ConfigStore, table::TableStore, frame::FrameStore };
use wca::{ Args, Type };
use executor::actions::Report;
use error_tools::Result;

pub mod actions;
use actions::
{
  frame::{ list_frames, download_frames },
  feed::list_feeds,
  config::{ add_config, delete_config, list_configs },
  query::execute_query,
  table::{ list_columns, list_tables },
};

fn action< 'a, F, Fut, R >( async_endpoint : F, args : &'a Args ) -> Result< R >
where
  F : FnOnce( FeedStorage< SledStorage >, &'a Args ) -> Fut,
  Fut : std::future::Future< Output = Result< R > >,
  R : actions::Report,
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
    async_endpoint( feed_storage, args ).await
  } )
}

/// Run feed updates.
pub fn execute() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  let ca = wca::CommandsAggregator::former()
  .command( "frames.download" )
    .hint( "Download frames from feed sources provided in config files." )
    .long_hint(concat!
    (
      "Download frames from feed sources provided in config files.\n",
      "    Example: .frames.download",
    ))
    .routine( | args |
    {
      match action( download_frames, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "feeds.list" )
    .long_hint( concat!
    (
      "List all feeds from storage.\n",
      "    Example: .feeds.list",
    ))
    .routine( | args |
    {
      match action( list_feeds, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "frames.list" )
    .long_hint( concat!
    (
      "List all frames saved in storage.\n",
      "    Example: .frames.list",
    ))
    .routine( | args |
    {
      match action( list_frames, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "config.add" )
    .long_hint( concat!
    (
      "Add file with feeds configurations. Subject: path to config file.\n",
      "    Example: .config.add ./config/feeds.toml\n",
      "    The file should contain config entities with fields:\n",
      "   - `update_period` : update frequency for feed. Example values: `12h`, `1h 20min`, `2days 5h`;\n",
      "   - `link` : URL for feed source;\n\n",
      "    Example:\n",
      "    [[config]]\n",
      "    update_period = \"1min\"\n",
      "    link = \"https://feeds.bbci.co.uk/news/world/rss.xml\"\n",
    ))
    .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
    .routine( | args : Args |
    {
      match action( add_config, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "config.delete" )
    .long_hint( concat!
    (
      "Delete file with feeds configuraiton. Subject: path to config file.\n",
      "    Example: .config.delete ./config/feeds.toml",
    ))
    .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
    .routine( | args : Args |
    {
      match action( delete_config, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "config.list" )
    .long_hint( concat!
    (
      "List all config files saved in storage.\n",
      "    Example: .config.list",
    ))
    .routine( | args |
    {
      match action( list_configs, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "tables.list" )
    .long_hint( concat!
    (
      "List all tables saved in storage.\n",
      "    Example: .tables.list",
    ))
    .routine( | args |
    {
      match action( list_tables, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "table.list" )
    .long_hint( concat!
    (
      "List fields of specified table.\n",
      "Subject: table name.\n",
      "    Example: .table.list feed",
    ))
    .subject().hint( "Name" ).kind( wca::Type::String ).optional( false ).end()
    .routine( | args : Args |
    {
      match action( list_columns, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()

  .command( "query.execute" )
    .long_hint( concat!
    (
      "Execute custom query. Subject: query string, with special characters escaped.\n",
      "    Example query:\n",
      "  - select all frames:\n",
      r#"    .query.execute \'SELECT \* FROM frame\'"#,
      "\n",
      "  - select title and link to the most recent frame:\n",
      r#"    .query.execute \'SELECT title, links, MIN\(published\) FROM frame\'"#,
      "\n\n",
    ))
    .subject().hint( "Query" ).kind( Type::List( Type::String.into(), ' ' ) ).optional( false ).end()
    .routine( | args : Args |
    {
      match action( execute_query, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) => println!( "{:?}", err ),
      }
    })
    .end()
  .help_variants( [ wca::HelpVariants::General, wca::HelpVariants::SubjectCommand ] )
  .perform();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args )?;

  Ok( () )
}

/// Manages feed subsriptions and updates.
pub struct FeedManager< C, S : FeedStore + ConfigStore + FrameStore + Store + Send >
{
  /// Subscription configuration with link and update period.
  pub config : Vec< SubscriptionConfig >,
  /// Storage for saving feed.
  pub storage : S,
  /// Client for fetching feed from links in FeedConfig.
  pub client : C,
}

impl< C, S : FeedStore + ConfigStore + FrameStore + Store + Send > std::fmt::Debug for FeedManager< C, S >
{
  fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!(f, "Feed manager with storage and client" )
  }
}

impl< S : FeedStore + ConfigStore + FrameStore + TableStore + Store + Send > FeedManager< FeedClient, S >
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

impl< C : FeedFetch, S : FeedStore + ConfigStore + FrameStore + TableStore + Store + Send > FeedManager< C, S >
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

  /// Execute custom query, print result.
  pub async fn execute_custom_query( &mut self, query : String ) -> Result< impl actions::Report >
  {
    self.storage.execute_query( query ).await
  }

}
