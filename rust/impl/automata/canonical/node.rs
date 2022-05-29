/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  // use std::collections::HashSet;
  use indexmap::IndexSet;
  use std::fmt;

  ///
  /// Canonical implementation of node.
  ///

  pub struct Node< Kind = crate::NodeKindless >
  where
    Kind : NodeKindInterface,
  {
    /// Input node.
    pub out_nodes : IndexSet< < Self as HasId >::Id >,
    /// Kind of the node.
    pub kind : Kind,
    /// Name.
    pub name : < Self as HasId >::Id,
  }

  //

  impl Node
  {

    /// Construct a name instance of the node.
    pub fn make_named< Name >( name : Name ) ->Self
    where
      Name : Into< < Self as HasId >::Id >,
    {
      let out_nodes = IndexSet::new();
      let kind = Default::default();
      Self
      {
        out_nodes,
        kind,
        name : name.into(),
      }
    }

  }

  //

  impl< Kind > HasId
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {

    type Id = crate::IdentityWithInt;

    fn id( &self ) -> Self::Id
    {
      self.name
    }

  }

  //

  // impl< Id, Kind > HasId
  // for Node< Id, Kind >
  // where
  //   Kind : NodeKindInterface,
  //   Id : IdentityInterface,
  // {
  //   type Id = Id;
  //   fn id( &self ) -> Self::Id
  //   {
  //     self.name
  //   }
  // }

  //

  impl< Kind > NodeBasicInterface
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
  }

  //

  impl Extend< < Self as HasId >::Id >
  for Node
  {
    fn extend< Iter >( &mut self, iter : Iter )
    where
      Iter : IntoIterator< Item = < Self as HasId >::Id >
    {
      for node in iter
      {
        self.out_nodes.insert( node );
      }
    }
  }

  //

  impl< Kind > fmt::Debug
  for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "node::{:?}", self.id() ) )?;
      for e in &self.out_nodes
      {
        f.write_fmt( format_args!( "\n - {:?}", e ) )?;
      }
      f.write_fmt( format_args!( "" ) )
    }
  }

  //

  impl< Kind > PartialEq for Node< Kind >
  where
    Kind : NodeKindInterface,
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.id() == other.id()
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
  pub use super::private::{ Node };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
