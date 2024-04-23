#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Descriptor
{
  name : String,
  is_mandatory : bool,
}

/// Parameters required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Parameters
{
  #[ subformer( former::VectorDefinition ) ]
  // #[ element_subformer( Descriptor ) ]
  descriptors : Vec< Descriptor >,
}

impl< Definition > former::FormerBegin< Definition >
for DescriptorFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = DescriptorFormerStorage >,
{

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin_coercing( None, context, on_end )
  }

}

impl< Definition > ParametersFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = ParametersFormerStorage >,
{


  #[ inline( always ) ]
  pub fn _descriptor_former_with_closure< Former2, Definition2, Types2 >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = DescriptorFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2 : former::FormerDefinition< Types = Types2, End = former::FormingEndClosure< Types2 > >,
    Definition2::End : former::FormingEnd< Definition2::Types >,
    Former2 : former::FormerBegin
    <
      Definition2,
    >,
  {
    let on_end = | substorage : DescriptorFormerStorage, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if super_former.storage.descriptors.is_none()
      {
        super_former.storage.descriptors = Some( Default::default() );
      }
      if let Some( ref mut descriptors ) = super_former.storage.descriptors
      {
        former::ContainerAdd::add( descriptors, former::StoragePreform::preform( substorage ) );
      }
      super_former
    };
    Former2::_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
    // Former2::_begin( None, Some( self ), on_end )
  }

  #[ inline( always ) ]
  pub fn _descriptor_former_set2< Former2, Definition2, Types2 >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = DescriptorFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Definition > >,
    Former2 : former::FormerBegin< Definition2 >,
  {
    Former2::_begin( None, Some( self ), ParametersDescriptorAddElementOnEnd::default() )
  }

  // #[ inline( always ) ]
  // pub fn _descriptor_former_set3< Former2 >( self ) ->
  // Former2
  // where
  //   Former2 : SubFormerTrait2< Definition = Definition, Former = Self >,
  //   // Types2 : former::FormerDefinitionTypes
  //   // <
  //   //   Storage = DescriptorFormerStorage,
  //   //   Formed = Self,
  //   //   Context = Self,
  //   // >,
  //   // Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Definition > >,
  //   // Former2 : SubFormerTrait< Self, Definition, Definition2, Types2 >,
  // {
  //   Former2::_begin( None, Some( self ), ParametersDescriptorAddElementOnEnd::default() )
  // }

  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  DescriptorSubformer< Self, impl DescriptorSubformerEnd< Self > >
  {
    self._descriptor_former_set2
    ::< DescriptorFormer< _ >, _, _, >()
    .name( name )
  }

}

pub trait EntityToFormer
{
  type Storage;
  type Former;
  // type Formed;
  // type Context;
  // type Types;
  // type Definition;
}

impl EntityToFormer for Descriptor
{
  type Storage = DescriptorFormerStorage;
  type Former = DescriptorFormer;
  // type Formed;
  // type Context;
  // type Types;
  // type Definition;
}

// pub trait SubFormerTrait2
// where
//   < Self::Definition2 as former::FormerDefinition >::Types : former::FormerDefinitionTypes
//   <
//     Storage = DescriptorFormerStorage,
//     Formed = Self::Former,
//     Context = Self::Former,
//   >,
//   Self : former::FormerBegin< Self::Definition2 >,
// {
//   type Former;
//   type Definition;
//   type Definition2 : former::FormerDefinition
//   <
//     End = ParametersDescriptorAddElementOnEnd
//     <
//       < Self::Definition2 as former::FormerDefinition >::Types,
//       Self::Definition,
//     >,
//   >;
//   // type Types2;
// }
//
// pub trait SubFormerTrait< Former, Definition, Definition2, Types2 >
// where
//   Types2 : former::FormerDefinitionTypes
//   <
//     Storage = DescriptorFormerStorage,
//     Formed = Former,
//     Context = Former,
//   >,
//   Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Definition > >,
//   Self : former::FormerBegin< Definition2 >,
// {
// }
//
// impl< T, Former, Definition, Definition2, Types2 > SubFormerTrait< Former, Definition, Definition2, Types2 >
// for T
// where
//   Types2 : former::FormerDefinitionTypes
//   <
//     Storage = DescriptorFormerStorage,
//     Formed = Former,
//     Context = Former,
//   >,
//   Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Definition > >,
//   Self : former::FormerBegin< Definition2 >,
// {
// }

/// Handles the completion of and element of subformer's container.
pub struct ParametersDescriptorAddElementOnEnd< Definition >
{
  _phantom : core::marker::PhantomData< fn( Definition ) >,
}

impl< Definition > Default
for ParametersDescriptorAddElementOnEnd< Definition >
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Types2, Definition > former::FormingEnd< Types2, >
for ParametersDescriptorAddElementOnEnd< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = ParametersFormerStorage,
  >,
  Types2 : former::FormerDefinitionTypes
  <
    // Storage = DescriptorFormerStorage,
    // Storage = < DescriptorFormerDefinitionTypes as former::FormerDefinitionTypes >::Storage,
    Storage = < Descriptor as EntityToFormer >::Storage,
    Formed = ParametersFormer< Definition >,
    Context = ParametersFormer< Definition >,
  >,
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    substorage : Types2::Storage,
    super_former : core::option::Option< Types2::Context >,
  )
  -> Types2::Formed
  {
    let mut super_former = super_former.unwrap();
    if super_former.storage.descriptors.is_none()
    {
      super_former.storage.descriptors = Some( Default::default() );
    }
    if let Some( ref mut descriptors ) = super_former.storage.descriptors
    {
      former::ContainerAdd::add( descriptors, former::StoragePreform::preform( substorage ) );
    }
    super_former
  }
}

#[ test ]
fn basic()
{

  let got = Parameters::former()
  .descriptors()
    .add( Descriptor::former().name( "a" ).form() )
    .add( Descriptor::former().name( "b" ).form() )
    .end()
  .form();

  let descriptors = vec!
  [
    Descriptor { name : "a".to_string(), is_mandatory : false },
    Descriptor { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parameters { descriptors };
  a_id!( got, exp );

}

#[ test ]
fn descriptor()
{

  let got = Parameters::former()
  .descriptor( "a" ).end()
  .descriptor( "b" ).end()
    // .add( Descriptor::former().name( "a" ).form() )
    // .add( Descriptor::former().name( "b" ).form() )
    // .end()
  .form();

  let descriptors = vec!
  [
    Descriptor { name : "a".to_string(), is_mandatory : false },
    Descriptor { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parameters { descriptors };
  a_id!( got, exp );

}
