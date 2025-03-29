//! Storage for frames, feeds and config files.

use crate::*;
use std::sync::Arc;
use error_tools::{ untyped::Context, untyped::Result };
use tokio::sync::Mutex;
use gluesql::
{
  core::
  {
    ast_builder::{ table, Build, Execute },
    store::{ GStore, GStoreMut },
  },
  prelude::Glue,
  sled_storage::{ sled::Config, SledStorage },
};
use action::query::QueryReport;

mod frame;
mod table;
mod feed;
mod config;

/// Storage for feed frames.
#[ derive( Clone ) ]
pub struct FeedStorage< S : GStore + GStoreMut + Send >( Arc< Mutex< Glue< S > > > );

impl< S : GStore + GStoreMut + Send > std::fmt::Debug for FeedStorage< S >
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    writeln!( f, "GlueSQL storage" )
  }
}

impl FeedStorage< SledStorage >
{
  /// Initialize new storage from configuration, create feed table.
  pub async fn init_storage( config : &Config ) -> Result< Self >
  {
    let storage = SledStorage::try_from( config.clone() )
    .context( format!( "Failed to initialize storage with config {:?}", config ) )?
    ;

    let mut glue = Glue::new( storage );

    let config_table = table( "config" )
    .create_table_if_not_exists()
    .add_column( "path TEXT PRIMARY KEY" )
    .build()?
    ;

    config_table.execute( &mut glue ).await?;

    let feed_table = table( "feed" )
    .create_table_if_not_exists()
    .add_column( "link TEXT PRIMARY KEY" )
    .add_column( "type TEXT" )
    .add_column( "title TEXT" )
    .add_column( "updated TIMESTAMP" )
    .add_column( "authors TEXT" )
    .add_column( "description TEXT" )
    .add_column( "published TIMESTAMP" )
    .add_column( "update_period TEXT" )
    .add_column( "config_file TEXT FOREIGN KEY REFERENCES config(path)" )
    .build()?
    ;

    feed_table.execute( &mut glue ).await?;

    let frame_table = table( "frame" )
    .create_table_if_not_exists()
    .add_column( "id TEXT PRIMARY KEY" )
    .add_column( "title TEXT" )
    .add_column( "stored_time TIMESTAMP" )
    .add_column( "authors LIST" )
    .add_column( "content TEXT" )
    .add_column( "links LIST" )
    .add_column( "summary TEXT" )
    .add_column( "categories LIST" )
    .add_column( "published TIMESTAMP" )
    .add_column( "source TEXT" )
    .add_column( "rights TEXT" )
    .add_column( "media LIST" )
    .add_column( "language TEXT" )
    .add_column( "feed_link TEXT FOREIGN KEY REFERENCES feed(link)" )
    .build()?
    ;

    frame_table.execute( &mut glue ).await?;

    Ok( Self( Arc::new( Mutex::new( glue ) ) ) )
  }
}

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait( ?Send ) ]
pub trait Store
{
  /// Execute custom query passed as String.
  async fn query_execute( &mut self, query : String ) -> Result< QueryReport >;
}

#[ async_trait::async_trait( ?Send ) ]
impl< S : GStore + GStoreMut + Send > Store for FeedStorage< S >
{
  async fn query_execute( &mut self, query : String ) -> Result< QueryReport >
  {
    let glue = &mut *self.0.lock().await;
    let payloads = glue.execute( &query ).await.context( "Failed to execute query" )?;

    let report = QueryReport ( payloads );

    Ok( report )
  }
}
