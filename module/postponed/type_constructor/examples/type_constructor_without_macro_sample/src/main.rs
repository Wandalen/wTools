
fn main()
{

  let i32_in_tuple = type_constructor::Single::< i32 >::from( 13 );
  dbg!( i32_in_tuple );
  // i32_in_tuple = Single( 13 )
  let i32_and_f32_in_tuple = type_constructor::Pair::< i32, f32 >::from( ( 13, 13.0 ) );
  dbg!( i32_and_f32_in_tuple );
  // vec_of_i32_in_tuple = Pair( 13, 13.0 )
  let two_i32_in_tuple = type_constructor::HomoPair::< i32 >::from( ( 13, 31 ) );
  dbg!( two_i32_in_tuple );
  // vec_of_i32_in_tuple = HomoPair( 13, 31 )
  #[ cfg( all( feature = "many", feature = "use_std" ) ) ]
  {
    let vec_of_i32_in_tuple = type_constructor::Many::< i32 >::from( [ 1, 2, 3 ] );
    dbg!( vec_of_i32_in_tuple );
    // vec_of_i32_in_tuple = Many([ 1, 2, 3 ])
  }

}
