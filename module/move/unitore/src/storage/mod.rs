use crate::*;
use std::{ collections::HashMap, sync::Arc, time::Duration };
use error_tools::{ err, for_app::Context, Result };
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
// use crate::report::
// {
  // qqq : don't put report into different file, keep the in the same file where it used
  // aaa: put into separate files with functions that use them
// };
use executor::endpoints::
{
  feeds::FeedsReport,
  query::QueryReport,
  frames::{ UpdateReport, ListReport },
  table::TablesReport,
  list_fields::FieldsReport,
};
use wca::wtools::Itertools;

pub mod model;
use model::{ FeedRow, FrameRow };

/// Storage for feed frames.
#[ derive( Clone ) ]
pub struct FeedStorage< S : GStore + GStoreMut + Send >
{
  /// GlueSQL storage.
  pub storage : Arc< Mutex< Glue< S > > >,
  frame_fields : Vec< [ &'static str; 3 ] >,
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
pub trait FeedStore
{
  /// Insert items from list into feed table.
  async fn save_frames( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< Payload >;

  /// Insert items from list into feed table.
  async fn save_feed( &mut self, feed : Vec< ( Feed, Duration ) > ) -> Result< () >;

  /// Update items from list in feed table.
  async fn update_feed( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< () >;

  /// Process fetched feed, new items will be saved, modified items will be updated.
  async fn process_feeds( &mut self, feeds : Vec< ( Feed, Duration ) > ) -> Result< UpdateReport >;

  /// Get all feed frames from storage.
  async fn get_all_frames( &mut self ) -> Result< ListReport >;

  /// Get all feeds from storage.
  async fn get_all_feeds( &mut self ) -> Result< FeedsReport >;

  /// Execute custom query passed as String.
  async fn execute_query( &mut self, query : String ) -> Result< QueryReport >;

  /// Get list of column titles of feed table.
  fn columns_titles( &mut self ) -> FieldsReport;

  /// Add subscription.
  async fn add_config( &mut self, config : String ) -> Result< Payload >;

  /// Remove subscription.
  async fn delete_config( &mut self, path : String ) -> Result< Payload >;

  /// List subscriptions.
  async fn list_configs( &mut self ) -> Result< Payload >;

  /// List tables in storage.
  async fn list_tables( &mut self ) -> Result< TablesReport >;

  /// List columns of table.
  async fn list_columns( &mut self, table_name : String ) -> Result< TablesReport >;

  /// Add feeds entries.
  async fn add_feeds( &mut self, feeds : Vec< FeedRow > ) -> Result< Payload >;
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

  async fn execute_query( &mut self, query : String ) -> Result< QueryReport >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( &query ).await.context( "Failed to execute query" )?;

    let report = QueryReport ( payloads );

    Ok( report )
  }

  async fn list_tables( &mut self ) -> Result< TablesReport >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( "SELECT * FROM GLUE_TABLE_COLUMNS" ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn list_columns( &mut self, table_name : String ) -> Result< TablesReport >
  {
    let glue = &mut *self.storage.lock().await;
    let query_str = format!( "SELECT * FROM GLUE_TABLE_COLUMNS WHERE TABLE_NAME='{}'", table_name );
    let payloads = glue.execute( &query_str ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn get_all_frames( &mut self ) -> Result< ListReport >
  {
    let res = table( "frame" ).select().execute( &mut *self.storage.lock().await ).await?;

    let mut reports = Vec::new();
    let all_frames = match res
    {
      Payload::Select { labels: label_vec, rows: rows_vec } =>
      {
        crate::executor::endpoints::frames::SelectedEntries
        {
          selected_rows : rows_vec,
          selected_columns : label_vec,
        }
      },
      _ => crate::executor::endpoints::frames::SelectedEntries::new(),
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
      let mut report = crate::executor::endpoints::frames::FramesReport::new( title );
      report.existing_frames = frames.len();
      report.selected_frames = crate::executor::endpoints::frames::SelectedEntries
      {
        selected_rows : frames,
        selected_columns : all_frames.selected_columns.clone(),
      };
      reports.push( report );
    }

    Ok( ListReport( reports ) )
  }

  async fn get_all_feeds( &mut self ) -> Result< FeedsReport >
  {
    let res = table( "feed" ).select().project( "title, link, update_period" ).execute( &mut *self.storage.lock().await ).await?;
    let mut report = FeedsReport::new();
    match res
    {
      Payload::Select { labels: label_vec, rows: rows_vec } =>
      {
        report.selected_entries = crate::executor::endpoints::frames::SelectedEntries
        {
          selected_rows : rows_vec,
          selected_columns : label_vec,
        }
      },
      _ => {},
    }

    Ok( report )
  }

  async fn save_frames( &mut self, frames : Vec< ( Entry, String ) > ) -> Result< Payload >
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
    .await
    .context( "Failed to insert frames" )?
    ;

    Ok( insert )
  }

  async fn save_feed( &mut self, feed : Vec< ( Feed, Duration ) > ) -> Result< () >
  {
    let feeds_rows = feed.into_iter().map( | feed | FeedRow::from( feed ).0 ).collect_vec();

    for entry in feeds_rows
    {
    let _update = table( "feed" )
    .update()
    .set( "title", entry[ 1 ].to_owned() )
    .set( "updated", entry[ 2 ].to_owned() )
    .set( "authors", entry[ 3 ].to_owned() )
    .set( "description", entry[ 4 ].to_owned() )
    .set( "published", entry[ 5 ].to_owned() )
    .filter( col( "link" ).eq( entry[ 0 ].to_owned() ) )
    .execute( &mut *self.storage.lock().await )
    .await
    .context( "Failed to insert feed" )?
    ;
    }

    Ok( () )
  }

  async fn update_feed( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< () >
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
      .await
      .context( "Failed to update frames" )?
      ;
    }
    Ok( () )
  }

  async fn process_feeds
  (
    &mut self,
    feeds : Vec< ( Feed, Duration ) >,
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
      } ).collect::< Vec< _ > >()[ 0 ]
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
      let mut frames_report = crate::executor::endpoints::frames::FramesReport::new( feed.0.title.clone().unwrap().content );
      // check if feed is new
      if let Some( existing_feeds ) = existing_feeds.select()
      {

        let existing_feeds = existing_feeds
        .filter_map( | feed | feed.get( "link" ).map( | link | String::from( crate::storage::model::RowValue( link ) ) ))
        .collect_vec()
        ;

        let link = &feed.0.links.iter().filter_map( | link |
          {
            if let Some( media_type ) = &link.media_type
            {
              if media_type == &String::from( "application/rss+xml" )
              {
                return Some( link.href.clone() );
              }
            }
            None
          } ).collect::< Vec< _ > >()[ 0 ];

        if !existing_feeds.contains( link )
        {
          self.add_feeds( vec![ FeedRow::from( feed.clone() ) ] ).await?;
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

  async fn add_config( &mut self, config : String ) -> Result< Payload >
  {
    let res = table( "config" )
    .insert()
    .columns
    (
      "path",
    )
    .values( vec![ vec![ text( config ) ] ] )
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

  async fn delete_config( &mut self, path : String ) -> Result< Payload >
  {
    let res = table( "config" )
    .delete()
    .filter( col( "path" ).eq( format!( "'{}'", path ) ) )
    .execute( &mut *self.storage.lock().await )
    .await?;

    if res == Payload::Delete( 0 )
    {
      return Err( err!( format!( "Config file with path {} not found in storage", path ) ) )
    }

    Ok( res )
  }

  async fn list_configs( &mut self ) -> Result< Payload >
  {
    let res = table( "config" ).select().execute( &mut *self.storage.lock().await ).await?;
    Ok( res )
  }

  async fn add_feeds( &mut self, feed : Vec< FeedRow > ) -> Result< Payload >
  {
    let feeds_rows = feed.into_iter().map( | feed | feed.0 ).collect_vec();

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
