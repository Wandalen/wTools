//! Frame storing and retrieving functionality.

use crate::*;
use error_tools::untyped::Result;
use gluesql::core::
{
  ast_builder::{ null, text, timestamp, ExprNode }, chrono::{ DateTime, SecondsFormat, Utc }, executor::Payload
};
use action::frame::ListReport;

/// Frame entity.
#[ derive( Debug ) ]
pub struct Frame
{
  /// Frame id.
  pub id : String,
  /// Frame title.
  pub title : Option< String >,
  /// Time at which this item was fetched from source.
  pub stored_time : Option< DateTime< Utc > >,
  /// List of authors of the frame.
  pub authors : Option< Vec< String > >,
  /// The content of the frame in html or plain text.
  pub content : Option< String >,
  /// List of links associated with this item of related Web page and attachments.
  pub links : Option< Vec< String > >,
  /// Short summary, abstract, or excerpt of the frame item.
  pub summary : Option< String >,
  /// A list of categories that the item belongs to.
  pub categories : Option< Vec< String > >,
  /// Time at which this item was first published or updated.
  pub published : Option< DateTime< Utc > >,
  /// Specifies the source feed if the frame was copied from one feed into another feed.
  pub source : Option< String >,
  /// Information about copyrights over the feed.
  pub rights : Option< String >,
  /// List of media oblects, encountered in the frame.
  pub media : Option< Vec< String > >,
  /// The language of the frame.
  pub language : Option< String >,
  /// Link to feed that contains this frame.
  pub feed_link : String,
}

// qqq : not obvious
// aaa : added explanation
/// Convert from feed_rs feed entry and feed link to Frame struct for convenient use and storage.
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

    let links = entry.links
    .iter()
    .map( | link | link.href.clone() )
    .collect::< Vec< _ > >()
    .clone()
    ;

    let categories = entry.categories
    .iter()
    .map( | cat | cat.term.clone() )
    .collect::< Vec< _ > >()
    ;

    let media = entry.media
    .iter()
    .flat_map( | m | m.content.clone() )
    .filter_map( | m | m.url.map( | url | url.to_string() ) )
    .collect::< Vec< _ > >()
    ;

    Frame
    {
      id : entry.id,
      title : entry.title.map( | title | title.content ).clone(),
      stored_time : entry.updated,
      authors: ( !authors.is_empty() ).then( || authors ),
      // qqq : why join?
      // aaa : fixed, saved as list
      content,
      links: ( !links.is_empty() ).then( || links ),
      // qqq : why join?
      // aaa : fixed, saved as list
      summary : entry.summary.map( | c | c.content ).clone(),
      categories: ( !categories.is_empty() ).then( || categories ),
      // qqq : why join?
      // aaa : fixed, saved as list
      published : entry.published.clone(),
      source : entry.source.clone(),
      rights : entry.rights.map( | r | r.content ).clone(),
      media: ( !media.is_empty() ).then( || media ),
      // qqq : why join?
      // aaa : fixed, saved as list
      language : entry.language.clone(),
      feed_link,
    }
  }
}

/// Frames storing and retrieving.
#[ async_trait::async_trait( ?Send ) ]
pub trait FrameStore
{
  /// Save new frames to storage.
  /// New frames will be inserted into `frame` table. 
  async fn frames_save( &mut self, feed : Vec< Frame > ) -> Result< Payload >;

  /// Update existing frames in storage with new changes.
  /// If frames in storage were modified in feed source, they will be changed to match new version. 
  async fn frames_update( &mut self, feed : Vec< Frame > ) -> Result< () >;

  /// Get all feed frames from storage.
  async fn frames_list( &mut self ) -> Result< ListReport >;
}
// qqq : what is update? what update? don't use word update without noun and explanation what deos it mean
// aaa : fixed comments


// qqq : what is it for and why?
// aaa : added explanation

/// Get convenient frame format for using with GlueSQL expression builder.
/// Converts from Frame struct into vec of GlueSQL expression nodes. 
impl From< Frame > for Vec< ExprNode< 'static > >
{
  fn from( entry : Frame ) -> Self
  {
    let title = entry.title
    .map( | title | text( title ) )
    .unwrap_or( null() )
    ;

    let stored_time = entry.stored_time
    .map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) )
    .unwrap_or( null() )
    ;

    let authors = entry.authors
    .map( | authors |
      text
      (
        format!( "[{}]", authors.into_iter().map( | a | format!( "\"{}\"", a ) ).collect::< Vec< _ > >().join( ", " ) )
      )
    )
    .unwrap_or( null() )
    ;

    let content = entry.content
    .map( | content | text( content ) )
    .unwrap_or( null() )
    ;

    let links = entry.links
    .map( | links |
      text
      (
        format!( "[{}]", links.into_iter().map( | link | format!( "\"{}\"", link ) ).collect::< Vec< _ > >().join( ", " ) ) 
      )
    )
    .unwrap_or( null() )
    ;

    let summary = entry.summary
    .map( | summary | text( summary ) )
    .unwrap_or( null() )
    ;

    let categories = entry.categories
    .map( | categories |
      text
      (
        format!
        (
          "[{}]",
          categories.into_iter().map( | category | format!( "\"{}\"", category ) ).collect::< Vec< _ > >().join( ", " ),
        ) 
      )
    )
    .unwrap_or( null() )
    ;

    let published = entry.published
    .map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) )
    .unwrap_or( null() )
    ;

    let source = entry.source.map( | s | text( s ) ).unwrap_or( null() );
    let rights = entry.rights.map( | r | text( r ) ).unwrap_or( null() );
    let media = entry.media
    .map( | media |
      text
      (
        format!( "[{}]", media.into_iter().map( | media | format!( "\"{}\"", media ) ).collect::< Vec< _ > >().join( ", " ) ) 
      )
    )
    .unwrap_or( null() )
    ;

    let language = entry.language.clone().map( text ).unwrap_or( null() );

    vec!
    [
      text( entry.id ),
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
      text( entry.feed_link )
    ]
  }
}

// qqq : RowValue or CellValue?
// aaa : fixed name
/// GlueSQL Value wrapper for display.
#[ derive( Debug ) ]
pub struct CellValue< 'a >( pub &'a gluesql::prelude::Value );

impl std::fmt::Display for CellValue< '_ >
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

impl From< CellValue< '_ > > for String
{
  fn from( value : CellValue< '_ > ) -> Self
  {
    use gluesql::core::data::Value::*;
    match &value.0
    {
      Str( val ) => val.clone(),
      _ => String::new(),
    }
  }
}
