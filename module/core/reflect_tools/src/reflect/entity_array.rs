//!
//! Implementation of Entity for an array.
//!

use super::*;

/// Internal namespace.
pub mod private
{
  use super::*;

  impl< T, const N : usize > Instance for [ T ; N ]
  where
    EntityDescriptor< [ T ; N ] > : Entity,
  {
    type Entity = EntityDescriptor::< Self >;
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      EntityDescriptor::< Self >::new()
    }
  }

  impl< T, const N : usize > Entity for EntityDescriptor< [ T ; N ] >
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
      N
    }

    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< [ T ; N ] >()
    }

    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< [ T ; N ] >()
    }

    #[ inline( always ) ]
    fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > >
    {

      // qqq : write optimal implementation
     // let mut result : [ KeyVal ; N ] = [ KeyVal::default() ; N ];
//
//       for i in 0..N
//       {
//         result[ i ] = KeyVal { key : "x", val : Box::new( < T as Instance >::Reflect() ) }
//       }

      let result : Vec< KeyVal > = ( 0 .. N )
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
