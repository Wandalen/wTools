#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// Child struct with Former derived for builder pattern support
#[ derive( Debug, PartialEq, former::Former ) ]
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
  #[ subform( setter = false ) ]
  #[ scalar( setter = false ) ] // xxx : should not be required in this case
  command : HashMap< String, Child >,
}

// Use ChildFormer as custom subformer for ParentFormer to add commands by name.
impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn _children_add_with_closure< Former2, Definition2, Types2 >( self ) ->
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
        former::ContainerAdd::add
        (
          children,
          < < HashMap< String, Child > as former::Container >::Val as former::ValToElement< HashMap< String, Child > > >
          ::val_to_element( former::StoragePreform::preform( substorage ) )
        );
      }
      super_former
    };
    Former2::former_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
  }

  #[ inline( always ) ]
  pub fn command( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._command_add::< ChildFormer< _ >, _, >()
    .name( name )
  }

}

impl former::ValToElement< HashMap< String, Child > > for Child
{
  type Element = ( String, Child );
  #[ inline( always ) ]
  fn val_to_element( self ) -> Self::Element
  {
    ( self.name.clone(), self )
  }
}

// == begin of generated

// == end of generated

#[ test ]
fn basic()
{

  let got = Parent::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  a_id!( got.command.len(), 2 );

}
