
use wtools as TheModule;
use wtools::test_suite;

//

fn _is_slice_basic()
{

  let src : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( TheModule::is_slice!( src ), true );
  assert_eq!( TheModule::is_slice!( vec!( 1, 2, 3 ) ), false );
  assert_eq!( TheModule::is_slice!( 13_f32 ), false );
  assert_eq!( TheModule::is_slice!( true ), false );
  let src = false;
  assert_eq!( TheModule::is_slice!( src ), false );
  assert_eq!( TheModule::is_slice!( Box::new( true ) ), false );
  let src = Box::new( true );
  assert_eq!( TheModule::is_slice!( src ), false );

}

//

// trace_macros!( true );
test_suite!
{
  is_slice_basic,
}
// trace_macros!( false );