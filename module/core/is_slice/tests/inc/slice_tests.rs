use super :: *;

//

/// Tests that slice references from variables are correctly detected.
#[ test ]
fn slice_detect_from_variable()
{
  let src : &[ i32 ] = &[ 1, 2, 3 ];
  assert!( the_module::is_slice!( src ) );
}

/// Tests that slice references from literals are correctly detected.
#[ test ]
fn slice_detect_from_literal()
{
  assert!( the_module::is_slice!( &[ 1, 2, 3 ][ .. ] ) );
}

/// Tests that array references are correctly distinguished from slices.
#[ test ]
fn array_reference_not_slice()
{
  assert!( !the_module::is_slice!( &[ 1, 2, 3 ] ) );
}

/// Tests that Vec types are not detected as slices.
#[ test ]
fn vec_not_slice()
{
  assert!( !the_module::is_slice!( std::vec!( 1, 2, 3 ) ) );
}

/// Tests that primitive types are not detected as slices.
#[ test ]
fn primitives_not_slice()
{
  assert!( !the_module::is_slice!( 13_f32 ) );
  assert!( !the_module::is_slice!( true ) );
  let src = false;
  assert!( !the_module::is_slice!( src ) );
}

/// Tests that boxed types are not detected as slices.
#[ test ]
fn boxed_not_slice()
{
  assert!( !the_module::is_slice!( Box::new( true ) ) );
  let src = Box::new( true );
  assert!( !the_module::is_slice!( src ) );
}

/// Tests empty slice detection.
#[ test ]
fn empty_slice_detect()
{
  let empty : &[ i32 ] = &[];
  assert!( the_module::is_slice!( empty ) );
  let empty_literal : &[ i32 ] = &[][ .. ];
  assert!( the_module::is_slice!( empty_literal ) );
}

/// Tests that string slices are not detected as slices (out-of-scope per spec).
#[ test ]
fn string_slice_not_slice()
{
  let s = "hello";
  assert!( !the_module::is_slice!( s ) );
  assert!( !the_module::is_slice!( "world" ) );
}

/// Tests byte slice detection (should return true for &[u8]).
#[ test ]
fn byte_slice_detect()
{
  let bytes : &[ u8 ] = &[ 1, 2, 3 ];
  assert!( the_module::is_slice!( bytes ) );
  assert!( the_module::is_slice!( &[ 0u8, 255u8 ][ .. ] ) );
}

/// Tests slices of different numeric types.
#[ test ]
fn numeric_slice_types()
{
  let i32_slice : &[ i32 ] = &[ 1, 2, 3 ];
  assert!( the_module::is_slice!( i32_slice ) );

  let u64_slice : &[ u64 ] = &[ 100, 200 ];
  assert!( the_module::is_slice!( u64_slice ) );

  let f64_slice : &[ f64 ] = &[ 1.0, 2.0 ];
  assert!( the_module::is_slice!( f64_slice ) );
}

/// Tests nested slices (slice of arrays vs slice of slices).
#[ test ]
fn nested_slice_types()
{
  // Slice of arrays - should be true (it's a slice)
  let slice_of_arrays : &[ [ i32; 2 ] ] = &[ [ 1, 2 ], [ 3, 4 ] ];
  assert!( the_module::is_slice!( slice_of_arrays ) );

  // Array of slices is not directly testable as a slice reference
  // but we can test that array reference is not a slice
  let array_of_values : [ i32; 3 ] = [ 1, 2, 3 ];
  assert!( !the_module::is_slice!( &array_of_values ) );
}

/// Tests slices from function returns.
#[ test ]
fn slice_from_function()
{
  fn get_slice() -> &'static [ i32 ]
  {
    &[ 1, 2, 3 ]
  }

  assert!( the_module::is_slice!( get_slice() ) );
}

/// Tests slices from struct fields.
#[ test ]
fn slice_from_struct_field()
{
  struct Container
  {
    data : Vec< i32 >,
  }

  let container = Container { data : vec![ 1, 2, 3 ] };
  let slice_ref : &[ i32 ] = &container.data;
  assert!( the_module::is_slice!( slice_ref ) );
}

/// Tests char slices.
#[ test ]
fn char_slice_detect()
{
  let chars : &[ char ] = &[ 'a', 'b', 'c' ];
  assert!( the_module::is_slice!( chars ) );
}

/// Tests bool slices.
#[ test ]
fn bool_slice_detect()
{
  let bools : &[ bool ] = &[ true, false, true ];
  assert!( the_module::is_slice!( bools ) );
}

