#[ cfg( feature = "use_std" ) ]
use winterval::*;

fn main()
{

  let src = 2..5;
  #[ cfg( feature = "use_std" ) ]
  assert_eq!( src.closed(), ( 2, 4 ) );
  assert_eq!( src.contains( &3 ), true );

  let src = 2..=4;
  #[ cfg( feature = "use_std" ) ]
  assert_eq!( src.closed(), ( 2, 4 ) );
  assert_eq!( src.contains( &3 ), true );
}
