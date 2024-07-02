//!
//! Implementation of Entity for a HashMap.
//!

use super::*;

/// Internal namespace.
pub mod private
{
  use super::*;
  // qqq : xxx : implement for HashMap
  // aaa : added implementation of Instance trait for HashMap
  use std::collections::HashMap;
  impl< K, V > Instance for HashMap< K, V >
  where
    KeyedCollectionDescriptor< HashMap< K, V > > : Entity,
    primitive::Primitive : From< K >,
    K : Clone,
  {
    type Entity = KeyedCollectionDescriptor::< HashMap< K, V > >;
    fn _reflect( &self ) -> Self::Entity
    {
      KeyedCollectionDescriptor::< Self >::new
      (
        self.len(),
        self.keys().into_iter().map( | k | primitive::Primitive::from( k.clone() ) ).collect::< Vec< _ > >(),
      )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      KeyedCollectionDescriptor::< Self >::new( 0, Vec::new() )
    }
  }

  impl< K, V > Entity for KeyedCollectionDescriptor< HashMap< K, V > >
  where
    K : 'static + Instance + IsScalar + Clone,
    primitive::Primitive : From< K >,
    V : 'static + Instance,
  {
    #[ inline( always ) ]
    fn is_container( &self ) -> bool
    {
      true
    }

    #[ inline( always ) ]
    fn len( &self ) -> usize
    {
      self.len
    }

    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< HashMap< K, V > >()
    }

    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< HashMap< K, V > >()
    }

    #[ inline( always ) ]
    fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > >
    {
      let mut result : Vec< KeyVal > = ( 0 .. self.len() )
      .map( | k | KeyVal { key : Primitive::usize( k ), val : Box::new( < V as Instance >::Reflect() ) } )
      .collect();

      for i in 0..self.len()
      {
          result[ i ] = KeyVal { key : self.keys[ i ].clone(), val : Box::new( < V as Instance >::Reflect() ) }
      }

      Box::new( result.into_iter() )
    }
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
  // pub use private::
  // {
  // };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
