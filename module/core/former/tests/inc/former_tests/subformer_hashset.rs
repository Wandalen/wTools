#![ allow( dead_code ) ]

use super::*;
use collection_tools::HashSet;

#[ test ]
fn push()
{

  //

  let got : HashSet< String > = the_module::HashSetSubformer::new()
  .insert( "a" )
  .insert( "b" )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

#[ test ]
fn replace()
{

  let got : HashSet< String > = the_module::HashSetSubformer::new()
  .insert( "x" )
  .replace( hset![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}
