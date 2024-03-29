use feed_rs::parser as feed_parser;
use error_tools::Result;

#[ tokio::test ]
async fn frame() -> Result< () >
{
  let feed = feed_parser::parse( include_str!( "./fixtures/plain_feed.xml" ).as_bytes() )?;

  let frame = unitore::storage::frame::Frame::from( ( feed.entries[ 0 ].clone(), String::new() ) );


  assert!( frame.id == feed.entries[ 0 ].id );
  println!( "{:#?}", feed.entries[ 0 ].media );
  println!( "{:#?}", frame );

  Ok( () )
}
