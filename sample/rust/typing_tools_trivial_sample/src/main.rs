use typing_tools::*;

fn main()
{
  let src = Box::new( true );
  assert_eq!( implements!( src => Copy ), false );
  assert_eq!( implements!( src => Clone ), true );
}
