use super::*;

//

#[test]
fn is_slice_basic() {
  let src: &[i32] = &[1, 2, 3];
  assert!(the_module::is_slice!(src));
  assert!(the_module::is_slice!(&[1, 2, 3][..]));
  assert_eq!(the_module::is_slice!(&[1, 2, 3]), false);

  // the_module::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // the_module::inspect_type_of!( &[ 1, 2, 3 ] );

  assert_eq!(the_module::is_slice!(vec!(1, 2, 3)), false);
  assert_eq!(the_module::is_slice!(13_f32), false);
  assert_eq!(the_module::is_slice!(true), false);
  let src = false;
  assert_eq!(the_module::is_slice!(src), false);
  assert_eq!(the_module::is_slice!(Box::new(true)), false);
  let src = Box::new(true);
  assert_eq!(the_module::is_slice!(src), false);
}
