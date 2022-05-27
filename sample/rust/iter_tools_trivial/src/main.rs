#[ cfg( feature = "itertools" ) ]
use iter_tools::*;

fn main()
{
  #[ cfg( feature = "itertools" ) ]
  {
    /* standard functions */
    let vec = vec![ 5, 1, -2 ];
    let min = min( &vec );
    assert_eq!( *min.unwrap(), -2 );

    /* non standard functions */
    let vec = vec![ 5, 1, -2 ];
    let added = vec![ "a", "b", "c" ];
    let mut result = vec![];
    let zipped = zip( &vec, &added );
    for ( left, right ) in zipped
    {
      result.push( ( *left, *right ) );
    }
    assert_eq!( result, vec![ ( 5, "a" ), ( 1, "b" ), ( -2, "c" ) ] );
  }
}
