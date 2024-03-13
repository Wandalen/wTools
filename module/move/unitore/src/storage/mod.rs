use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use feed_rs::model::{ Entry, Feed };
use gluesql::
{
  core::
  {
    ast_builder::{ col, table, text, Build, Execute },
    data::Value,
    executor::Payload,
    store::{ GStore, GStoreMut },
  },
  prelude::Glue,
  sled_storage::{ sled::Config, SledStorage },
};
// qqq : ask
use crate::report::
{
  // qqq : don't put report into different file, keep the in the same file where it used
  FramesReport,
  FieldsReport,
  FeedsReport,
  SelectedEntries,
  QueryReport,
  ConfigReport,
  UpdateReport,
  ListReport,
  TablesReport,
};
use wca::wtools::Itertools;

pub mod model;
use model::{ FeedRow, FrameRow };

/// Storage for feed frames.
pub struct FeedStorage< S : GStore + GStoreMut + Send >
{
  /// GlueSQL storage.
  pub storage : Arc< Mutex< Glue< S > > >,
  frame_fields : Vec< [ &'static str; 3 ] >,
}

impl FeedStorage< SledStorage >
{
  /// Initialize new storage from configuration, create feed table.
  pub async fn init_storage( config : Config ) -> Result< Self, Box< dyn std::error::Error + Send + Sync > >
  {
    let storage = SledStorage::try_from( config )?;
    let mut glue = Glue::new( storage );

    let sub_table = table( "config" )
    .create_table_if_not_exists()
    .add_column( "path TEXT PRIMARY KEY" )
    .build()?
    ;

    sub_table.execute( &mut glue ).await?;

    let feed_table = table( "feed" )
    .create_table_if_not_exists()
    .add_column( "id TEXT PRIMARY KEY" )
    .add_column( "type TEXT" )
    .add_column( "title TEXT" )
    .add_column( "link TEXT UNIQUE" )
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
      [ "feed_id", "TEXT", "Id of feed that contains this frame." ],
    ];
    let mut table = table( "frame" ).create_table_if_not_exists().add_column( "id TEXT PRIMARY KEY" );

    for column in frame_fields.iter().skip( 1 ).take( frame_fields.len() - 2 )
    {
      table = table.add_column( format!( "{} {}", column[ 0 ], column[ 1 ] ).as_str() );
    }

    let table = table.add_column( "feed_id TEXT FOREIGN KEY REFERENCES Feeds(id)" )
    .build()?
    ;

    table.execute( &mut glue ).await?;

    Ok( Self{ storage : Arc::new( Mutex::new( glue ) ), frame_fields } )
  }
}

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait( ?Send ) ]
pub trait FeedStore
{
  /// Insert items from list into feed table.
  async fn save_frames( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >;

  /// Insert items from list into feed table.
  async fn save_feed( &mut self, feed : Vec< ( Feed, Duration ) > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;

  /// Update items from list in feed table.
  async fn update_feed( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;

  /// Process fetched feed, new items will be saved, modified items will be updated.
  async fn process_feeds( &mut self, feeds : Vec< ( Feed, Duration ) > ) -> Result< UpdateReport, Box< dyn std::error::Error + Send + Sync > >;

  /// Get all feed frames from storage.
  async fn get_all_frames( &mut self ) -> Result< ListReport, Box< dyn std::error::Error + Send + Sync > >;

  /// Get all feeds from storage.
  async fn get_all_feeds( &mut self ) -> Result< FeedsReport, Box< dyn std::error::Error + Send + Sync > >;

  /// Execute custom query passed as String.
  async fn execute_query( &mut self, query : String ) -> Result< QueryReport, Box< dyn std::error::Error + Send + Sync > >;

  /// Get list of column titles of feed table.
  fn columns_titles( &mut self ) -> FieldsReport;

  /// Add subscription.
  async fn add_config( &mut self, config : String ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >;

  /// Remove subscription.
  async fn remove_subscription( &mut self, link : String ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >;

  /// List subscriptions.
  async fn list_subscriptions( &mut self ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >;

  /// List tables in storage.
  async fn list_tables( &mut self ) -> Result< TablesReport, Box< dyn std::error::Error + Send + Sync > >;

  /// List columns of table.
  async fn list_columns( &mut self, table_name : String ) -> Result< TablesReport, Box< dyn std::error::Error + Send + Sync > >;

  /// Add feeds entries.
  async fn add_feeds( &mut self, feeds : Vec< FeedRow > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;
}

#[ async_trait::async_trait( ?Send ) ]
impl FeedStore for FeedStorage< SledStorage >
{
  fn columns_titles( &mut self ) -> FieldsReport
  {
    FieldsReport
    {
      fields_list : self.frame_fields.clone()
    }
  }

  async fn execute_query( &mut self, query : String ) -> Result< QueryReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( &query ).await?;

    let report = QueryReport { result : payloads };

    Ok( report )
  }

  async fn list_tables( &mut self ) -> Result< TablesReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( "SELECT * FROM GLUE_TABLE_COLUMNS" ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn list_columns( &mut self, table_name : String ) -> Result< TablesReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let glue = &mut *self.storage.lock().await;
    let query_str = format!( "SELECT * FROM GLUE_TABLE_COLUMNS WHERE TABLE_NAME='{}'", table_name );
    let payloads = glue.execute( &query_str ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn get_all_frames( &mut self ) -> Result< ListReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let res = table( "frame" ).select().execute( &mut *self.storage.lock().await ).await?;

    let mut reports = Vec::new();
    let all_frames = match res
    {
      Payload::Select { labels: label_vec, rows: rows_vec } =>
      {
        SelectedEntries
        {
          selected_rows : rows_vec,
          selected_columns : label_vec,
        }
      },
      _ => SelectedEntries::new(),
    };

    let mut feeds_map = HashMap::new();

    for row in all_frames.selected_rows
    {
      let title_val = row.last().unwrap().clone();
      let title = String::from( title_val );
      feeds_map.entry( title )
      .and_modify( | vec : &mut Vec< Vec< Value > > | vec.push( row.clone() ) )
      .or_insert( vec![ row ] )
      ;
    }

    for ( title, frames ) in feeds_map
    {
      let mut report = FramesReport::new( title );
      report.existing_frames = frames.len();
      report.selected_frames = SelectedEntries { selected_rows : frames, selected_columns : all_frames.selected_columns.clone() };
      reports.push( report );
    }

    Ok( ListReport( reports ) )
  }

  async fn get_all_feeds( &mut self ) -> Result< FeedsReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let res = table( "feed" ).select().project( "id, title, link" ).execute( &mut *self.storage.lock().await ).await?;
    let mut report = FeedsReport::new();
    match res
    {
      Payload::Select { labels: label_vec, rows: rows_vec } =>
      {
        report.selected_entries = SelectedEntries
        {
          selected_rows : rows_vec,
          selected_columns : label_vec,
        }
      },
      _ => {},
    }

    Ok( report )
  }

  async fn save_frames( &mut self, frames : Vec< ( Entry, String ) > ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >
  {
    let entries_rows = frames.into_iter().map( | entry | FrameRow::from( entry ).0 ).collect_vec();

    let insert = table( "frame" )
    .insert()
    .columns
    (
      self.frame_fields.iter().map( | field | field[ 0 ] ).join( "," ).as_str()
    )
    .values( entries_rows )
    .execute( &mut *self.storage.lock().await )
    .await?
    ;

    Ok( insert )
  }

  async fn save_feed( &mut self, feed : Vec< ( Feed, Duration ) > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let feeds_rows = feed.into_iter().map( | feed | FeedRow::from( feed ).0 ).collect_vec();

    let _insert = table( "feed" )
    .insert()
    .columns
    (
      "id,
      title,
      link,
      updated,
      authors,
      description,
      published,
      update_period",
    )
    .values( feeds_rows )
    .execute( &mut *self.storage.lock().await )
    .await?
    ;

    Ok( () )
  }

  async fn update_feed( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let entries_rows = feed.into_iter().map( | entry | FrameRow::from( entry ).0 ).collect_vec();

    for entry in entries_rows
    {
      let _update = table( "frame" )
      .update()
      .set( "title", entry[ 1 ].to_owned() )
      .set( "content", entry[ 4 ].to_owned() )
      .set( "links", entry[ 5 ].to_owned() )
      .set( "summary", entry[ 6 ].to_owned() )
      .set( "published", entry[ 8 ].to_owned() )
      .set( "media", entry[ 9 ].to_owned() )
      .filter( col( "id" ).eq( entry[ 0 ].to_owned() ) )
      .execute( &mut *self.storage.lock().await )
      .await?
      ;
    }
    Ok( () )
  }

  async fn process_feeds
  (
    &mut self,
    feeds : Vec< ( Feed, Duration ) >,
  ) -> Result< UpdateReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let new_feed_ids = feeds.iter().filter_map( | feed | feed.0.links.get( 0 ) ).map( | link | format!("'{}'", link.href ) ).join( "," );
    let existing_feeds = table( "feed" )
    .select()
    .filter( format!( "link IN ({})", new_feed_ids ).as_str() )
    .project( "link" )
    .execute( &mut *self.storage.lock().await )
    .await?
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
        .filter_map( | feed | feed.get( "link" ).map( | link | String::from( crate::report::RowValue( link ) ) ))
        .collect_vec()
        ;

        if !existing_feeds.contains( &&feed.0.links[ 0 ].href )
        {
          self.save_feed( vec![ feed.clone() ] ).await?;
          frames_report.new_frames = feed.0.entries.len();
          frames_report.is_new_feed = true;

          new_entries.extend
          (
            feed.0.entries
            .clone()
            .into_iter()
            .zip( std::iter::repeat( feed.0.id.clone() ).take( feed.0.entries.len() ) )
          );
          reports.push( frames_report );
          continue;
        }
      }

      let existing_frames = table( "frame" )
      .select()
      .filter(col( "feed_id" ).eq( text( feed.0.id.clone() ) ) )
      .project( "id, published" )
      .execute( &mut *self.storage.lock().await )
      .await?
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
                modified_entries.push( ( entry.clone(), feed.0.id.clone() ) );
              }
            }
          }
          else
          {
            frames_report.new_frames += 1;
            new_entries.push( ( entry.clone(), feed.0.id.clone() ) );
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
      let _updated_report = self.update_feed( modified_entries ).await?;
    }

    Ok( UpdateReport( reports ) )
  }

  async fn add_config( &mut self, config : String ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >
  {

    let res = table( "config" )
    .insert()
    .columns
    (
      "path",
    )
    .values( vec![ vec![ text( config ) ] ] )
    .execute( &mut *self.storage.lock().await )
    .await?;

    Ok( ConfigReport { result : res } )
  }

  async fn remove_subscription( &mut self, link : String ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let res = table( "config" )
    .delete()
    .filter( col( "link" ).eq( link ) )
    .execute( &mut *self.storage.lock().await )
    .await?;

    Ok( ConfigReport { result : res } )
  }

  async fn list_subscriptions( &mut self ) -> Result< ConfigReport, Box< dyn std::error::Error + Send + Sync > >
  {
    let res = table( "config" ).select().execute( &mut *self.storage.lock().await ).await?;
    Ok( ConfigReport { result : res } )
  }

  async fn add_feeds( &mut self, feed : Vec< FeedRow > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let feeds_rows = feed.into_iter().map( | feed | feed.0 ).collect_vec();

    let _insert = table( "feed" )
    .insert()
    .columns
    (
      "id,
      title,
      link,
      updated,
      authors,
      description,
      published,
      update_period",
    )
    .values( feeds_rows )
    .execute( &mut *self.storage.lock().await )
    .await?
    ;

    Ok( () )
  }
}
