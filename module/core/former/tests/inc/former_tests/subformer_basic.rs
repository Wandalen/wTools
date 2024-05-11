#![ deny( missing_docs ) ]
#![ allow( dead_code ) ]
use super::*;

//
// this should work
//
// let ca = Parent::former()
// .parameter1( "val" )
// .command( "echo" )
//   .name( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .end()
// .command( "exit" )
//   .name( "just exit" )
//   .routine( || exit() )
//   .end()
// .perform()
// ;
// ca.execute( input ).unwrap();

// == property

#[ derive( Debug, PartialEq, Default ) ]
pub struct Property< Name >
{
  name : Name,
  description : String,
  code : isize,
}

/// generated by new
impl< Name > Property< Name >
{
  #[ inline ]
  pub fn new< Description, Code >( name : Name, description : Description, code : Code ) -> Self
  where
    Name : core::convert::Into< Name >,
    Description : core::convert::Into< String >,
    Code : core::convert::Into< isize >,
  {
    Self { name : name.into(), description : description.into(), code : code.into() }
  }
}

// == command

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Child< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub subject : String,
  #[ container( definition = former::HashMapDefinition ) ]
  pub properties : collection_tools::HashMap< K, Property< K > >,
}

// manual
impl< K, Definition > ChildFormer< K, Definition >
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition< Storage = ChildFormerStorage< K > >,
  Definition::Storage : former::StoragePreform,
{

  /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
  #[ inline( always ) ]
  pub fn property< Name, Description, Code >
  ( mut self, name : Name, description : Description, code : Code ) -> Self
  where
    Name : core::convert::Into< K > + Clone,
    Description : core::convert::Into< String >,
    Code : core::convert::Into< isize >,
  {
    if self.storage.properties.is_none()
    {
      self.storage.properties = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut properties ) = self.storage.properties
    {
      let property = Property
      {
        name : name.clone().into(),
        description : description.into(),
        code : code.into(),
      };
      properties.insert( name.into(), property );
    }
    self
  }

}

// == aggregator

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Parent< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub parameter1 : String,
  #[ container( definition = former::HashMapDefinition ) ]
  pub commands : collection_tools::HashMap< String, Child< K > >,
}

// ==

include!( "./only_test/subformer_basic.rs" );
