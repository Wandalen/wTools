use std::time::Duration;

use feed_rs::model::{ Entry, Feed };
use gluesql::core::
{
  ast_builder::{ function::generate_uuid, null, text, timestamp, ExprNode },
  chrono::SecondsFormat,
};

pub struct FeedRow( pub Vec< ExprNode< 'static > > );

impl FeedRow
{
  pub fn new( feed_link : String, update_period : Duration ) -> Self
  {
    FeedRow( vec!
    [
      text( feed_link ),
      null(),
      // text( feed_link ),
      null(),
      null(),
      null(),
      null(),
      text( update_period.as_secs().to_string() ),
    ] )
  }
}

impl From< ( Feed, Duration ) > for FeedRow
{
  fn from( value : ( Feed, Duration ) ) -> Self
  {
    let duration = value.1;
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
        } ).collect::< Vec< _ > >()[ 0 ]
        .clone(),
      value.title.clone().map( | title | text( title.content ) ).unwrap_or( null() ),
      // value.links.get( 0 ).map( | link | text( link.href.clone() ) ).unwrap_or( null() ),
      value.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( value.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) ),
      value.description.clone().map( | desc | text( desc.content ) ).unwrap_or( null() ),
      value.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( duration.as_secs().to_string() ),
    ];
    FeedRow( row )
  }
}

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
