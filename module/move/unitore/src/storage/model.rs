use feed_rs::model::{ Entry, Feed };
use gluesql::core::
{
  ast_builder::{ null, text, timestamp, ExprNode },
  chrono::{ SecondsFormat, Utc },
};
use crate::storage::SubscriptionConfig;

pub struct FeedRow( pub Vec< ExprNode< 'static > > );

impl From< Feed > for FeedRow
{
  fn from( value : Feed ) -> Self
  {
    let mut row = Vec::new();
    row.push( text( value.id.clone() ) );
    row.push( value.title.clone().map( | title | text( title.content ) ).unwrap_or( null() ) );
    row.push( value.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) );
    row.push( text( value.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) ).to_owned() );
    row.push( value.description.clone().map( | desc | text( desc.content ) ).unwrap_or( null() ) );
    row.push( value.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) );

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

pub struct SubscriptionRow( pub Vec< ExprNode< 'static > > );

impl From< SubscriptionConfig > for SubscriptionRow
{
  fn from( value : SubscriptionConfig ) -> Self
  {
    let row = SubscriptionRow( vec!
    [
      text( value.link ),
      text( value.period.as_secs().to_string() ),
      timestamp( Utc::now().to_rfc3339_opts( SecondsFormat::Millis, true ) )
    ] );

    row
  }
}
