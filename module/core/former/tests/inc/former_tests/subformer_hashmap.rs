#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn push()
{

  //

  let got : HashMap< String, String > = the_module::HashMapSubformer::new()
  .insert( "a", "x" )
  .insert( "b", "y" )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : HashMap< String, String > = the_module::HashMapSubformer::new()
  .insert( "x", "x" )
  .replace( hmap![ "a".to_string() => "x".to_string(), "b".to_string() => "y".to_string() ] )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

}

