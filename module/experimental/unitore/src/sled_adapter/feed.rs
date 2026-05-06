//! Feed operation with Sled storage.

use crate :: *;
use core ::time ::Duration;
use error_tools :: { untyped ::Result, untyped ::Context };
use gluesql ::
{
  core ::
  {
  ast_builder :: { col, null, table, text, Execute, timestamp, ExprNode },
  executor ::Payload,
  data ::Value,
  chrono ::SecondsFormat,
 },
  prelude ::SledStorage,
};
use entity ::
{
  feed :: { Feed, FeedStore, FeedsReport },
  frame :: { FrameStore, UpdateReport, SelectedEntries, FramesReport },
};
use sled_adapter ::FeedStorage;
use itertools ::Itertools;

#[ async_trait ::async_trait( ?Send ) ]
impl FeedStore for FeedStorage< SledStorage >
{
  async fn feeds_list( &mut self ) -> Result< FeedsReport >
  {
  let res = table( "feed" )
  .select()
  .project( "title, link, update_period, config_file" )
  .execute( &mut *self.0.lock().await )
  .await?
  ;

  let mut report = FeedsReport ::new();
  if let Payload ::Select { labels: label_vec, rows: rows_vec } = res {
   report.0 = SelectedEntries
   {
    selected_rows: rows_vec,
    selected_columns: label_vec,
  }
  }

  Ok( report )
 }

  async fn feeds_update( &mut self, feed: Vec< Feed > ) -> Result< () >
  {
  for feed in feed
  {
   let _update = table( "feed" )
   .update()
   .filter( col( "link" ).eq( feed.link.to_string() ) )
   .set( "title", feed.title.map_or( null(), text ) )
   .set(
  "updated",
  feed.updated.map_or( null(), | d | timestamp( d.to_rfc3339_opts( SecondsFormat ::Millis, true ) ) ),
 )
   .set( "authors", feed.authors.map_or( null(), text ) )
   .set( "description", feed.description.map_or( null(), text ) )
   .set(
  "published",
  feed.published.map_or( null(), | d | timestamp( d.to_rfc3339_opts( SecondsFormat ::Millis, true ) ) ),
 )
   .execute( &mut *self.0.lock().await )
   .await
   .context( "Failed to insert feed" )?
   ;
 }

  Ok( () )
 }

  async fn feeds_process
  (
  &mut self,
  feeds: Vec< ( feed_rs ::model ::Feed, Duration, url ::Url ) >,
 ) -> Result< UpdateReport >
  {
  let mut new_entries = Vec ::new();
  let mut modified_entries = Vec ::new();
  let mut reports = Vec ::new();

  for feed in &feeds
  {
   let mut frames_report = FramesReport ::new( feed.0.title.clone().unwrap().content );

   let existing_frames = table( "frame" )
   .select()
   .filter( col( "feed_link" ).eq( text( feed.2.to_string() ) ) )
   .project( "id, published" )
   .execute( &mut *self.0.lock().await )
   .await
   .context( "Failed to get existing frames while saving new frames" )?
   ;

   if let Some( rows ) = existing_frames.select()
   {
  let rows = rows.collect :: < Vec< _ > >();
  frames_report.existing_frames = rows.len();
  let existing_entries = rows.iter()
  .map( | r | ( r.get( "id" ).map( | &val | val.clone() ), r.get( "published" ).map( | &val | val.clone() ) ) )
  .filter_map( | ( id, published ) |
   id.map( | id |
  (
   id,
   published.and_then( | date |
  {
   match date
   {
  Value ::Timestamp( date_time ) => Some( date_time ),
  _ => None,
 }
 } )
 )
 )
 )
  .filter_map( | ( id, published ) | match id { Value ::Str( id ) => Some( ( id, published ) ), _ => None } )
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
  modified_entries.push( ( entry.clone(), feed.2.to_string() ).into() );
 }
 }
 }
   else
   {
  frames_report.new_frames += 1;
  new_entries.push( ( entry.clone(), feed.2.to_string() ).into() );
 }
 }
 }
   reports.push( frames_report );
 }

  if !new_entries.is_empty()
  {
   let _saved_report = self.frames_save( new_entries ).await?;
 }
  if !modified_entries.is_empty()
  {
   let _updated_report = self.frames_update( modified_entries ).await?;
 }

  Ok( UpdateReport( reports ) )
 }

  async fn feeds_save( &mut self, feed: Vec< Feed > ) -> Result< Payload >
  {
  let feeds_rows: Vec< Vec< ExprNode< 'static > > > = feed.into_iter().map( Into ::into ).collect_vec();

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
   update_period,
   config_file",
 )
  .values( feeds_rows )
  .execute( &mut *self.0.lock().await )
  .await
  .context( "Failed to insert feeds" )?
  ;

  Ok( insert )
 }
}
