use is_slice::*;

fn main()
{

  dbg!( is_slice!( &[ 1, 2, 3 ] ) );
  dbg!( is_slice!( Box::new( true ) ) );

}
