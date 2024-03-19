#![ allow( dead_code ) ]

use super::*;
use collection_tools::HashMap;

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
