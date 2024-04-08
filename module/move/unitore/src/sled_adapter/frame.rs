// Frames operation with Sled storage.

use crate::*;
use std::collections::HashMap;
use error_tools::{ Result, for_app::Context };
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
use storage::FeedStorage;
use wca::wtools::Itertools;

#[ async_trait::async_trait( ?Send ) ]
impl FrameStore for FeedStorage< SledStorage >
{
  async fn frames_list( &mut self ) -> Result< ListReport >
  {
    let res = table( "frame" ).select().execute( &mut *self.storage.lock().await ).await?;

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

    // let glue = &mut *self.storage.lock().await;

  //     /// Frame id.
  // pub id : String,
  // /// Frame title.
  // pub title : Option< String >,
  // stored_time : Option< DateTime< Utc > >,
  // authors : Option< Vec< String > >,
  // content : Option< String >,
  // links : Option< Vec< String > >,
  // summary : Option< String >,
  // categories : Option< Vec< String > >,
  // published : Option< DateTime< Utc > >,
  // source : Option< String >,
  // rights : Option< String >,
  // media : Option< Vec< String > >,
  // language : Option< String >,
  // feed_link : String,

  // use gluesql::core::ast_builder::text;
  //   let mut values_str = String::new();
  //   let null = "NULL".to_string();
  //   let values_str = frames.into_iter().map(|frame| format!(
  //     "('{}', {}, '{}', {}, {}, {}, '{}', {}, '{}')", 
  //     frame.id, 
  //     frame.title.map(|t|format!("'{}'", t)).unwrap_or( "Null".to_string() ), 
  //     frame.stored_time.map(|d|d.to_string()).unwrap_or("Null".to_string()),
  //     frame.authors.map(|authors| {let authors = authors.into_iter().map(|a|format!("'[\"{}\"]'", a)).collect::<Vec<_>>(); authors.join(", ")}).unwrap_or("'[]'".to_string()),
  //     null.clone(),
  //     frame.links.map(|links| {let links = links.into_iter().map(|a|format!("\"{}\"", a)).collect::<Vec<_>>(); format!("'[{}]'", &links.join(", "))}).unwrap_or("'[]'".to_string()),
  //     frame.summary.unwrap_or(null.clone()),
  //     frame.categories.map(|categories| {let categories = categories.into_iter().map(|a|format!("{}", a)).collect::<Vec<_>>(); dbg!(format!("'[\"{}\"]'", &categories.join(", ")))}).unwrap_or(null.clone()),
  //     frame.published.map(|d|d.to_string()).unwrap_or(null.clone()),
      // frame.source.unwrap_or(null.clone()),
      // frame.rights.unwrap_or(null.clone()),
      // // frame.media.map(|media| {let media = media.into_iter().map(|a|format!("\"{}\"", a)).collect::<Vec<_>>(); media.join(", ")}).unwrap_or(null.clone()),
      // frame.language.unwrap_or(null.clone()),
      // frame.feed_link,
    // )
    // )
    //   .collect::<Vec<_>>();

    // for frame in frames
    // {
    //   let frame_str = format!(
    //     "({}, {}, {})", 
    //     frame.id, frame.title.unwrap_or( "NULL".to_string() ), frame.stored_time.map(|d|d.to_string()).unwrap_or("NULL".to_string()));
    //   values_str.push_str(&format!("({}),", frame_str ));
    // }
    // let query_str = format!( "INSERT INTO frame(id, title, stored_time, authors, content, links, summary, categories, published) VALUES {};", values_str.join(", ") );
    //println!("{}", query_str);
    // let mut payloads = glue.execute( &query_str ).await?;

    // INSERT INTO ListType VALUES
    // (1, '[1, 2, 3]'),
    // (2, '["hello", "world", 30, true, [9,8]]'),
    // (3, '[{ "foo": 100, "bar": [true, 0, [10.5, false] ] }, 10, 20]');

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

  async fn frames_update( &mut self, feed : Vec< Frame > ) -> Result< () >
  {
    //let entries_rows : Vec< Vec< ExprNode< 'static > > > = Vec::new();
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
