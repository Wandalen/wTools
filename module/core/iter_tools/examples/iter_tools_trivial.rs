//! qqq : write proper description

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use iter_tools::*;

  /* standard functions */
  let vec = vec![ 5, 1, -2 ];
  let min = min( &vec );
  assert_eq!( *min.unwrap(), -2 );

  /* non standard functions */
  let vec = vec![ 5, 1, -2 ];
  let mut result = vec![];
  let reversed = rev( &vec );
  for v in reversed
  {
    result.push( *v );
  }
  assert_eq!( result, vec![ -2, 1, 5, ] );

}
