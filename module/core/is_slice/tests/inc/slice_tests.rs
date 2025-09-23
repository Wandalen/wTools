use super :: *;

//

#[ test ]
fn is_slice_basic() 
{
  let src: &[ i32] = &[ 1, 2, 3];
  assert!(the_module ::is_slice!(src));
  assert!(the_module ::is_slice!(&[ 1, 2, 3][..]));
  assert!(!the_module ::is_slice!(&[ 1, 2, 3]));

  // the_module ::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // the_module ::inspect_type_of!( &[ 1, 2, 3 ] );

  assert!(!the_module ::is_slice!(std ::vec!(1, 2, 3)));
  assert!(!the_module ::is_slice!(13_f32));
  assert!(!the_module ::is_slice!(true));
  let src = false;
  assert!(!the_module ::is_slice!(src));
  assert!(!the_module ::is_slice!(Box ::new(true)));
  let src = Box ::new(true);
  assert!(!the_module ::is_slice!(src));
}
