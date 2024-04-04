//! Feed storage entity and storage functions.

use crate::*;
use std::time::Duration;
use error_tools::{ for_app::Context, Result };
use gluesql::
{
  core::
  {
    ast_builder::{ null, col, table, text, Execute, timestamp, ExprNode },
    data::Value,
    executor::Payload,
    chrono::{ Utc, DateTime, SecondsFormat },
  },
  sled_storage::SledStorage,
};

use action::
{
  feed::FeedsReport,
  frame::{ UpdateReport, SelectedEntries, FramesReport },
};
use storage::FeedStorage;
use entity::frame::FrameStore;
use wca::wtools::Itertools;

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

#[ async_trait::async_trait( ?Send ) ]
impl FeedStore for FeedStorage< SledStorage >
{
  async fn feeds_list( &mut self ) -> Result< FeedsReport >
  {
    let res = table( "feed" )
    .select()
    .project( "title, link, update_period, config_file" )
    .execute( &mut *self.storage.lock().await )
    .await?
    ;
  
    let mut report = FeedsReport::new();
    match res
    {
      Payload::Select { labels: label_vec, rows: rows_vec } =>
      {
        report.0 = SelectedEntries
        {
          selected_rows : rows_vec,
          selected_columns : label_vec,
        }
      },
      _ => {},
    }

    Ok( report )
  }

  async fn feeds_update( &mut self, feed : Vec< Feed > ) -> Result< () >
  {
    for feed in feed
    {
      let _update = table( "feed" )
      .update()
      .set( "title", feed.title.map( text ).unwrap_or( null() ) )
      .set(
        "updated",
        feed.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      )
      .set( "authors", feed.authors.map( text ).unwrap_or( null() ) )
      .set( "description", feed.description.map( text ).unwrap_or( null() ) )
      .set(
        "published",
        feed.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      )
      .filter( col( "link" ).eq( feed.link.to_string() ) )
      .execute( &mut *self.storage.lock().await )
      .await
      .context( "Failed to insert feed" )?
      ;
    }

    Ok( () )
  }

  async fn feeds_process
  (
    &mut self,
    feeds : Vec< ( feed_rs::model::Feed, Duration, url::Url ) >,
  ) -> Result< UpdateReport >
  {
    let mut new_entries = Vec::new();
    let mut modified_entries = Vec::new();
    let mut reports = Vec::new();

    for feed in &feeds
    {
      let mut frames_report = FramesReport::new( feed.0.title.clone().unwrap().content );

      let existing_frames = table( "frame" )
      .select()
      .filter( col( "feed_link" ).eq( text( feed.2.to_string() ) ) )
      .project( "id, published" )
      .execute( &mut *self.storage.lock().await )
      .await
      .context( "Failed to get existing frames while saving new frames" )?
      ;

      if let Some( rows ) = existing_frames.select()
      {
        let rows = rows.collect::< Vec< _ > >();
        frames_report.existing_frames = rows.len();
        let existing_entries = rows.iter()
        .map( | r | ( r.get( "id" ).map( | &val | val.clone() ), r.get( "published" ).map( | &val | val.clone() ) ) )
        .flat_map( | ( id, published ) |
          id.map( | id |
            (
              id,
              published.map( | date |
                {
                  match date
                  {
                    Value::Timestamp( date_time ) => Some( date_time ),
                    _ => None,
                  }
                } )
              .flatten()
            )
          )
        )
        .flat_map( | ( id, published ) | match id { Value::Str( id ) => Some( ( id, published ) ), _ => None } )
        .collect_vec()
        ;

        let existing_ids = existing_entries.iter().map( | ( id, _ ) | id ).collect_vec();
        for entry in &feed.0.entries
        {
          // if extry with same id is already in db, check if it is updated
          if let Some( position ) = existing_ids.iter().position( | &id | id == &entry.id )
          {
            if let Some( date ) = existing_entries[ position ].1
            {
              if date.and_utc() != entry.published.unwrap()
              {
                frames_report.updated_frames += 1;
                modified_entries.push( ( entry.clone(), feed.2.to_string() ).into() );
              }
            }
          }
          else
          {
            frames_report.new_frames += 1;
            new_entries.push( ( entry.clone(), feed.2.to_string() ).into() );
          }
        }
      }
      reports.push( frames_report );
    }

    if !new_entries.is_empty()
    {
      let _saved_report = self.frames_save( new_entries ).await?;
    }
    if !modified_entries.is_empty()
    {
      let _updated_report = self.frames_update( modified_entries ).await?;
    }

    Ok( UpdateReport( reports ) )
  }

  async fn feeds_save( &mut self, feed : Vec< Feed > ) -> Result< Payload >
  {
    let feeds_rows : Vec< Vec< ExprNode< 'static > > > = feed.into_iter().map( | feed | feed.into() ).collect_vec();

    let insert = table( "feed" )
    .insert()
    .columns
    (
      "link,
      title,
      updated,
      authors,
      description,
      published,
      update_period,
      config_file",
    )
    .values( feeds_rows )
    .execute( &mut *self.storage.lock().await )
    .await
    .context( "Failed to insert feeds" )?
    ;

    Ok( insert )
  }
}

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
