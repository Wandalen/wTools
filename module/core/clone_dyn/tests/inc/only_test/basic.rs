
// ## Test Matrix for `only_test/basic.rs`
//
// This file contains basic tests for `clone_into_box` and `clone` functions.
//
// | ID | Description | Target Crate(s) | Test File(s) | Key Logic | Feature Combination | Expected Outcome |
// |---|---|---|---|---|---|---|
// | T1.1 | Verify `clone_into_box` for copyable types (`i32`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` | Pass |
// | T1.2 | Verify `clone_into_box` for clonable types (`String`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` | Pass |
// | T1.3 | Verify `clone_into_box` for slice types (`&str`, `&[i32]`). | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone_into_box` | `clone_dyn_types` | Pass |
// | T2.1 | Verify `clone()` helper for various types. | `clone_dyn`, `clone_dyn_types` | `only_test/basic.rs` | `clone` | `clone_dyn_types` | Pass |

#[ test ]
fn clone_into_box()
{

  // copyable

  let a : i32 = 13;
  let _b : Box<  i32  > = the_module::clone_into_box( &a );
  a_id!( a, *_b );

  // clonable

  let a : String = "abc".to_string();
  let _b : Box<  String  > = the_module::clone_into_box( &a );
  a_id!( a, *_b );

  // str slice

  let a : &str = "abc";
  let _b : Box<  str  > = the_module::clone_into_box( a );
  a_id!( *a, *_b );

  // slice

  let a : &[ i32 ] = &[ 1, 2, 3 ];
  let _b : Box<  [ i32 ]  > = the_module::clone_into_box( a );
  a_id!( *a, *_b );

  //

}

#[ test ]
fn clone()
{

  // copyable

  let a : i32 = 13;
  let _b : i32 = the_module::clone( &a );
  a_id!( a, _b );

  // clonable

  let a : String = "abc".to_string();
  let _b : String = the_module::clone( &a );
  a_id!( a, _b );

  // str slice

  let a : &str = "abc";
  let _b : &str = the_module::clone( &a );
  a_id!( a, _b );

  // slice

  let a : &[ i32 ] = &[ 1, 2, 3 ];
  let _b : &[ i32 ] = the_module::clone( &a );
  a_id!( a, _b );

  //

}

#[ test ]
fn basic()
{

  //

  let e_i32 : Box<  dyn Trait1  > = Box::new( 13 );
  let e_i64 : Box<  dyn Trait1  > = Box::new( 14 );
  let e_string : Box<  dyn Trait1  > = Box::new( "abc".to_string() );
  let e_str_slice : Box<  dyn Trait1  > = Box::new( "abcd" );
  let e_slice : Box<  dyn Trait1  > = Box::new( &[ 1i32, 2i32 ] as &[ i32 ] );

  //

  let vec : Vec< Box<  dyn Trait1  > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let _vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( _vec, [ 13, 14, 3, 4, 2 ] );

  //

  let vec : Vec< Box<  dyn Trait1  > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec2 = the_module::clone( &vec );
  let _vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let _vec2 = vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( _vec, _vec2 );

  //

  let vec : Vec< Box<  dyn Trait1  > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec2 = vec.clone();
  let _vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let _vec2 = vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( _vec, _vec2 );

  //

}
