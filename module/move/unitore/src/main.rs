//!
use unitore::client::FeedClient;
use unitore::feed_config::read_feed_config;

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error + Send + Sync > > 
{
  let client = FeedClient;
  //let _f = client.fetch( String::from( "https://feeds.bbci.co.uk/news/world/rss.xml" ) ).await?;

  let feed_configs = read_feed_config().unwrap();

  for config in feed_configs
  {
    client.fetch( config.link ).await?;
  }
  Ok( () )
}