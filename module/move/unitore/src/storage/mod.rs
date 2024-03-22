//! Storage for frames, feeds and config files.

use crate::*;
use std::sync::Arc;
use error_tools::{ for_app::Context, Result };
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

use executor::actions::query::QueryReport;

pub mod config;
pub mod frame;
pub mod table;
pub mod feed;

/// Storage for feed frames.
#[ derive( Clone ) ]
pub struct FeedStorage< S : GStore + GStoreMut + Send >
{
  /// GlueSQL storage.
  pub storage : Arc< Mutex< Glue< S > > >,
  frame_fields : Vec< [ &'static str; 3 ] >,
}

impl< S : GStore + GStoreMut + Send > std::fmt::Debug for FeedStorage< S >
{
  fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!(f, "GlueSQL storage" )
  }
}

impl FeedStorage< SledStorage >
{
  /// Initialize new storage from configuration, create feed table.
  pub async fn init_storage( config : Config ) -> Result< Self >
  {
    let storage = SledStorage::try_from( config.clone() )
    .context( format!( "Failed to initialize storage with config {:?}", config ) )?
    ;

    let mut glue = Glue::new( storage );

    let sub_table = table( "config" )
    .create_table_if_not_exists()
    .add_column( "path TEXT PRIMARY KEY" )
    .build()?
    ;

    sub_table.execute( &mut glue ).await?;

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
    .build()?
    ;

    feed_table.execute( &mut glue ).await?;

    let frame_fields = vec!
    [
      [ "id", "TEXT", "A unique identifier for this frame in the feed. " ],
      [ "title", "TEXT", "Title of the frame" ],
      [ "updated", "TIMESTAMP", "Time at which this item was fetched from source." ],
      [ "authors", "TEXT", "List of authors of the frame, optional." ],
      [ "content", "TEXT", "The content of the frame in html or plain text, optional." ],
      [ "links", "TEXT", "List of links associated with this item of related Web page and attachments." ],
      [ "summary", "TEXT", "Short summary, abstract, or excerpt of the frame item, optional." ],
      [ "categories", "TEXT", "Specifies a list of categories that the item belongs to." ],
      [ "published", "TIMESTAMP", "Time at which this item was first published or updated." ],
      [ "source", "TEXT", "Specifies the source feed if the frame was copied from one feed into another feed, optional." ],
      [ "rights", "TEXT", "Conveys information about copyrights over the feed, optional." ],
      [ "media", "TEXT", "List of media oblects, encountered in the frame, optional." ],
      [ "language", "TEXT", "The language specified on the item, optional." ],
      [ "feed_link", "TEXT", "Link of feed that contains this frame." ],
    ];
    let mut table = table( "frame" ).create_table_if_not_exists().add_column( "id TEXT PRIMARY KEY" );

    for column in frame_fields.iter().skip( 1 ).take( frame_fields.len() - 2 )
    {
      table = table.add_column( format!( "{} {}", column[ 0 ], column[ 1 ] ).as_str() );
    }

    let table = table.add_column( "feed_link TEXT FOREIGN KEY REFERENCES feed(link)" )
    .build()?
    ;

    table.execute( &mut glue ).await?;

    Ok( Self{ storage : Arc::new( Mutex::new( glue ) ), frame_fields } )
  }
}

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait( ?Send ) ]
pub trait Store
{
  /// Execute custom query passed as String.
  async fn execute_query( &mut self, query : String ) -> Result< QueryReport >;
}

#[ async_trait::async_trait( ?Send ) ]
impl< S : GStore + GStoreMut + Send > Store for FeedStorage< S >
{
  async fn execute_query( &mut self, query : String ) -> Result< QueryReport >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( &query ).await.context( "Failed to execute query" )?;

    let report = QueryReport ( payloads );

    Ok( report )
  }
}
