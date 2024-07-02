//! Actions and report for config files.

use std::path::PathBuf;

use crate::*;
use error_tools::{ untyped::Context, Result };
use sled_adapter::FeedStorage;
use entity::
{
  feed::{ FeedStore, Feed },
  config::{ ConfigStore, Config },
};
use action::Report;
use gluesql::{ prelude::Payload, sled_storage::SledStorage };

/// Add configuration file with subscriptions to storage.
pub async fn config_add( mut storage : FeedStorage< SledStorage >, path : &PathBuf ) -> Result< impl Report >
{
  let path = proper_path_tools::path::normalize( path );

  let mut err_str = format!( "Invalid path for config file {:?}", path );

  let start = path.components().next();
  if let Some( start ) = start
  {
    let abs_path : &std::path::Path = start.as_ref();
    let abs_path = abs_path.canonicalize();
    if let Ok( mut abs_path ) = abs_path
    {
      for component in path.components().skip( 1 )
      {
        abs_path.push( component );
      }
      err_str = format!( "Invalid path for config file {:?}", abs_path );
    }
  }

  if !path.exists()
  {
    return Err( error_tools::untyped::Error::msg( err_str ) );
  }

  let abs_path = path.canonicalize()?;
  let config = Config::new( abs_path.to_string_lossy().to_string() );

  let config_report = storage
  .config_add( &config )
  .await
  .context( "Added 0 config files.\n Failed to add config file to storage." )?
  ;

  let feeds = feed_config::read( config.path() )?
  .into_iter()
  .map( | feed | Feed::new( feed.link, feed.update_period, config.path() ) )
  .collect::< Vec< _ > >()
  ;

  let new_feeds = storage.feeds_save( feeds ).await?;

  Ok( ConfigReport{ payload : config_report, new_feeds : Some( new_feeds ) } )
}

/// Remove configuration file from storage.
pub async fn config_delete( mut storage : FeedStorage< SledStorage >, path : &PathBuf ) -> Result< impl Report >
{
  let path = proper_path_tools::path::normalize( path );
  let path = path.canonicalize().context( format!( "Invalid path for config file {:?}", path ) )?;
  let config = Config::new( path.to_string_lossy().to_string() );

  Ok( ConfigReport::new(
    storage
    .config_delete( &config )
    .await
    .context( "Failed to remove config from storage." )?
  ) )
}

/// List all files with subscriptions that are currently in storage.
pub async fn config_list( mut storage : FeedStorage< SledStorage >, _args : &wca::Args ) -> Result< impl Report >
{
  Ok( ConfigReport::new( storage.config_list().await? ) )
}

/// Information about result of command for subscription config.
#[ derive( Debug ) ]
pub struct ConfigReport
{
  payload : Payload,
  new_feeds : Option< Payload >,
}

impl ConfigReport
{
  /// Create new report for config report with provided payload.
  pub fn new( payload : Payload ) -> Self
  {
    Self { payload, new_feeds : None }
  }
}

impl std::fmt::Display for ConfigReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    const EMPTY_CELL : &'static str = "";

    match &self.payload
    {
      Payload::Insert( number ) =>
      {
        writeln!( f, "Added {} config file(s)", number )?;
        writeln!(
          f,
          "Added {} feed(s)",
          self.new_feeds
          .as_ref()
          .and_then( | payload |
            match payload
            {
              Payload::Insert( number ) => Some( *number ),
              _ => None,
            }
          )
          .unwrap_or_default(),
        )?;
      },
      Payload::Delete( number ) => writeln!( f, "Deleted {} config file", number )?,
      Payload::Select { labels: _label_vec, rows: rows_vec } =>
      {
        writeln!( f, "Selected configs:" )?;
        let mut rows = Vec::new();
        for row in rows_vec
        {
          rows.push( vec![ EMPTY_CELL.to_owned(), String::from( row[ 0 ].clone() ) ] );
        }

        let table = tool::table_display::plain_table( rows );
        if let Some( table ) = table
        {
          write!( f, "{}", table )?;
        }
      },
      _ => {},
    };

    Ok( () )
  }
}

impl Report for ConfigReport {}
