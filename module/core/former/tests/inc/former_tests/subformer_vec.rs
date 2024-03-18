#![ allow( dead_code ) ]

use super::*;

#[ test ]
fn push()
{

  //

  let got : Vec< String > = the_module::VectorSubformer::new()
  .push( "a" )
  .push( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

#[ test ]
fn replace()
{

  let got : Vec< String > = the_module::VectorSubformer::new()
  .push( "x" )
  .replace( vec![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}
