#![ allow( dead_code ) ]
use super::*;

//
// this should work
//
// let ca = Aggregator::former()
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
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub subject : String,
  // #[ subformer( the_module::HashMapSubformer ) ]
  #[ subformer( former::HashMapDefinition ) ]
  pub properties : collection_tools::HashMap< K, Property< K > >,
}

// manual
impl< K, Definition > CommandFormer< K, Definition >
where
  K : core::hash::Hash + std::cmp::Eq,
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
  Definition::Types : former::FormerDefinitionTypes< Storage = CommandFormerStorage< K > >,
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

// #[ derive( Debug, PartialEq, the_module::Former ) ]
// pub struct Aggregator< K >
// where
//   K : core::hash::Hash + std::cmp::Eq,
// {
//   pub parameter1 : String,
//   // #[ subformer( the_module::HashMapSubformer ) ]
//   #[ subformer( former::HashMapDefinition ) ]
//   pub commands : collection_tools::HashMap< String, Command< K > >,
// }
//
// // manual
// // impl< K, Context, End >
// // AggregatorFormer< K, Context, End >
// // where
// //   K : core::hash::Hash + std::cmp::Eq,
// //   End : the_module::FormingEnd< Aggregator< K >, Context >,
//
// impl< K, Definition > AggregatorFormer< K, Definition >
// where
//   K : core::hash::Hash + std::cmp::Eq,
//   Definition : former::FormerDefinition,
//   < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePreform,
//   Definition::Types : former::FormerDefinitionTypes< Storage = AggregatorFormerStorage< K > >,
// {
//
//   #[ inline( always ) ]
//   pub fn command< IntoName >( self, name : IntoName )
//   -> CommandFormer< K, Self, impl the_module::FormingEnd< Command< K >, Self > >
//   where
//     K : core::hash::Hash + std::cmp::Eq,
//     IntoName : core::convert::Into< String >,
//   {
//     let on_end = | command : Command< K >, super_former : core::option::Option< Self > | -> Self
//     {
//       let mut super_former = super_former.unwrap();
//       if let Some( ref mut commands ) = super_former.storage.commands
//       {
//         commands.insert( command.name.clone(), command );
//       }
//       else
//       {
//         let mut commands : collection_tools::HashMap< String, Command< K > > = Default::default();
//         commands.insert( command.name.clone(), command );
//         super_former.storage.commands = Some( commands );
//       }
//       super_former
//     };
//     let former = CommandFormer::begin( None, Some( self ), on_end );
//     former.name( name )
//   }
//
// }

// ==

// include!( "./only_test/subformer_basic.rs" );
// xxx : uncomment