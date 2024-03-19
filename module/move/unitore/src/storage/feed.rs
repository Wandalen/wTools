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

use executor::actions::
{
  feed::FeedsReport,
  frame::{ UpdateReport, SelectedEntries, FramesReport },
};
use storage::{ FeedStorage, frame::{ FrameStore, RowValue } };
use wca::wtools::Itertools;

#[ derive( Debug ) ]
pub struct Feed
{
  pub link : url::Url,
  pub title : Option< String >,
  pub updated : Option< DateTime< Utc > >,
  pub authors : Option< String >,
  pub description : Option< String >,
  pub published : Option< DateTime< Utc > >,
  pub update_period : Duration,
}

impl Feed
{
  pub fn new( link : url::Url, update_period : Duration ) -> Self
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
    }
  }
}

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait( ?Send ) ]
pub trait FeedStore
{

  /// Insert items from list into feed table.
  async fn update_feed( &mut self, feed : Vec< Feed > ) -> Result< () >;

  /// Process fetched feed, new items will be saved, modified items will be updated.
  async fn process_feeds( &mut self, feeds : Vec< ( feed_rs::model::Feed, Duration, url::Url ) > ) -> Result< UpdateReport >;

  /// Get all feeds from storage.
  async fn get_all_feeds( &mut self ) -> Result< FeedsReport >;

  /// Add feeds entries.
  async fn add_feeds( &mut self, feeds : Vec< Feed > ) -> Result< Payload >;
}

#[ async_trait::async_trait( ?Send ) ]
impl FeedStore for FeedStorage< SledStorage >
{
  async fn get_all_feeds( &mut self ) -> Result< FeedsReport >
  {
    let res = table( "feed" ).select().project( "title, link, update_period" ).execute( &mut *self.storage.lock().await ).await?;
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

  async fn update_feed( &mut self, feed : Vec< Feed > ) -> Result< () >
  {
    //let feeds_rows = feed.into_iter().map( | feed | FeedRow::from( feed ).0 ).collect_vec();

    for feed in feed
    {
      let _update = table( "feed" )
      .update()
      .set( "title", feed.title.map( text ).unwrap_or( null() ) )
      .set( "updated", feed.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) )
      .set( "authors", feed.authors.map( text ).unwrap_or( null() ) )
      .set( "description", feed.description.map( text ).unwrap_or( null() ) )
      .set( "published", feed.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) )
      .filter( col( "link" ).eq( feed.link.to_string() ) )
      .execute( &mut *self.storage.lock().await )
      .await
      .context( "Failed to insert feed" )?
      ;
    }

    Ok( () )
  }

  async fn process_feeds
  (
    &mut self,
    feeds : Vec< ( feed_rs::model::Feed, Duration, url::Url ) >,
  ) -> Result< UpdateReport >
  {
    let new_feed_links = feeds
    .iter()
    .map( | feed |
      feed.0.links.iter().filter_map( | link |
      {
        if let Some( media_type ) = &link.media_type
        {
          if media_type == &String::from( "application/rss+xml" )
          {
            return Some( format!( "'{}'", link.href.clone() ) );
          }
        }
        None
      } )
      .collect::< Vec< _ > >()
      .get( 0 )
      .unwrap_or( &format!( "'{}'", feed.2 ) )
      .clone()
    )
    .join( "," )
    ;

    let existing_feeds = table( "feed" )
    .select()
    .filter( format!( "link IN ({})", new_feed_links ).as_str() )
    .project( "link" )
    .execute( &mut *self.storage.lock().await )
    .await
    .context( "Failed to select links of existing feeds while saving new frames" )?
    ;

    let mut new_entries = Vec::new();
    let mut modified_entries = Vec::new();
    let mut reports = Vec::new();

    for feed in &feeds
    {
      let mut frames_report = FramesReport::new( feed.0.title.clone().unwrap().content );
      // check if feed is new
      if let Some( existing_feeds ) = existing_feeds.select()
      {

        let existing_feeds = existing_feeds
        .filter_map( | feed | feed.get( "link" ).map( | link | String::from( RowValue( link ) ) ))
        .collect_vec()
        ;

        let link = &feed.2.to_string();

        if !existing_feeds.contains( link )
        {
          self.add_feeds( vec![ feed.clone().into() ] ).await?;
          frames_report.new_frames = feed.0.entries.len();
          frames_report.is_new_feed = true;

          new_entries.extend
          (
            feed.0.entries
            .clone()
            .into_iter()
            .zip( std::iter::repeat( feed.0.id.clone() ).take( feed.0.entries.len() ) )
            .map( | entry | entry.into() )
          );
          reports.push( frames_report );
          continue;
        }
      }

      let existing_frames = table( "frame" )
      .select()
      .filter(col( "feed_link" ).eq( text( feed.0.id.clone() ) ) )
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

    if new_entries.len() > 0
    {
      let _saved_report = self.save_frames( new_entries ).await?;
    }
    if modified_entries.len() > 0
    {
      let _updated_report = self.update_frames( modified_entries ).await?;
    }

    Ok( UpdateReport( reports ) )
  }

  async fn add_feeds( &mut self, feed : Vec< Feed > ) -> Result< Payload >
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
      update_period",
    )
    .values( feeds_rows )
    .execute( &mut *self.storage.lock().await )
    .await
    .context( "Failed to insert feeds" )?
    ;

    Ok( insert )
  }
}

/// Feed in format convenient for saving in storage.
#[ derive( Debug ) ]
pub struct FeedRow( pub Vec< ExprNode< 'static > > );

impl FeedRow
{
  /// Create new feed row for storage.
  pub fn new( feed_link : String, update_period : Duration ) -> Self
  {
    FeedRow( vec!
    [
      text( feed_link ),
      null(),
      null(),
      null(),
      null(),
      null(),
      text( update_period.as_secs().to_string() ),
    ] )
  }
}

impl From< ( feed_rs::model::Feed, Duration, String ) > for FeedRow
{
  fn from( value : ( feed_rs::model::Feed, Duration, String ) ) -> Self
  {
    let duration = value.1;
    let link = value.2;
    let value = value.0;
    
    let row = vec!
    [
      value.links.iter().filter_map( | link |
        {
          if let Some( media_type ) = &link.media_type
          {
            if media_type == &String::from( "application/rss+xml" )
            {
              return Some( text( link.href.clone() ) );
            }
          } 
          None
        } )
        .collect::< Vec< _ > >()
        .get( 0 )
        .unwrap_or( &text( link ) )
        .clone(),
      value.title.clone().map( | title | text( title.content ) ).unwrap_or( null() ),
      value.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( value.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) ),
      value.description.clone().map( | desc | text( desc.content ) ).unwrap_or( null() ),
      value.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( duration.as_secs().to_string() ),
    ];
    FeedRow( row )
  }
}

impl From< ( feed_rs::model::Feed, Duration, url::Url ) > for Feed
{
  fn from( val : ( feed_rs::model::Feed, Duration, url::Url ) ) -> Self
  {
    let duration = val.1;
    let link = val.2;
    let value = val.0;

    let authors = value.authors.into_iter().map( | p | p.name ).collect::< Vec< _ > >();
    let description = value.description.map( | desc | desc.content );

    Self
    {
      link,
      title : value.title.map( | title | title.content ),
      updated : value.updated,
      published : value.published,
      description,
      authors : ( !authors.is_empty() ).then( || authors.join( ", " ) ),
      update_period : duration,
    }
    
    
  }
}

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
    ]
  }
}
