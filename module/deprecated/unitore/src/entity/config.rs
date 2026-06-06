//! Functionality for storing and retrieving config files.

use error_tools ::untyped ::Result;
use gluesql ::core ::executor ::Payload;

/// Config file path.
#[ derive( Debug ) ]
pub struct Config( pub String );

impl Config
{
  /// Create new config with provided path.
  #[must_use] 
  pub fn new( path: String ) -> Self
  {
  Self( path )
 }

  /// Get path of config file.
  #[must_use] 
  pub fn path( &self ) -> String
  {
  self.0.clone()
 }
}

/// Functionality of config storing.
#[ async_trait ::async_trait( ?Send ) ]
pub trait ConfigStore
{
  /// Add subscription.
  async fn config_add( &mut self, config: &Config ) -> Result< Payload >;

  /// Remove subscription.
  async fn config_delete( &mut self, config: &Config ) -> Result< Payload >;

  /// List subscriptions.
  async fn config_list( &mut self ) -> Result< Payload >;
}

// qqq: use AbsolutePath newtype from `path_tools`
// qqq: normalize all paths with `path_tools ::path ::normalize`
// https: //docs.rs/pth/latest/pth/path/fn.normalize.html
// added path normalization


