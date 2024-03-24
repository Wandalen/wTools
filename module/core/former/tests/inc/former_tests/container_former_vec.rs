#![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

#[ test ]
fn definitions()
{


  pub fn f1< Definition >( _x : Definition )
  where
    Definition : former::FormerDefinitionTypes,
  {
  }

  pub fn f2< Definition >( _x : Definition )
  where
    Definition : former::FormerDefinition,
  {
  }

  pub fn f3< Definition, End >( _x : End )
  where
    Definition : former::FormerDefinitionTypes,
    End : former::FormingEnd< Definition >,
  {
  }

  f1( former::VectorDefinition::< String, () >::new() );
  f2( former::VectorDefinition::< String, (), Vec< String >, the_module::ReturnStorage >::new() );
  f3::< former::VectorDefinition< String, () >, the_module::ReturnStorage >( the_module::ReturnStorage );
  f3::< < former::VectorDefinition< String, (), Vec< String >, the_module::ReturnStorage > as the_module::FormerDefinition >::Types, the_module::ReturnStorage >( the_module::ReturnStorage );

}

//

#[ test ]
fn push()
{

  //

  let got : Vec< String > = the_module
  ::ContainerSubformer
  ::< String, former::VectorDefinition< String, (), Vec< String >, the_module::ReturnStorage > >
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
    former::VectorDefinition< String, (), Vec< String >, the_module::ReturnStorage >,
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

  let got : Vec< String > = the_module::VectorSubformer::< String, (), Vec< String >, the_module::ReturnStorage >::new()
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

//

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

//

#[ test ]
fn begin_and_custom_end()
{

  // xxx : make example with that

  // basic case

  fn return_13( _storage : Vec< String >, context : Option< () > ) -> f32
  {
    13.1
  }
  let got = the_module::VectorSubformer::begin( None, None, return_13 )
  .push( "a" )
  .push( "b" )
  .form();
  let exp = 13.1;
  a_id!( got, exp );

  // with a context

  fn context_plus_13( _storage : Vec< String >, context : Option< f32 > ) -> f32
  {
    if let Some( context ) = context
    {
      13.1 + context
    }
    else
    {
      13.1
    }
  }
  let got = the_module::VectorSubformer::begin( None, Some( 10.0 ), context_plus_13 )
  .push( "a" )
  .push( "b" )
  .form();
  let exp = 23.1;
  a_id!( got, exp );

  //

}

//

#[ test ]
fn custom_definition()
{

  // xxx2 : continue
  struct Return13;
  impl former::FormerDefinitionTypes for Return13
  {
    type Storage = Vec< String >;
    type Formed = i32;
    type Context = ();
  }

  impl former::FormerDefinition for Return13
  {
    type Types = Return13;
    type End = Return13;
  }

  // -

  impl the_module::FormingEnd< Return13 >
  for Return13
  {
    fn call
    (
      &self,
      storage : < Return13 as the_module::FormerDefinitionTypes >::Storage,
      context : Option< < Return13 as the_module::FormerDefinitionTypes >::Context >
    ) -> < Return13 as the_module::FormerDefinitionTypes >::Formed
    {
      13
    }
  }

  //

  let got = the_module::ContainerSubformer::< String, Return13 >::begin( None, None, Return13 )
  .push( "a" )
  .push( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  //

//   // -
//
//   fn return_13( _storage : Vec< String >, _context : Option< () > ) -> i32
//   {
//     13
//   }
//
//   let end_wrapper : the_module::FormingEndWrapper< Return13 > = the_module::FormingEndWrapper::new( return_13 );

}

//
