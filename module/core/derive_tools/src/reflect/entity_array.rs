//!
//! Implementation of Entity for an array.
//!

use super::*;

/// Internal namespace.
pub mod private
{
  use super::*;

  // aaa : implementation for slice
  impl< T > Instance for &'static [ T ]
  where
    EntityDescriptor< &'static [ T ] > : Entity,
  {
    type Entity = EntityDescriptor::< &'static [ T ] >;
    fn _reflect( &self ) -> Self::Entity
    {
      EntityDescriptor::< Self >::new_container( self.len() )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      EntityDescriptor::< Self >::new()
    }
  }

  impl< T > Entity for EntityDescriptor< &'static [ T ] >
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
      if let Some( len ) = self.container_info
      {
        len
      }
      else
      {
        0  
      }
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

  // aaa : implementation for Vec
  impl< T > Instance for Vec< T >
  where
    EntityDescriptor< Vec< T > > : Entity,
  {
    type Entity = EntityDescriptor::< Vec< T > >;
    fn _reflect( &self ) -> Self::Entity
    {
      EntityDescriptor::< Self >::new_container( self.len() )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      EntityDescriptor::< Self >::new()
    }
  }
  
  impl< T > Entity for EntityDescriptor< Vec< T > >
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
      if let Some( len ) = self.container_info
      {
        len
      }
      else
      {
        0  
      }
    }

    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< Vec< T > >()
    }

    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< Vec< T > >()
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

  // qqq : implementation for HashMap keys not finished
  use std::collections::HashMap;
  impl< K, V > Instance for HashMap< K, V >
  where
    EntityDescriptor< HashMap< K, V > > : Entity,
  {
    type Entity = EntityDescriptor::< HashMap< K, V > >;
    fn _reflect( &self ) -> Self::Entity
    {
      EntityDescriptor::< Self >::new_container( self.len() )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {

      EntityDescriptor::< Self >::new()
    }
  }
  
  impl< K, V > Entity for EntityDescriptor< HashMap< K, V > >
  where
    K : 'static + Instance + IsScalar,
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
      if let Some( len ) = self.container_info
      {
        len
      }
      else
      {
        0  
      }
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
      //let primitive = k.into();

      let result : Vec< KeyVal > = ( 0..self.len() )
      .map( | k | KeyVal { key : Primitive::usize( k ), val : Box::new( < V as Instance >::Reflect() ) } )
      .collect();

      Box::new( result.into_iter() )
    }
  }

  // aaa : implementation for HashSet
  use std::collections::HashSet;
  impl< T > Instance for HashSet< T >
  where
    EntityDescriptor< HashSet< T > > : Entity,
  {
    type Entity = EntityDescriptor::< HashSet< T > >;
    fn _reflect( &self ) -> Self::Entity
    {
      EntityDescriptor::< Self >::new_container( self.len() )
    }
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {

      EntityDescriptor::< Self >::new()
    }
  }
  
  impl< T > Entity for EntityDescriptor< HashSet< T > >
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
      if let Some( len ) = self.container_info
      {
        len
      }
      else
      {
        0  
      }
    }

    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< HashSet< T > >()
    }

    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< HashSet< T > >()
    }

    #[ inline( always ) ]
    fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > >
    {
      let result : Vec< KeyVal > = ( 0..self.len() )
      .map( | k | KeyVal { key : Primitive::usize( k ), val : Box::new( < T as Instance >::Reflect() ) } )
      .collect();

      Box::new( result.into_iter() )
    }
  }

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
      //let mut result : [ KeyVal ; N ] = [ KeyVal::default() ; N ];
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
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  // pub use super::private::
  // {
  // };
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
