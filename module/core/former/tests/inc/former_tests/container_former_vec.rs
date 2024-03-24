#![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

#[ test ]
fn definitions()
{


  pub fn f1< Definition >( _x : Definition )
  where
    Definition : former::FormerDefinition,
  {
  }

  pub fn f2< Definition >( _x : Definition )
  where
    Definition : former::FormerDefinition2,
  {
  }

  pub fn f3< Definition, End >( _x : End )
  where
    Definition : former::FormerDefinition,
    End : former::FormingEnd< Definition >,
  {
  }

  f1( former::VectorDefinitionTypes::< String, () >::new() );
  f2( former::VectorDefinition::< String, (), the_module::ReturnStorage >::new() );
  f3::< former::VectorDefinitionTypes< String, () >, the_module::ReturnStorage >( the_module::ReturnStorage );
  f3::< < former::VectorDefinition< String, (), the_module::ReturnStorage > as the_module::FormerDefinition2 >::Definition, the_module::ReturnStorage >( the_module::ReturnStorage );

}

#[ test ]
fn push()
{

  //

  let got : Vec< String > = the_module
  ::ContainerSubformer
  ::< String, former::VectorDefinition< String, (), the_module::ReturnStorage > >
  ::new()
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

  let got : Vec< String > = the_module::ContainerSubformer::
  <
    String,
    former::VectorDefinition< String, (), the_module::ReturnStorage >,
  >::new()
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

  let got : Vec< String > = the_module::VectorSubformer::< String, (), the_module::ReturnStorage >::new()
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

  //

  use the_module::VecExt;
  let got : Vec< String > = Vec::former()
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
