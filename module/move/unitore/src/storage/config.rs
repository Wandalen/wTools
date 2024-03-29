//! Functionality for storing and retrieving config files.

use super::*;
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
  async fn add_config( &mut self, config : &Config ) -> Result< Payload >;

  /// Remove subscription.
  async fn delete_config( &mut self, config : &Config ) -> Result< Payload >;

  /// List subscriptions.
  async fn list_configs( &mut self ) -> Result< Payload >;
}

// qqq : port and adapters should not be in the same file
// Ideally, they should be in different crates, but you should at least put them in different folders
// there should be a `sled_adapter`` folder

#[ async_trait::async_trait( ?Send ) ]
impl ConfigStore for FeedStorage< SledStorage >
{
  async fn add_config( &mut self, config : &Config ) -> Result< Payload >
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

    // let res = match &res
    // {
    //   Err( err ) =>
    //   {
    //     if let gluesql::core::error::Error::Validate( val_err ) = err
    //     {
    //       let res = match val_err
    //       {
    //         gluesql::core::error::ValidateError::DuplicateEntryOnPrimaryKeyField( _ ) =>
    //         {
    //           res.context( "Config with same path already exists." )
    //         },
    //         _ => res.into()
    //       };

    //       res
    //     }
    //     res.into()
    //   },
    //   Ok( _ ) => res.into(),
    // };

    Ok( res? )
  }

  async fn delete_config( &mut self, config : &Config ) -> Result< Payload >
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

  async fn list_configs( &mut self ) -> Result< Payload >
  {
    let res = table( "config" ).select().execute( &mut *self.storage.lock().await ).await?;
    Ok( res )
  }
}

// qqq : use AbsolutePath newtype from `path_tools`
// qqq : normalize all paths with `path_tools::path::normalize`
// https://docs.rs/proper_path_tools/latest/proper_path_tools/path/fn.normalize.html

// unitore .query.execute \'SELECT \* FROM feed\'
// qqq : something is broken in this table. also lack of association with config files

// unitore .query.execute \'SELECT \* FROM x\'
// qqq : it is not obvious where one record ends and another begins
