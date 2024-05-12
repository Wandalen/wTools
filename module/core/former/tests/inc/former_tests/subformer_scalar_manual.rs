#![ allow( dead_code ) ]

use super::*;

/// Child
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  data : bool,
}

/// Parent

#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ scalar_subform ]
  child : Child,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{


}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _child_scalar_subformer< Former2, Definition2 >( self ) ->
  Former2
  where
    Definition2 : former::FormerDefinition
    <
      End = ParentFormerScalarSubformChildEnd< Definition >,
      Storage = < Child as former::EntityToStorage >::Storage,
      Formed = Self,
      Context = Self,
    >,
    Definition2::Types : former::FormerDefinitionTypes
    <
      Storage = < Child as former::EntityToStorage >::Storage,
      Formed = Self,
      Context = Self,
    >,
    Former2 : former::FormerBegin< Definition2 >,
  {
    Former2::former_begin( None, Some( self ), ParentFormerScalarSubformChildEnd::default() )
  }

}

// = end

/// Handles the completion of and element of subformer's container.
pub struct ParentFormerScalarSubformChildEnd< Definition >
{
  _phantom : core::marker::PhantomData< fn( Definition ) >,
}

impl< Definition > Default
for ParentFormerScalarSubformChildEnd< Definition >
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
for ParentFormerScalarSubformChildEnd< Definition >
where
  Definition : former::FormerDefinition
  <
    Storage = < Parent as former::EntityToStorage >::Storage,
  >,
  Types2 : former::FormerDefinitionTypes
  <
    Storage = < Child as former::EntityToStorage >::Storage,
    Formed = ParentFormer< Definition >,
    Context = ParentFormer< Definition >,
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
    debug_assert!( super_former.storage.child.is_none() );
    super_former.storage.child = Some( ::core::convert::Into::into( former::StoragePreform::preform( substorage ) ) );
    super_former
  }
}

// == begin of generated

// == end of generated

// include!( "./only_test/subformer_scalar.rs" );
// xxx : uncomment