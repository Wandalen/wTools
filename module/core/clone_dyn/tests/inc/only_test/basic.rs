
#[ test ]
fn clone_into_box()
{

  // copyable

  let a : i32 = 13;
  let b : Box< i32 > = clone_dyn::clone_into_box( &a );
  a_id!( a, *b );

  // clonable

  let a : String = "abc".to_string();
  let b : Box< String > = clone_dyn::clone_into_box( &a );
  a_id!( a, *b );

  // str slice

  let a : &str = "abc";
  let b : Box< str > = clone_dyn::clone_into_box( a );
  a_id!( *a, *b );

  // slice

  let a : &[ i32 ] = &[ 1, 2, 3 ];
  let b : Box< [ i32 ] > = clone_dyn::clone_into_box( a );
  a_id!( *a, *b );

  //

}

#[ test ]
fn clone()
{

  // copyable

  let a : i32 = 13;
  let b : i32 = clone_dyn::clone( &a );
  a_id!( a, b );

  // clonable

  let a : String = "abc".to_string();
  let b : String = clone_dyn::clone( &a );
  a_id!( a, b );

  // str slice

  let a : &str = "abc";
  let b : &str = clone_dyn::clone( &a );
  a_id!( a, b );

  // slice

  let a : &[ i32 ] = &[ 1, 2, 3 ];
  let b : &[ i32 ] = clone_dyn::clone( &a );
  a_id!( a, b );

  //

}

#[ test ]
fn basic()
{

  //

  let e_i32 : Box< dyn Trait1 > = Box::new( 13 );
  let e_i64 : Box< dyn Trait1 > = Box::new( 14 );
  let e_string : Box< dyn Trait1 > = Box::new( "abc".to_string() );
  let e_str_slice : Box< dyn Trait1 > = Box::new( "abcd" );
  let e_slice : Box< dyn Trait1 > = Box::new( &[ 1i32, 2i32 ] as &[ i32 ] );

  //

  let vec : Vec< Box< dyn Trait1 > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let vec2 = vec![ 13, 14, 3, 4, 2 ];
  a_id!( vec, vec2 );

  //

  let vec : Vec< Box< dyn Trait1 > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec2 = clone_dyn::clone( &vec );
  let vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let vec2 = vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( vec, vec2 );

  //

  let vec : Vec< Box< dyn Trait1 > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec2 = vec.clone();
  let vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let vec2 = vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( vec, vec2 );

  //

}
