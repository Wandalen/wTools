// #![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

//

#[ test ]
fn add()
{

  // expliccit with ContainerSubformer

  let got : Vec< String > = the_module
  ::ContainerSubformer
  ::< String, former::VectorDefinition< String, (), Vec< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // expliccit with VectorSubformer

  let got : Vec< String > = the_module::VectorSubformer::< String, (), Vec< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with VectorSubformer

  let got : Vec< String > = the_module::VectorSubformer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : Vec< String > = the_module::VectorSubformer
  ::begin_coercing( Some( vec![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::VecExt;
  let got : Vec< String > = Vec::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  //

}

//

#[ test ]
fn replace()
{

  let got : Vec< String > = the_module::VectorSubformer::new( former::ReturnStorage )
  .add( "x" )
  .replace( vec![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

//
