#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// Child struct with Former derived for builder pattern support
#[ derive( Clone, Debug, PartialEq, former::Former ) ]
pub struct Child
{
  name : String,
  description : String,
}

// Parent struct to hold commands
#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Parent
{
  #[ subform_entry( setter = false ) ]
  command : HashMap< String, Child >,
}

// Use ChildFormer as custom subformer for ParentFormer to add commands by name.
impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  // more generic version
  #[ inline( always ) ]
  pub fn _children_subform_entry_with_closure< Former2, Definition2, Types2 >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = ChildFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2 : former::FormerDefinition
    <
      Types = Types2,
      End = former::FormingEndClosure< Types2 >,
      Storage = ChildFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2::End : former::FormingEnd< Definition2::Types >,
    Former2 : former::FormerBegin
    <
      Definition2,
    >,
  {
    let on_end = | substorage : ChildFormerStorage, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if super_former.storage.command.is_none()
      {
        super_former.storage.command = Some( Default::default() );
      }
      if let Some( ref mut children ) = super_former.storage.command
      {
        former::CollectionAdd::add
        (
          children,
          < < HashMap< String, Child > as former::Collection >::Val as former::ValToEntry< HashMap< String, Child > > >
          ::val_to_entry( former::StoragePreform::preform( substorage ) )
        );
      }
      super_former
    };
    Former2::former_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
  }

  // reuse _command_subform_entry
  #[ inline( always ) ]
  pub fn command( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._command_subform_entry::< ChildFormer< _ >, _, >()
    .name( name )
  }

  // that's how you should do custom subformer setters if you can't reuse _command_subform_entry
  #[ inline( always ) ]
  pub fn command2( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    let on_end = | substorage : ChildFormerStorage, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      let preformed = former::StoragePreform::preform( substorage );

      if super_former.storage.command.is_none()
      {
        super_former.storage.command = Some( Default::default() );
      }

      // add instance to the collection
      super_former.storage.command.as_mut().unwrap()
      .entry( preformed.name.clone() )
      .or_insert( preformed.clone() );

      // custom logic to add two instances to the collection
      super_former.storage.command.as_mut().unwrap()
      .entry( format!( "{}_2", preformed.name ) )
      .or_insert( preformed.clone() );

      super_former
    };
    let subformer = ChildAsSubformer::< Self, _ >::begin( None, Some( self ), former::FormingEndClosure::new( on_end ) );
    subformer.name( name )
  }

}

impl former::ValToEntry< HashMap< String, Child > > for Child
{
  type Entry = ( String, Child );
  #[ inline( always ) ]
  fn val_to_entry( self ) -> Self::Entry
  {
    ( self.name.clone(), self )
  }
}

// == begin of generated

// == end of generated

#[ test ]
fn custom1()
{

  let got = Parent::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  let got = got.command.iter().map( | e | e.0 ).cloned().collect::< collection_tools::HashSet< String > >();
  let exp = collection_tools::hset!
  [
    "echo".into(),
    "exit".into(),
  ];
  a_id!( got, exp );

}

#[ test ]
fn custom2()
{

  let got = Parent::former()
  .command2( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command2( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  let got = got.command.iter().map( | e | e.0 ).cloned().collect::< collection_tools::HashSet< String > >();
  let exp = collection_tools::hset!
  [
    "echo".into(),
    "echo_2".into(),
    "exit".into(),
    "exit_2".into(),
  ];
  a_id!( got, exp );

}
