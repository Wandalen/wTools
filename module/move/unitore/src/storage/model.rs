use std::time::Duration;

use feed_rs::model::Feed;
use gluesql::core::
{
  ast_builder::{ null, text, timestamp, ExprNode },
  chrono::SecondsFormat,
};

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
      value.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( value.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) ),
      value.description.clone().map( | desc | text( desc.content ) ).unwrap_or( null() ),
      value.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ),
      text( duration.as_secs().to_string() ),
    ];
    FeedRow( row )
  }
}

