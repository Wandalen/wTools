//! Feed storage entity and storage functions.

use crate::*;
use std::time::Duration;
use error_tools::Result;
use gluesql::core::
{
  ast_builder::{ null, text, timestamp, ExprNode },
  executor::Payload,
  chrono::{ Utc, DateTime, SecondsFormat },
};

use action::
{
  feed::FeedsReport,
  frame::UpdateReport,
};

/// Feed item.
#[ derive( Debug ) ]
pub struct Feed
{
  /// Link to feed source.
  pub link : url::Url,
  /// Title of feed.
  pub title : Option< String >,
  /// Last time the feed was fetched.
  pub updated : Option< DateTime< Utc > >,
  /// Authors of feed.
  pub authors : Option< String >,
  /// Short description of feed content.
  pub description : Option< String >,
  /// Date and time when feed was published.
  pub published : Option< DateTime< Utc > >,
  /// How often the feed frames must be fetched.
  pub update_period : Duration,
  /// Path to config file, from which this feed was saved.
  pub config_file : String,
}

impl Feed
{
  /// Create new feed item from source url and update period.
  pub fn new( link : url::Url, update_period : Duration, config: String ) -> Self
  {
    Self
    {
      link,
      title : None,
      updated : None,
      authors : None,
      description : None,
      published : None,
      update_period,
      config_file : config,
    }
  }
}

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait( ?Send ) ]
pub trait FeedStore
{
  /// Save new feeds to storage.
  /// New feeds from config files that doesn't exist in storage will be inserted into `feed` table.
  async fn feeds_save( &mut self, feeds : Vec< Feed > ) -> Result< Payload >;

  /// Update existing feeds in storage with new information.
  /// Feed is updated one time during first fetch. 
  async fn feeds_update( &mut self, feed : Vec< Feed > ) -> Result< () >;

  /// Process new fetched feeds and frames.
  /// Frames from recent fetch will be sorted into three categories:
  /// - new items that will be inserted into `frame` table;
  /// - modified items that will be updated;
  /// - unchanged frames saved from previous fetches will be ignored.
  async fn feeds_process( &mut self, feeds : Vec< ( feed_rs::model::Feed, Duration, url::Url ) > ) -> Result< UpdateReport >;

  /// Get existing feeds from storage.
  /// Retrieves all feeds from `feed` table in storage.
  async fn feeds_list( &mut self ) -> Result< FeedsReport >;
}
// qqq : poor description and probably naming. improve, please
// aaa : updated description


/// Get convenient format of frame item for using with GlueSQL expression builder.
/// Converts from Feed struct into vec of GlueSQL expression nodes. 
impl From< Feed > for Vec< ExprNode< 'static > >
{
  fn from( value : Feed ) -> Self
  {
    vec!
    [
      text( value.link.to_string() ),
      value.title.map( text ).unwrap_or( null() ),
      value.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      value.authors.map( text ).unwrap_or( null() ),
      value.description.map( text ).unwrap_or( null() ),
      value.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( value.update_period.as_secs().to_string() ),
      text( value.config_file ),
    ]
  }
}
