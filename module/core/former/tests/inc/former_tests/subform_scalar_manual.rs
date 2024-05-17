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
  #[ scalar( setter = false ) ]
  // #[ scalar_subform ]
  child : Child,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _child_subform_scalar< Former2, Definition2 >( self ) ->
  Former2
  where
    Definition2 : former::FormerDefinition
    <
      End = ParentFormerSubformScalarChildEnd< Definition >,
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
    Former2::former_begin( None, Some( self ), ParentFormerSubformScalarChildEnd::default() )
  }

}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._child_subform_scalar
    ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  }

}

// = end

/// Represents the endpoint for the forming process of a scalar field managed by a subformer within a `Parent` entity.
///
/// This structure is a critical component of the forming process when using a subform scalar setter. It handles
/// the finalization of the scalar field's value that has been configured through its dedicated subformer.
/// Essentially, this end action integrates the individually formed scalar value back into the parent structure.
///
/// ## Type Parameters
///
/// - `Definition`: The type that defines the former setup for the `Parent` entity, influencing storage and behavior during forming.
///
/// ## Parameters of `call`
///
/// - `substorage`: Storage type specific to the `Child`, containing the newly formed scalar value.
/// - `super_former`: An optional context of the `ParentFormer`, which will receive the value. The function ensures
///   that this context is not `None` and inserts the formed value into the designated field within `Parent`'s storage.
///

pub struct ParentFormerSubformScalarChildEnd< Definition >
{
  _phantom : core::marker::PhantomData< fn( Definition ) >,
}

impl< Definition > Default
for ParentFormerSubformScalarChildEnd< Definition >
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
for ParentFormerSubformScalarChildEnd< Definition >
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

include!( "./only_test/subform_scalar.rs" );
