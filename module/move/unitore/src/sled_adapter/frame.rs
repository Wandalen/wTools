//! Frames operation with Sled storage.

use crate::*;
use std::collections::HashMap;
use error_tools::{ untyped::Result, untyped::Context };
use gluesql::
{
  core::
  {
    ast_builder::{ col, table, Execute, ExprNode },
    executor::Payload,
    data::Value,
  },
  sled_storage::SledStorage,
};
use entity::frame::{ FrameStore, Frame };
use action::frame::{ SelectedEntries, FramesReport, ListReport };
use sled_adapter::FeedStorage;
use wca::iter_tools::Itertools;

#[ async_trait::async_trait( ?Send ) ]
impl FrameStore for FeedStorage< SledStorage >
{
  async fn frames_list( &mut self ) -> Result< ListReport >
  {
    let res = table( "frame" ).select().execute( &mut *self.0.lock().await ).await?;

    let mut reports = Vec::new();
    let all_frames =
    if let Payload::Select { labels: label_vec, rows: rows_vec } = res
    {
      SelectedEntries
      {
        selected_rows : rows_vec,
        selected_columns : label_vec,
      }
    }
    else
    {
      SelectedEntries::new()
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
      report.selected_frames = SelectedEntries
      {
        selected_rows : frames,
        selected_columns : all_frames.selected_columns.clone(),
      };
      reports.push( report );
    }

    Ok( ListReport( reports ) )
  }

  async fn frames_save( &mut self, frames : Vec< Frame > ) -> Result< Payload >
  {
    let entries_rows : Vec< Vec< ExprNode< 'static > > > = frames.into_iter().map( | entry | entry.into() ).collect_vec();

    let insert = table( "frame" )
    .insert()
    .columns
    (
      "id,
      title,
      stored_time,
      authors,
      content,
      links,
      summary,
      categories,
      published,
      source,
      rights,
      media,
      language,
      feed_link"
    )
    .values( entries_rows )
    .execute( &mut *self.0.lock().await )
    .await
    .context( "Failed to insert frames" )?
    ;

    Ok( insert )
  }

  async fn frames_update( &mut self, feed : Vec< Frame > ) -> Result< () >
  {
    let entries_rows : Vec< Vec< ExprNode< 'static > > > = feed.into_iter().map( | entry | entry.into() ).collect_vec();

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
      .execute( &mut *self.0.lock().await )
      .await
      .context( "Failed to update frames" )?
      ;
    }
    Ok( () )
  }
}
