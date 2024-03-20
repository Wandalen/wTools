//! Frame storing and retrieving functionality.

use crate::*;
use std::collections::HashMap;
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

use executor::actions::frame::{ FramesReport, ListReport, SelectedEntries };
use storage::FeedStorage;
use wca::wtools::Itertools;

/// Frame entity.
#[ derive( Debug ) ]
pub struct Frame
{
  /// Frame id.
  pub id : String,
  /// Frame title.
  pub title : Option< String >,
  updated : Option< DateTime< Utc > >,
  authors : Option< String >,
  content : Option< String >,
  links : Option< String >,
  summary : Option< String >,
  categories : Option< String >,
  published : Option< DateTime< Utc > >,
  source : Option< String >,
  rights : Option< String >,
  media : Option< String >,
  language : Option< String >,
  feed_link : String,
}

// qqq : not obvious
impl From< ( feed_rs::model::Entry, String ) > for Frame
{
  fn from( ( entry, feed_link ) : ( feed_rs::model::Entry, String ) ) -> Self
  {
    let authors = entry.authors
    .iter()
    .map( | p | p.name.clone() )
    .collect::< Vec< _ > >()
    ;

    let content = entry.content
    .map( | c | c.body.unwrap_or( c.src.map( | link | link.href ).unwrap_or_default() ) )
    .filter( | s | !s.is_empty() )
    .clone()
    ;

    let mut links = entry.links
    .iter()
    .map( | link | link.href.clone() )
    .clone()
    ;

    let categories = entry.categories
    .iter()
    .map( | cat | cat.term.clone() )
    .collect::< Vec< _ > >()
    ;

    let media = entry.media
    .iter()
    .map( | m | m.content.clone() )
    .flatten()
    .filter_map( | m | m.url.map( | url | url.to_string() ) )
    .collect::< Vec< _ > >()
    ;

    Frame
    {
      id : entry.id,
      title : entry.title.map( | title | title.content ).clone(),
      updated : entry.updated.clone(),
      authors : ( !authors.is_empty() ).then( || authors.join( ", " ) ),
      // qqq : why join?
      content,
      links : ( !links.len() == 0 ).then( || links.join( ", " ) ),
      // qqq : why join?
      summary : entry.summary.map( | c | c.content ).clone(),
      categories : ( !categories.is_empty() ).then( || categories.join( ", " ) ),
      // qqq : why join?
      published : entry.published.clone(),
      source : entry.source.clone(),
      rights : entry.rights.map( | r | r.content ).clone(),
      media : ( !media.is_empty() ).then( || media.join( ", " ) ),
      // qqq : why join?
      language : entry.language.clone(),
      feed_link,
    }
  }
}

/// Frames storing and retrieving.
#[ async_trait::async_trait( ?Send ) ]
pub trait FrameStore
{
  /// Insert items from list into feed table.
  async fn save_frames( &mut self, feed : Vec< Frame > ) -> Result< Payload >;

  /// Update items from list in feed table.
  async fn update_frames( &mut self, feed : Vec< Frame > ) -> Result< () >;

  /// Get all feed frames from storage.
  async fn list_frames( &mut self ) -> Result< ListReport >;
}
// qqq : what is update? what update? don't use word update without noun and explanation what deos it mean

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
      report.selected_frames = SelectedEntries
      {
        selected_rows : frames,
        selected_columns : all_frames.selected_columns.clone(),
      };
      reports.push( report );
    }

    Ok( ListReport( reports ) )
  }

  async fn save_frames( &mut self, frames : Vec< Frame > ) -> Result< Payload >
  {
    let entries_rows : Vec< Vec< ExprNode< 'static > > > = frames.into_iter().map( | entry | entry.into() ).collect_vec();

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

  async fn update_frames( &mut self, feed : Vec< Frame > ) -> Result< () >
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
      .execute( &mut *self.storage.lock().await )
      .await
      .context( "Failed to update frames" )?
      ;
    }
    Ok( () )
  }
}

// qqq : what is it for and why?
impl From< Frame > for Vec< ExprNode< 'static > >
{
  fn from( entry : Frame ) -> Self
  {
    let title = entry.title
    .map( | title | text( title ) )
    .unwrap_or( null() )
    ;

    let updated = entry.updated
    .map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) )
    .unwrap_or( null() )
    ;

    let authors = entry.authors
    .map( | authors | text( authors ) )
    .unwrap_or( null() )
    ;

    let content = entry.content
    .map( | content | text ( content ) )
    .unwrap_or( null() )
    ;

    let links = entry.links
    .map( | links | text ( links ) )
    .unwrap_or( null() )
    ;

    let summary = entry.summary
    .map( | summary | text ( summary ) )
    .unwrap_or( null() )
    ;

    let categories = entry.categories
    .map( | categories | text ( categories ) )
    .unwrap_or( null() )
    ;

    let published = entry.published
    .map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) )
    .unwrap_or( null() )
    ;

    let source = entry.source.map( | s | text( s ) ).unwrap_or( null() );
    let rights = entry.rights.map( | r | text( r ) ).unwrap_or( null() );
    let media = entry.media
    .map( | media | text ( media ) )
    .unwrap_or( null() )
    ;

    let language = entry.language.clone().map( | l | text( l ) ).unwrap_or( null() );

    vec!
    [
      text( entry.id ),
      title,
      updated,
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
      text( entry.feed_link )
    ]
  }
}

// qqq : RowValue or CellValue?
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
