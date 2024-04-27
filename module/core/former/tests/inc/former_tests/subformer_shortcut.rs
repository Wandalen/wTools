#![ allow( dead_code ) ]

use super::*;

// xxx : rename
/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Descriptor
{
  name : String,
  is_mandatory : bool,
}

// impl former::EntityToFormer_ for Descriptor
// where
//   Self : Sized,
// {
//   type Storage = DescriptorFormerStorage;
//   type Former = DescriptorFormer;
// }

/// Parameters required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Parameters
{
  // xxx : is definition as argument fine?
  #[ container( former::VectorDefinition ) ]
  #[ subform ]
  descriptors : Vec< Descriptor >,
}

// impl former::EntityToFormer_ for Parameters
// where
//   Self : Sized,
// {
//   type Storage = ParametersFormerStorage;
//   type Former = ParametersFormer;
// }

// impl< Definition > former::EntityToFormer< Definition > for Parameters
// where
//   Definition : former::FormerDefinition< Storage = ParametersFormerStorage >,
// {
//   type Former = ParametersFormer< Definition >;
// }
//
// impl former::EntityToStorage for Parameters
// {
//   type Storage = ParametersFormerStorage;
// }

impl< Definition > former::FormerBegin< Definition >
for DescriptorFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = DescriptorFormerStorage >,
{

  #[ inline( always ) ]
  fn former_begin
  (
    storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  )
  -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin_precise( None, context, on_end )
  }

}

impl< Definition > ParametersFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parameters as former::EntityToStorage >::Storage >,
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
    Definition2 : former::FormerDefinition
    <
      Types = Types2,
      End = former::FormingEndClosure< Types2 >,
      Storage = DescriptorFormerStorage,
      Formed = Self,
      Context = Self,
    >,
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
    Former2::former_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
  }

  #[ inline( always ) ]
  pub fn _descriptor_former_set2< Former2, Definition2 >( self ) ->
  Former2
  where
    Definition2 : former::FormerDefinition
    <
      End = ParametersFormerAddDescriptorsEnd< Definition >,
      Storage = < Descriptor as former::EntityToStorage >::Storage,
      Formed = Self,
      Context = Self,
    >,
    Definition2::Types : former::FormerDefinitionTypes
    <
      Storage = < Descriptor as former::EntityToStorage >::Storage,
      Formed = Self,
      Context = Self,
    >,
    Former2 : former::FormerBegin< Definition2 >,
  {
    Former2::former_begin( None, Some( self ), ParametersFormerAddDescriptorsEnd::default() )
  }

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  DescriptorSubformer< Self, impl DescriptorSubformerEnd< Self > >
  {
    self._descriptor_former_set2
    ::< DescriptorFormer< _ >, _, >()
    .name( name )
  }

}

// xxx : make manual version of the file
// // zzz : improve description
// /// Handles the completion of and element of subformer's container.
// pub struct ParametersFormerAddDescriptorsEnd< Definition >
// {
//   _phantom : core::marker::PhantomData< fn( Definition ) >,
// }
//
// impl< Definition > Default
// for ParametersFormerAddDescriptorsEnd< Definition >
// {
//   #[ inline( always ) ]
//   fn default() -> Self
//   {
//     Self
//     {
//       _phantom : core::marker::PhantomData,
//     }
//   }
// }
//
// impl< Types2, Definition > former::FormingEnd< Types2, >
// for ParametersFormerAddDescriptorsEnd< Definition >
// where
//   Definition : former::FormerDefinition,
//   Definition::Types : former::FormerDefinitionTypes
//   <
//     Storage = < Parameters as former::EntityToStorage >::Storage,
//   >,
//   Types2 : former::FormerDefinitionTypes
//   <
//     // Storage = < Descriptor as former::EntityToStorage >::Storage,
//     Storage = < < Vec< Descriptor > as former::ContainerAdd >::Element as former::EntityToStorage >::Storage,
//     Formed = ParametersFormer< Definition >,
//     Context = ParametersFormer< Definition >,
//     // Formed = < Parameters as former::EntityToFormer >::Former,
//     // Context = < Parameters as former::EntityToFormer >::Former,
//   >,
//   // Types2::Storage : former::StoragePreform< Preformed =  >,
// {
//   #[ inline( always ) ]
//   fn call
//   (
//     &self,
//     substorage : Types2::Storage,
//     super_former : core::option::Option< Types2::Context >,
//   )
//   -> Types2::Formed
//   {
//     let mut super_former = super_former.unwrap();
//     if super_former.storage.descriptors.is_none()
//     {
//       super_former.storage.descriptors = Some( Default::default() );
//     }
//     if let Some( ref mut fields ) = super_former.storage.descriptors
//     {
//       former::ContainerAdd::add( fields, former::StoragePreform::preform( substorage ) );
//     }
//     super_former
//   }
// }

//

include!( "./only_test/subformer_shortcut.rs" );
