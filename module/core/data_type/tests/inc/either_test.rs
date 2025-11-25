#[ allow( unused_imports ) ]
use super :: *;

#[ test ]
fn basic_test()
{
  let left : the_module ::Either< _, () > = the_module ::Either ::Left( 13 );
  assert_eq!( left.flip(), the_module ::Either ::Right( 13 ) );
}
