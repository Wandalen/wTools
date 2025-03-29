use feed_rs::parser as feed_parser;
use error_tools::untyped::Result;

#[ tokio::test ]
async fn frame() -> Result< () >
{
  let feed = feed_parser::parse( include_str!( "./fixtures/plain_feed.xml" ).as_bytes() )?;
  let frame = unitore::entity::frame::Frame::from( ( feed.entries[ 0 ].clone(), String::new() ) );

  assert!( frame.id == feed.entries[ 0 ].id );

  Ok( () )
}
