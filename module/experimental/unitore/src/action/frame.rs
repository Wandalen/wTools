//! Frames actions and reports.

use crate :: *;
use sled_adapter ::FeedStorage;
use entity ::
{
  feed ::FeedStore,
  config ::ConfigStore,
  frame ::FrameStore,
};
use gluesql ::prelude :: { Payload, Value, SledStorage };
use feed_config;
use error_tools ::untyped :: { anyhow, Result };

pub use entity ::frame :: { SelectedEntries, FramesReport, ListReport, UpdateReport };

/// List all frames.
///
/// # Errors
/// Returns error if operation fails.
pub async fn frames_list( mut storage: FeedStorage< SledStorage > ) -> Result< impl Report >
{
  storage.frames_list().await
}

/// Update all frames from config files saved in storage.
///
/// # Errors
/// Returns error if operation fails.
pub async fn frames_download
(
  mut storage: FeedStorage< SledStorage >
) -> Result< impl Report >
{
  let payload = storage.config_list().await?;
  let configs = match &payload
  {
  Payload ::Select { labels: _, rows: rows_vec } =>
  {
   rows_vec.iter().filter_map( | val |
   {
  match &val[ 0 ]
  {
   Value ::Str( path ) => Some( path.to_owned() ),
   _ => None,
 }
 } ).collect :: < Vec< _ > >()
 },
  _ => Vec ::new(),
 };

  let mut subscriptions = Vec ::new();
  for config in &configs
  {
  let sub_vec = feed_config ::read( config )?;
  subscriptions.extend( sub_vec );
 }

  if subscriptions.is_empty()
  {
  return Err( anyhow!( format!
  (
   "Failed to download frames.\n Config file(s) {} contain no feed subscriptions!",
   configs.join( ", " )
 ) ) )
 }

  let mut feeds = Vec ::new();
  let client = retriever ::FeedClient;
  for subscription in  subscriptions
  {
  let feed = client.fetch( subscription.link.clone() ).await?;
  feeds.push( ( feed, subscription.update_period, subscription.link ) );
 }
  storage.feeds_process( feeds ).await

}

