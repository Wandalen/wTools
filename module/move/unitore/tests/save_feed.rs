use feed_rs::parser as feed_parser;
use unitore::
{
  feed_config::SubscriptionConfig,
  storage::FeedStorage,
  entity::{ frame::FrameStore, feed::FeedStore },
};
use error_tools::Result;

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

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;

  let feed_config = SubscriptionConfig
  {
    update_period : std::time::Duration::from_secs( 1000 ),
    link : url::Url::parse( "https://www.nasa.gov/feed/" )?,
  };

  let mut feeds = Vec::new();

  let feed = feed_parser::parse( include_str!("./fixtures/plain_feed.xml").as_bytes() )?;
  feeds.push( ( feed, feed_config.update_period.clone(), feed_config.link.clone() ) );
  feed_storage.feeds_process( feeds ).await?;

  let entries = feed_storage.frames_list().await?;

  let number_of_frames = entries.0[ 0 ].selected_frames.selected_rows.len();

  println!("{:#?}", entries);

  assert_eq!( number_of_frames, 10 );

  Ok( () )
}
