use async_trait::async_trait;
use feed_rs::parser as feed_parser;
use unitore::
{
  feed_config::SubscriptionConfig,
  retriever::FeedFetch,
  storage::{ FeedStorage, MockStore, frame::FrameStore, feed::FeedStore },
};
use error_tools::Result;

/// Feed client for testing.
#[derive(Debug)]
pub struct TestClient;

#[ async_trait ]
impl FeedFetch for TestClient
{
  async fn fetch( &self, _ : url::Url ) -> Result< feed_rs::model::Feed >
  {
    let feed = feed_parser::parse( include_str!( "./fixtures/plain_feed.xml" ).as_bytes() )?;

    Ok( feed )
  }
}

#[ tokio::test ]
async fn test_save_feed_plain() -> Result< () >
{
  // let mut f_store = MockFeedStore::new();
  // f_store
  // .expect_process_feeds()
  // .times( 1 )
  // .returning( | _ | Ok( UpdateReport(
  //   vec! [ FramesReport
  //   {
  //     new_frames : 2,
  //     updated_frames : 0,
  //     selected_frames : SelectedEntries::new(),
  //     existing_frames : 0,
  //     feed_link : String::new(),
  //     is_new_feed : false,
  //   } ] ) ) )
  // ;

  let config = gluesql::sled_storage::sled::Config::default()
  .path( "./test".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( config ).await?;

  let feed_config = SubscriptionConfig
  {
    update_period : std::time::Duration::from_secs( 1000 ),
    link : url::Url::parse( "https://www.nasa.gov/feed/" )?,
  };

  let mut feeds = Vec::new();
  let client = TestClient;

  let feed = FeedFetch::fetch( &client, feed_config.link.clone()).await?;
  feeds.push( ( feed, feed_config.update_period.clone(), feed_config.link.clone() ) );
  feed_storage.process_feeds( feeds ).await?;

  let entries = feed_storage.list_frames().await?;

  let number_of_frames = entries.0[ 0 ].selected_frames.selected_rows.len();

  assert_eq!( number_of_frames, 10 );

  Ok( () )
}
