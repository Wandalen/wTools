
// use wtools::vector;
// use wtools::vector::{ left_index, append_vectors_once };
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

fn _implements_basic()
{

  trait Trait1 {}
  fn implementsement_trait1( _ : &impl Trait1 ) -> bool { true }

  impl< T : Sized > Trait1 for &[ T ] {}
  impl< T : Sized, const N : usize > Trait1 for [ T; N ] {}
  impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
  let src : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( TheModule::implements!( src => Trait1 ), true );
  assert_eq!( implementsement_trait1( &src ), true );
  assert_eq!( TheModule::implements!( &[ 1, 2, 3 ] => Trait1 ), true );
  assert_eq!( implementsement_trait1( &[ 1, 2, 3 ] ), true );
  assert_eq!( TheModule::implements!( [ 1, 2, 3 ] => Trait1 ), true );

  impl< T : Sized > Trait1 for Vec< T > {}
  assert_eq!( TheModule::implements!( vec!( 1, 2, 3 ) => Trait1 ), true );

  impl Trait1 for f32 {}
  assert_eq!( TheModule::implements!( 13_f32 => Trait1 ), true );

  assert_eq!( TheModule::implements!( true => Copy ), true );
  assert_eq!( TheModule::implements!( true => Clone ), true );

  let src = true;
  assert_eq!( TheModule::implements!( src => Copy ), true );
  assert_eq!( TheModule::implements!( src => Clone ), true );

  assert_eq!( TheModule::implements!( Box::new( true ) => Copy ), false );
  assert_eq!( TheModule::implements!( Box::new( true ) => Clone ), true );

  let src = Box::new( true );
  assert_eq!( TheModule::implements!( src => Copy ), false );
  assert_eq!( TheModule::implements!( src => Clone ), true );

  let pointer_size = std::mem::size_of::< &u8 >();
  dbg!( &pointer_size );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< &[ u8 ] >() );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< *const [ u8 ] >() );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< Box< [ u8 ] > >() );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< std::rc::Rc< [ u8 ] > >() );
  assert_eq!( 1 * pointer_size, std::mem::size_of::< &[ u8 ; 20 ] >() );

}

//

// trace_macros!( true );
test_suite!
{
  is_slice_basic,
  implements_basic,
}
// trace_macros!( false );