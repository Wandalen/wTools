//! qqq : write proper description
use typing_tools::*;

fn main()
{
  let src = Box::new( true );
  assert!( !implements!( src => Copy ) );
  assert!( implements!( src => Clone ) );
}
