//!
use unitore::client::FeedClient;

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error + Send + Sync > > 
{
  let client = FeedClient;
  let _f = client.fetch( String::from( "https://feeds.bbci.co.uk/news/world/rss.xml" ) ).await?;
  Ok( () )
}