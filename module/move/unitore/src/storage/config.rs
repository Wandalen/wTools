//! Functionality for storing and retrieving config files.

use crate::*;
use error_tools::{ err, Result };
use gluesql::
{
  core::
  {
    ast_builder::{ col, table, text, Execute },
    executor::Payload,
  },
  sled_storage::SledStorage,
};

/// Config file path.
#[ derive( Debug ) ]
pub struct Config( pub String );

impl Config
{
  /// Create new config with provided path.
  pub fn new( path : String ) -> Self
  {
    Self( path )
  }

  /// Get path of config file.
  pub fn path( &self ) -> String
  {
    self.0.clone()
  }
}

/// Functionality of config storing.
#[ async_trait::async_trait( ?Send ) ]
pub trait ConfigStore
{
  /// Add subscription.
  async fn config_add( &mut self, config : &Config ) -> Result< Payload >;

  /// Remove subscription.
  async fn config_delete( &mut self, config : &Config ) -> Result< Payload >;

  /// List subscriptions.
  async fn config_list( &mut self ) -> Result< Payload >;
}

#[ async_trait::async_trait( ?Send ) ]
impl ConfigStore for storage::FeedStorage< SledStorage >
{
  async fn config_add( &mut self, config : &Config ) -> Result< Payload >
  {
    let res = table( "config" )
    .insert()
    .columns
    (
      "path",
    )
    .values( vec![ vec![ text( config.path() ) ] ] )
    .execute( &mut *self.storage.lock().await )
    .await;

    Ok( res? )
  }

  async fn config_delete( &mut self, config : &Config ) -> Result< Payload >
  {
    let res = table( "config" )
    .delete()
    .filter( col( "path" ).eq( format!( "'{}'", config.path() ) ) )
    .execute( &mut *self.storage.lock().await )
    .await?;

    if res == Payload::Delete( 0 )
    {
      return Err( err!( format!( "Config file with path {} not found in storage", config.path() ) ) )
    }

    Ok( res )
  }

  async fn config_list( &mut self ) -> Result< Payload >
  {
    let res = table( "config" ).select().execute( &mut *self.storage.lock().await ).await?;
    Ok( res )
  }
}
