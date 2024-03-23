#![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

// impl< Definition, T > FormingEnd< Definition >
// for ReturnStorage
// where
//   Definition : FormerDefinition< Context = (), Storage = T, Formed = T, End = Self >,

pub fn f1< Definition : former::FormerDefinition >( x : Definition )
{
}

#[ test ]
fn push()
{

  // f1( the_module::ReturnStorage );

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
