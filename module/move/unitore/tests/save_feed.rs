use async_trait::async_trait;
use feed_rs::parser as feed_parser;
use unitore::{
  executor::FeedManager, 
  report::{ SelectedEntries, FramesReport },
  feed_config::SubscriptionConfig,
  retriever::FeedFetch,
  storage::MockFeedStore,
};

pub struct TestClient;

#[ async_trait ]
impl FeedFetch for TestClient
{
  async fn fetch( &self, _ : String ) -> Result< feed_rs::model::Feed, Box< dyn std::error::Error + Send + Sync > >
  {
    let feed = feed_parser::parse( include_str!( "./fixtures/plain_feed.xml" ).as_bytes() )?;

    Ok( feed )
  }
}

#[ tokio::test ]
async fn test_save_feed_plain() -> Result< (), Box< dyn std::error::Error + Sync + Send > >
{
  let mut f_store = MockFeedStore::new();
  f_store
  .expect_process_feeds()
  .times( 1 )
  .returning( | _ | Ok( FramesReport { new_frames : 2, updated_frames : 0, selected_frames : SelectedEntries::new() } ) )
  ;

  let feed_config = SubscriptionConfig
  {
    period : std::time::Duration::from_secs( 1000 ),
    link : String::from( "test" ),
  };

  let mut manager = FeedManager
  {
    storage : f_store,
    client : TestClient,
    config : vec![ feed_config ],
  };
  manager.update_feed().await?;

  Ok( () )
}
