#![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

#[ test ]
fn push()
{

  let got : Vec< String > = the_module::ContainerSubformer::< String, former::VectorDefinition< String > >::new()
  .push( "a" )
  .push( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

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

  // let got : Vec< String > = the_module::VectorSubformer::new()
  // .push( "x" )
  // .replace( vec![ "a".to_string(), "b".to_string() ] )
  // .form();
  // let exp = vec!
  // [
  //   "a".to_string(),
  //   "b".to_string(),
  // ];
  // a_id!( got, exp );

}
