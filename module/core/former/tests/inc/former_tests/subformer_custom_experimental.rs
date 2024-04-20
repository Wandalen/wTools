#![ allow( dead_code ) ]
use super::*;

// == command

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub subject : K,
}

// == aggregator

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Aggregator< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub parameter1 : String,
  #[ subformer( former::HashMapDefinition ) ]
  pub commands : collection_tools::HashMap< String, Command< K > >,
}

// =

impl< K, Definition > AggregatorFormer
<
  K,
  Definition,
>
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = AggregatorFormerStorage< K > >,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform< Preformed = Aggregator< K > >,
{

  //
  #[ inline( always ) ]
  pub fn command_with_helper< IntoName >( self, name : IntoName )
  ->
  CommandSubformer< K, Self, impl CommandSubformerEnd< K, Self > >
  where
    IntoName : core::convert::Into< String >,
    ContainerAddElement
    <
      collection_tools::HashMap< String, Command< K > >,
      ( String, Command< K >, ),
      Command< K >
    >
    :
    CommandSubformerEnd< K, Self >,
  {
    let former
    = CommandFormer::_begin_precise
    (
      None,
      Some( self ),
      ContainerAddElement::default(),
    );
    former.name( name )
  }

}

//

// /// Convert an entity to an element which could be added to a container.
// pub trait IntoElement< Element >
// {
//   /// Convert an entity to an element which could be added to a container.
//   fn into_element( self ) -> Element;
// }
//
// impl< K > IntoElement< ( String, Command< K > ) >
// for Command< K >
// where
//   K : core::hash::Hash + std::cmp::Eq,
// {
//   fn into_element( self ) -> ( String, Command< K > )
//   {
//     ( self.name.clone(), self )
//   }
// }

//

impl< K > From< Command< K > >
for ( String, Command< K > )
where
  K : core::hash::Hash + std::cmp::Eq,
{
  #[ inline( always ) ]
  fn from( src : Command< K > ) -> Self
  {
    ( src.name.clone(), src )
  }
}

//

/// get container for a field out of a storage
pub trait FormerStorageExtractContainer< Target >
{
  fn container_mut( &mut self ) -> &mut Target;
}

impl< K > FormerStorageExtractContainer< collection_tools::HashMap< String, Command< K > > >
for AggregatorFormerStorage< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  fn container_mut( &mut self ) -> &mut collection_tools::HashMap< String, Command< K > >
  {
    if let Some( ref mut commands ) = self.commands
    {
      commands
    }
    else
    {
      let commands : collection_tools::HashMap< String, Command< K > > = Default::default();
      self.commands = Some( commands );
      self.commands.as_mut().unwrap()
    }
  }
}

//

/// extract storage from a former
pub trait FormerExtractStorage
{
  type Storage;
  fn storage_mut( &mut self ) -> &mut Self::Storage;
}

impl< K > FormerExtractStorage
for AggregatorFormer< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  type Storage = AggregatorFormerStorage< K >;
  fn storage_mut( &mut self ) -> &mut Self::Storage
  {
    &mut self.storage
  }
}

//

#[ derive( Debug ) ]
pub struct ContainerAddElement< SuperContainer, Element, SubFormed >
( core::marker::PhantomData< fn( SuperContainer, Element, SubFormed ) > );

impl< SuperContainer, Element, SubFormed > ::core::default::Default
for ContainerAddElement< SuperContainer, Element, SubFormed >
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self( core::marker::PhantomData )
  }
}

impl
<
  SuperFormer,
  SuperContainer,
  Element,
  SubFormed,
  SubDefinition,
>
former::FormingEnd
<
  SubDefinition,
  // CommandFormerDefinitionTypes
  // <
  //   K,
  //   AggregatorFormer< K, SuperDefinition >,
  //   AggregatorFormer< K, SuperDefinition >,
  // >,
>
for ContainerAddElement
<
  SuperContainer,
  Element,
  SubFormed,
>
where
  SuperFormer : FormerExtractStorage<>,
  < SuperFormer as FormerExtractStorage >::Storage : FormerStorageExtractContainer< SuperContainer >,
  SuperContainer : former::ContainerAdd< Element = Element >,

  SubDefinition : former::FormerDefinitionTypes
  <
    Formed = SuperFormer,
    Context = SuperFormer,
  >,
  SubDefinition::Storage : former::StoragePreform< Preformed = SubFormed >,

  SubFormed : Into< Element >,
{

  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : SubDefinition::Storage,
    super_former : Option< SuperFormer >,
  )
  ->
  SuperFormer
  {

    let storage : SubFormed = former::StoragePreform::preform( storage );
    let mut super_former = super_former.unwrap();

    let container = FormerStorageExtractContainer
    ::< SuperContainer >
    ::container_mut( FormerExtractStorage::storage_mut( &mut super_former ) );

    former::ContainerAdd::add
    (
      container,
      Into::< Element >::into( storage ),
    );

    super_former
  }

}

// ==

include!( "./only_test/subformer_custom_experimental.rs" );
