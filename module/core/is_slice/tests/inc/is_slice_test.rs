use super::*;

//

tests_impls!
{
  #[ test ]
  fn is_slice_basic()
  {
    let src : &[ i32 ] = &[ 1, 2, 3 ];
    a_id!( the_module::is_slice!( src ), true );
    a_id!( the_module::is_slice!( &[ 1, 2, 3 ][ .. ] ), true );
    a_id!( the_module::is_slice!( &[ 1, 2, 3 ] ), false );

    // the_module::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
    // the_module::inspect_type_of!( &[ 1, 2, 3 ] );

    a_id!( the_module::is_slice!( vec!( 1, 2, 3 ) ), false );
    a_id!( the_module::is_slice!( 13_f32 ), false );
    a_id!( the_module::is_slice!( true ), false );
    let src = false;
    a_id!( the_module::is_slice!( src ), false );
    a_id!( the_module::is_slice!( Box::new( true ) ), false );
    let src = Box::new( true );
    a_id!( the_module::is_slice!( src ), false );
  }
}

//

tests_index!
{
  is_slice_basic,
}
