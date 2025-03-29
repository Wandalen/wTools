//! Config file operation with Sled storage.

use crate::*;
use error_tools::{ err, untyped::Result };
use gluesql::
{
  core::
  {
    ast_builder::{ col, table, text, Execute, },
    executor::Payload,
  },
  sled_storage::SledStorage,
};
use entity::config::{ Config, ConfigStore };
use sled_adapter::FeedStorage;

#[ async_trait::async_trait( ?Send ) ]
impl ConfigStore for FeedStorage< SledStorage >
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
    .execute( &mut *self.0.lock().await )
    .await;

    Ok( res? )
  }

  async fn config_delete( &mut self, config : &Config ) -> Result< Payload >
  {
    let res = table( "config" )
    .delete()
    .filter( col( "path" ).eq( format!( "'{}'", config.path() ) ) )
    .execute( &mut *self.0.lock().await )
    .await?;

    if res == Payload::Delete( 0 )
    {
      return Err( err!( format!( "Config file with path {} not found in storage", config.path() ) ) )
    }

    Ok( res )
  }

  async fn config_list( &mut self ) -> Result< Payload >
  {
    let res = table( "config" ).select().execute( &mut *self.0.lock().await ).await?;
    Ok( res )
  }
}
