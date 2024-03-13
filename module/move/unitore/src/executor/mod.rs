//! Execute plan.

use super::*;
use feed_config::SubscriptionConfig;
use gluesql::sled_storage::{sled::Config, SledStorage};
use retriever::{ FeedClient, FeedFetch };
use storage::{ FeedStorage, FeedStore };
use wca::{ Args, Type };
use executor::endpoints::Report;
// use wca::prelude::*;

pub mod endpoints;
use endpoints::{
  list_fields::list_fields,
  frames::{ list_frames, download_frames, ListReport },
  feeds::list_feeds,
  config::{ add_config, remove_config, list_configs },
  query::execute_query,
  table::{ list_columns, list_tables },
  list_fields::FieldsReport,
};

use std::future::Future;

fn endpoint< 'a, F, Fut, R >( async_endpoint : F, args : &'a Args ) -> Result< R, Box< dyn std::error::Error + Send + Sync > >
where
  F : FnOnce( FeedStorage< SledStorage >, &'a Args ) -> Fut,
  Fut : Future< Output = Result< R, Box< dyn std::error::Error + Send + Sync > > >,
  R : endpoints::Report,
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
      match endpoint( download_frames, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
      }
    })
    .end()
    .command( "fields.list" )
    .long_hint( concat!
    (
      "List all fields in frame table with explanation and type.\n",
      "    Example: .fields.list",
    ))
    .routine( | args |
    {
      match endpoint( list_fields, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
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
      match endpoint( list_feeds, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
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
      match endpoint( list_frames, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
      }
    })
    .end()

  .command( "config.add" )
    .long_hint( concat!
    (
      "Add file with feeds configurations. Subject: path to config file.\n",
      "    Example: .config.add ./config/feeds.toml",
    ))
    .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
    .routine( | args : Args |
    {
      // if let Some( path ) = args.get_owned::< wca::Value >( 0 )
      // {
        match endpoint( add_config, &args )
        {
          Ok( report ) => report.report(),
          Err( report ) => println!( "{report}" ),
        }
      //}
    })
    .end()

  .command( "config.delete" )
    .long_hint( concat!
    (
      "Delete file with feeds configuraiton. Subject: path to config file.\n",
      "    Example: .config.delete ./config/feeds.toml",
    ))
    .subject().hint( "Link" ).kind( Type::Path ).optional( false ).end()
    .routine( | args : Args |
    {
      match endpoint( remove_config, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
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
      match endpoint( list_configs, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
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
      match endpoint( list_tables, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
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
      match endpoint( list_columns, &args )
      {
        Ok( report ) => report.report(),
        Err( report ) => println!( "{report}" ),
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
      match endpoint( execute_query, &args )
      {
        Ok( report ) => report.report(),
        Err( err ) =>
        {
          println!( "Error while executing SQL query:" );
          println!( "{}", err );
        }
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
  pub async fn update_feed( &mut self, subscriptions : Vec< SubscriptionConfig > ) -> Result< impl endpoints::Report, Box< dyn std::error::Error + Send + Sync > >
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
  pub async fn get_all_feeds( &mut self ) -> Result< endpoints::feeds::FeedsReport, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.get_all_feeds().await
  }

  /// Execute custom query, print result.
  pub async fn execute_custom_query( &mut self, query : String ) -> Result< impl endpoints::Report, Box< dyn std::error::Error + Send + Sync > >
  {
    self.storage.execute_query( query ).await
  }

  /// Get columns names of Frames table.
  pub fn get_columns( &mut self ) -> Result< FieldsReport, Box< dyn std::error::Error + Send + Sync > >
  {
    Ok( self.storage.columns_titles() )
  }

}
