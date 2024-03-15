use crate::*;
use std::collections::HashMap;
use error_tools::{ for_app::Context, Result };
use feed_rs::model::Entry;
use gluesql::
{
  core::
  {
    ast_builder::{ col, table, text, Execute },
    data::Value,
    executor::Payload,
  },
  sled_storage::SledStorage,
};

use gluesql::core::
{
  ast_builder::{ null, timestamp, ExprNode },
  chrono::SecondsFormat,
};

use executor::endpoints::frames::ListReport;
use wca::wtools::Itertools;

use super::FeedStorage;

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait( ?Send ) ]
pub trait FrameStore
{
  /// Insert items from list into feed table.
  async fn save_frames( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< Payload >;

  /// Update items from list in feed table.
  async fn update_feed( &mut self, feed : Vec< ( Entry, String ) > ) -> Result< () >;

  /// Get all feed frames from storage.
  async fn list_frames( &mut self ) -> Result< ListReport >;

}

#[ async_trait::async_trait( ?Send ) ]
impl FrameStore for FeedStorage< SledStorage >
{
  async fn list_frames( &mut self ) -> Result< ListReport >
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

}

/// Frame row format for saving in storage.
#[ derive( Debug ) ]
pub struct FrameRow( pub Vec< ExprNode< 'static > > );

/// Create row for QlueSQL storage from Feed Entry type.
impl From< ( Entry, String ) > for FrameRow
{
  fn from( entry : ( Entry, String ) ) -> Self
  {
    let feed_id = text( entry.1.clone() );
    let entry = &entry.0;

    let id = text( entry.id.clone() );
    let title = entry.title.clone().map( | title | text( title.content ) ).unwrap_or( null() );
    let updated = entry.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() );
    let authors = text( entry.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) ).to_owned();
    let content = entry.content
    .clone()
    .map( | c | text( c.body.unwrap_or( c.src.map( | link | link.href ).unwrap_or_default() ) ) ).unwrap_or( null() ) 
    ;
    let links = if entry.links.len() != 0
    {
      text
      (
        entry.links
        .clone()
        .iter()
        .map( | link | link.href.clone() )
        .fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    }
    else 
    {
      null()
    };
    let summary = entry.summary.clone().map( | c | text( c.content ) ).unwrap_or( null() );
    let categories = if entry.categories.len() != 0
    {
      text
      (
        entry.categories
        .clone()
        .iter()
        .map( | cat | cat.term.clone() )
        .fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    }
    else
    {
      null()
    };
    let published = entry.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() );
    let source = entry.source.clone().map( | s | text( s ) ).unwrap_or( null() );
    let rights = entry.rights.clone().map( | r | text( r.content ) ).unwrap_or( null() );
    let media = if entry.media.len() != 0
    {
      text
      (
        entry.media
        .clone()
        .iter()
        .map( | m | m.title.clone().map( | t | t.content ).unwrap_or_default() )
        .fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    }
    else 
    {
      null()
    };
    let language = entry.language.clone().map( | l | text( l ) ).unwrap_or( null() );

    FrameRow( vec![ id, title, updated, authors, content,links, summary, categories, published, source, rights, media, language, feed_id ] )
  }
}

/// GlueSQL Value wrapper for display.
#[ derive( Debug ) ]
pub struct RowValue< 'a >( pub &'a gluesql::prelude::Value );

impl std::fmt::Display for RowValue< '_ >
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    use gluesql::prelude::Value::*;
    match &self.0
    {
      Bool( val ) => write!( f, "{}", val )?,
      I8( val ) => write!( f, "{}", val )?,
      I16( val ) => write!( f, "{}", val )?,
      I32( val ) => write!( f, "{}", val )?,
      I64( val ) => write!( f, "{}", val )?,
      I128( val ) => write!( f, "{}", val )?,
      U8( val ) => write!( f, "{}", val )?,
      U16( val ) => write!( f, "{}", val )?,
      U32( val ) => write!( f, "{}", val )?,
      U64( val ) => write!( f, "{}", val )?,
      U128( val ) => write!( f, "{}", val )?,
      F32( val ) => write!( f, "{}", val )?,
      F64( val ) => write!( f, "{}", val )?,
      Str( val ) => write!( f, "{}", val )?,
      Null => write!( f, "Null" )?,
      Timestamp( val ) => write!( f, "{}", val )?,
      _ => write!( f, "" )?,
    }

    Ok( () )
  }
}

impl From< RowValue< '_ > > for String
{
  fn from( value : RowValue< '_ > ) -> Self
  {
    use gluesql::core::data::Value::*;
    match &value.0
    {
      Str( val ) => val.clone(),
      _ => String::new(),
    }
  }
}

