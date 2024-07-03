//!
//! Implementation of Entity for a slice.
//!

use super::*;

/// Internal namespace.
pub mod private
{
  use super::*;

  // qqq : xxx : implement for slice
  // aaa : added implementation of Instance trait for slice
  impl< T > Instance for &'static [ T ]
  where
    CollectionDescriptor< &'static [ T ] > : Entity,
  {
    type Entity = CollectionDescriptor::< &'static [ T ] >;
    fn _reflect( &self ) -> Self::Entity
    {
      CollectionDescriptor::< Self >::new( self.len() )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      CollectionDescriptor::< Self >::new( 1 )
    }
  }

  impl< T > Entity for CollectionDescriptor< &'static [ T ] >
  where
    T : 'static + Instance,
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
      core::any::type_name::< &'static [ T ] >()
    }

    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< &'static [ T ] >()
    }

    #[ inline( always ) ]
    fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > >
    {

      let result : Vec< KeyVal > = ( 0 .. self.len() )
      .map( | k | KeyVal { key : Primitive::usize( k ), val : Box::new( < T as Instance >::Reflect() ) } )
      .collect();

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