#![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

// impl< Definition, T > FormingEnd< Definition >
// for ReturnStorage
// where
//   Definition : FormerDefinition< Context = (), Storage = T, Formed = T, End = Self >,

pub fn f0< Definition >( x : Definition )
where
  Definition : former::FormerDefinition,
{
}

pub fn f1< Definition, End >( x : End )
where
  Definition : former::FormerDefinition,
  End : former::FormingEnd< Definition >,
{
}

#[ test ]
fn push()
{

  // f0( former::VectorDefinition::< String, (), the_module::ReturnStorage >::new() );
  // f1::< former::VectorDefinition< String, (), the_module::ReturnStorage >, _ >( the_module::ReturnStorage );

  //

  // let got : Vec< String > = the_module
  // ::ContainerSubformer
  // ::< String, former::VectorDefinition< String, (), the_module::ReturnStorage > >
  // ::new()
  // .push( "a" )
  // .push( "b" )
  // .form();
  // let exp = vec!
  // [
  //   "a".to_string(),
  //   "b".to_string(),
  // ];
  // a_id!( got, exp );

  // Definition : FormerDefinition< Context = (), Storage = T, Formed = T, End = Self >,

  //

  // let got : Vec< String > = the_module::ContainerSubformer::
  // <
  //   String,
  //   former::VectorDefinition< String, (), the_module::ReturnStorage >,
  // >::new()
  // .push( "a" )
  // .push( "b" )
  // .form();
  // let exp = vec!
  // [
  //   "a".to_string(),
  //   "b".to_string(),
  // ];
  // a_id!( got, exp );
//
//   //
//
//   let got : Vec< String > = the_module::VectorSubformer::< String, () >::new()
//   .push( "a" )
//   .push( "b" )
//   .form();
//   let exp = vec!
//   [
//     "a".to_string(),
//     "b".to_string(),
//   ];
//   a_id!( got, exp );

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
