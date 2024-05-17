// #![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

//

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

  f1( former::VectorDefinitionTypes::< String, (), Vec< String > >::default() );
  f2( former::VectorDefinition::< String, (), Vec< String >, the_module::NoEnd >::default() );
  f3::< former::VectorDefinitionTypes< String, (), Vec< String > >, the_module::ReturnStorage >( the_module::ReturnStorage );
  f3::< < former::VectorDefinition< String, (), Vec< String >, the_module::NoEnd > as the_module::FormerDefinition >::Types, the_module::ReturnStorage >( the_module::ReturnStorage );

}

//

#[ test ]
fn begin_and_custom_end()
{

  // basic case

  fn return_13( _storage : Vec< String >, _context : Option< () > ) -> f32
  {
    13.1
  }
  let got = the_module::VectorFormer::begin( None, None, return_13 )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13.1;
  a_id!( got, exp );

  let got = the_module::VectorFormer::new( return_13 )
  .add( "a" )
  .add( "b" )
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
  let got = the_module::VectorFormer::begin( None, Some( 10.0 ), context_plus_13 )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 23.1;
  a_id!( got, exp );

  //

}

//

#[ test ]
fn custom_definition()
{

  struct Return13;
  impl former::FormerDefinitionTypes for Return13
  {
    type Storage = Vec< String >;
    type Formed = i32;
    type Context = ();
  }

  impl former::FormerMutator
  for Return13
  {
  }

  impl former::FormerDefinition for Return13
  {
    type Types = Return13;
    type End = Return13;
    type Storage = Vec< String >;
    type Formed = i32;
    type Context = ();
  }

  // -

  impl former::FormingEnd< Return13 >
  for Return13
  {
    fn call
    (
      &self,
      _storage : < Return13 as former::FormerDefinitionTypes >::Storage,
      _context : Option< < Return13 as former::FormerDefinitionTypes >::Context >
    ) -> < Return13 as former::FormerDefinitionTypes >::Formed
    {
      13
    }
  }

  //

  let got = former::CollectionFormer::< String, Return13 >::begin( None, None, Return13 )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  let got = former::CollectionFormer::< String, Return13 >::new( Return13 )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  //

}

//

#[ test ]
fn custom_definition_parametrized()
{

  struct Return13< E >( ::core::marker::PhantomData< E > );

  impl< E > Return13< E >
  {
    pub fn new() -> Self
    {
      Self ( ::core::marker::PhantomData )
    }
  }

  impl< E > former::FormerDefinitionTypes for Return13< E >
  {
    type Storage = Vec< E >;
    type Formed = i32;
    type Context = ();
  }

  impl< E > former::FormerMutator
  for Return13< E >
  {
  }

  impl< E > former::FormerDefinition for Return13< E >
  {
    type Types = Return13< E >;
    type End = Return13< E >;
    type Storage = Vec< E >;
    type Formed = i32;
    type Context = ();
  }

  // -

  impl< E > the_module::FormingEnd< Return13< E > >
  for Return13< E >
  {
    fn call
    (
      &self,
      _storage : < Return13< E > as the_module::FormerDefinitionTypes >::Storage,
      _context : Option< < Return13< E > as the_module::FormerDefinitionTypes >::Context >
    ) -> < Return13< E > as the_module::FormerDefinitionTypes >::Formed
    {
      13
    }
  }

  //

  let got = the_module::CollectionFormer::< String, Return13< String > >::begin_coercing( None, None, Return13::new() )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  let got = the_module::CollectionFormer::< String, Return13< String > >::new_coercing( Return13::new() )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  //

  type MyCollection< E > = the_module::CollectionFormer::< E, Return13< E > >;

  let got = MyCollection::< String >::begin_coercing( None, None, Return13::new() )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  let got = MyCollection::< String >::new_coercing( Return13::new() )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  //

}

//

#[ test ]
fn custom_definition_custom_end()
{

  struct Return13;
  impl former::FormerDefinitionTypes for Return13
  {
    type Storage = Vec< String >;
    type Formed = i32;
    type Context = ();
  }
  impl former::FormerMutator
  for Return13
  {
  }
  impl former::FormerDefinition for Return13
  {
    type Types = Return13;
    type End = former::FormingEndClosure< < Self as former::FormerDefinition >::Types >;
    type Storage = Vec< String >;
    type Formed = i32;
    type Context = ();
  }

  fn return_13( _storage : Vec< String >, _context : Option< () > ) -> i32
  {
    13
  }

  let end_wrapper : the_module::FormingEndClosure< Return13 > = the_module::FormingEndClosure::new( return_13 );
  let got = the_module::CollectionFormer::< String, Return13 >::new( end_wrapper )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  let got = the_module::CollectionFormer::< String, Return13 >::new( return_13.into() )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  let got = the_module::CollectionFormer::< String, Return13 >::new_coercing( return_13 )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = 13;
  a_id!( got, exp );

  //

}

//
