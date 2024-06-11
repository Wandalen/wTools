
#[ test ]
fn clone_into_box()
{

  // copyable

  let a : i32 = 13;
  let b : Box< i32 > = the_module::clone_into_box( &a );
  a_id!( a, *b );

  // clonable

  let a : String = "abc".to_string();
  let b : Box< String > = the_module::clone_into_box( &a );
  a_id!( a, *b );

  // str slice

  let a : &str = "abc";
  let b : Box< str > = the_module::clone_into_box( a );
  a_id!( *a, *b );

  // slice

  let a : &[ i32 ] = &[ 1, 2, 3 ];
  let b : Box< [ i32 ] > = the_module::clone_into_box( a );
  a_id!( *a, *b );

  //

}

#[ test ]
fn clone()
{

  // copyable

  let a : i32 = 13;
  let b : i32 = the_module::clone( &a );
  a_id!( a, b );

  // clonable

  let a : String = "abc".to_string();
  let b : String = the_module::clone( &a );
  a_id!( a, b );

  // str slice

  let a : &str = "abc";
  let b : &str = the_module::clone( &a );
  a_id!( a, b );

  // slice

  let a : &[ i32 ] = &[ 1, 2, 3 ];
  let b : &[ i32 ] = the_module::clone( &a );
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
  let vec2 = the_module::clone( &vec );
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
