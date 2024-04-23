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
    Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Types2, Definition > >,
    Former2 : former::FormerBegin< Definition2 >,
  {
    Former2::_begin( None, Some( self ), ParametersDescriptorAddElementOnEnd::default() )
  }

  #[ inline( always ) ]
  pub fn _descriptor_former_set3< Former2, Definition2, Types2 >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = DescriptorFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Types2, Definition > >,
    Former2 : SubFormerTrait< Self, Definition, Definition2, Types2 >,
  {
    Former2::_begin( None, Some( self ), ParametersDescriptorAddElementOnEnd::default() )
  }

  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  DescriptorSubformer< Self, impl DescriptorSubformerEnd< Self > >
  {
    self._descriptor_former_set2
    ::
    <
      DescriptorFormer< _ >,
      _,
      _,
    >
    ()
    .name( name )
  }

}

pub trait SubFormerTrait< Former, Definition, Definition2, Types2 >
where
  Types2 : former::FormerDefinitionTypes
  <
    Storage = DescriptorFormerStorage,
    Formed = Former,
    Context = Former,
  >,
  Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Types2, Definition > >,
  Self : former::FormerBegin< Definition2 >,
{
}

impl< T, Former, Definition, Definition2, Types2 > SubFormerTrait< Former, Definition, Definition2, Types2 >
for T
where
  Types2 : former::FormerDefinitionTypes
  <
    Storage = DescriptorFormerStorage,
    Formed = Former,
    Context = Former,
  >,
  Definition2 : former::FormerDefinition< Types = Types2, End = ParametersDescriptorAddElementOnEnd< Types2, Definition > >,
  Self : former::FormerBegin< Definition2 >,
{
}

/// Handles the completion of the subformer for `Descriptor`.
pub struct ParametersDescriptorAddElementOnEnd< Definition, Types2 >
{
  _phantom : core::marker::PhantomData< fn( Definition, Types2 ) >,
}

impl< Definition, Types2 > Default
for ParametersDescriptorAddElementOnEnd< Definition, Types2 >
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
for ParametersDescriptorAddElementOnEnd< Types2, Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes
  <
    Storage = ParametersFormerStorage,
    // Storage = X,
  >,
  Types2 : former::FormerDefinitionTypes
  <
    Storage = DescriptorFormerStorage,
    Formed = ParametersFormer< Definition >,
    Context = ParametersFormer< Definition >,
  >,
  // Types2::Storage : former::StoragePreform,
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
