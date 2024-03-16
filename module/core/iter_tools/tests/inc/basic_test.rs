#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use the_module::*;

//

#[ test ]
#[ cfg( feature = "enabled" ) ]
fn basic()
{
  // test.case( "basic" );
  let src = vec![ 1, 2, 3 ];
  let exp = ( vec![ 2, 3, 4 ], vec![ 0, 1, 2 ] );
  let got : ( Vec< _ >, Vec< _ > ) = src.iter().map( | e |
  {(
    e + 1,
    e - 1,
  )}).multiunzip();
  a_id!( got, exp );
}
