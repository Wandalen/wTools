//!
//! Types, which are extension of std.
//!

/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Trait indicating that an entity is a container.
  ///
  /// Implementors of `IsContainer` are considered to be container types,
  /// which can hold zero or more elements. This trait is typically used in
  /// conjunction with reflection mechanisms to dynamically inspect, access,
  /// or modify the contents of a container at runtime.
  pub trait IsContainer
  {
  }

  ///
  /// Trait indicating that an entity is a scalar value.
  ///
  /// Implementors of `IsScalar` are considered to be scalar types,
  /// representing single, indivisible values as opposed to composite entities
  /// like arrays or structs. This distinction can be useful in reflection-based
  /// APIs or generic programming to treat scalar values differently from containers
  /// or other complex types.
  pub trait IsScalar
  {
  }

  ///
  /// Represents a trait for entity reflection.
  ///
  /// This trait is designed to provide reflection capabilities to the implementing struct,
  /// allowing runtime inspection of its properties, type name, and contained elements if any.
  // pub trait Instance : core::any::Any
  pub trait Instance
  {
    /// Entity descriptor.
    type Entity : Entity;
    /// Return a descriptor of type with current instance.
    fn reflect( &self ) -> Self::Entity
    {
      Self::Reflect()
    }
    /// Return a descriptor of type with type of instance.
    fn Reflect() -> Self::Entity;
  }

  impl< T > Instance for T
  where
    EntityDescriptor< T > : Entity,
    T : AutoInstance,
  {
    type Entity = EntityDescriptor::< Self >;
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      EntityDescriptor::< Self >::new()
    }
  }

  // /// xxx
  // pub trait AnyInstance : core::any::Any + Instance {}
  // impl< T : core::any::Any + Instance > AnyInstance for T {}

  ///
  /// Type descriptor
  ///
  #[ derive( PartialEq, Debug ) ]
  pub struct EntityDescriptor< I : Instance >
  {
    _phantom : core::marker::PhantomData< I >,
  }

  // xxx : qqq : qqq for Yulia : implement derive Phantom
  // #[ derive( PartialEq, Debug, Phantom ) ]
  // pub struct EntityDescriptor< I : Instance >;

  impl< I : Instance > EntityDescriptor< I >
  {
    /// Constructor of the descriptor.
    #[ inline( always ) ]
    pub fn new() -> Self
    {
      let _phantom = core::marker::PhantomData::< I >;
      Self { _phantom }
    }
  }

  /// Auto-implement descriptor for this type.
  pub trait AutoInstance {}

  impl< T > Entity for EntityDescriptor< T >
  where
    T : AutoInstance,
  {
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< T >()
    }
  }

  ///
  /// Type descriptor
  ///
  // pub trait Entity< I : Instance > : core::any::Any
  pub trait Entity
  {

    /// Determines if the entity acts as a container for other entities.
    ///
    /// # Returns
    ///
    /// Returns `true` if the entity can contain other entities (like a struct, vector, etc.),
    /// otherwise `false`.
    ///
    /// By default, this method returns `false`, assuming that the entity does not act as a container.
    #[ inline( always ) ]
    fn is_container( &self ) -> bool
    {
      false
    }

    /// Returns the number of elements contained in the entity.
    ///
    /// # Returns
    ///
    /// Returns the count of elements if the entity is a container, otherwise `0`.
    ///
    /// This method is particularly useful for collections or composite entities.
    /// By default, this method returns `0`, assuming the entity contains no elements.
    #[ inline( always ) ]
    fn len( &self ) -> usize
    {
      0
    }

    /// Retrieves the type name of the entity.
    ///
    /// # Returns
    ///
    /// Returns the type name of the implementing entity as a static string slice.
    ///
    /// This method leverages Rust's `type_name` function to provide the name at runtime,
    /// aiding in debugging and logging purposes.
    fn type_name( &self ) -> &'static str;

    /// Provides an iterator over the elements contained within the entity, if any.
    ///
    /// # Returns
    ///
    /// Returns a boxed iterator over `KeyVal` pairs representing the key-value mappings
    /// of the entity's elements. For non-container entities, an empty iterator is returned.
    ///
    /// This method is crucial for traversing composite entities or collections at runtime,
    /// allowing for dynamic inspection and manipulation.
    #[ inline( always ) ]
    fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > >
    {
      Box::new( [].into_iter() )
    }

  }

  // /// A trait for entities that support dynamic type inspection and reflection.
  // ///
  // /// This trait extends both `core::any::Any` for type checking and downcasting capabilities,
  // /// and `Entity` for reflection-based operations, enabling runtime inspection of
  // /// entity properties and structures.
  // pub trait AnyEntity : core::any::Any + Entity {}
  // impl< T : core::any::Any + Entity > AnyEntity for T {}

  /// Represents a key-value pair where the key is a static string slice
  /// and the value is a boxed entity that implements the `AnyEntity` trait.
  ///
  /// This struct is typically used in the context of reflecting over the properties
  /// or members of a container entity, allowing for dynamic access and inspection
  /// of its contents.
  ///
  // #[ derive( PartialEq, Debug ) ]
  pub struct KeyVal
  {
    /// The key associated with the value in the key-value pair.
    pub key : &'static str,
    /// The value associated with the key in the key-value pair.
    pub val : Box< dyn Entity >,
  }

  impl std::fmt::Debug for KeyVal
  {
    fn fmt( &self, f: &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f
      .debug_struct( "KeyVal" )
      .field( "key", &self.key )
      // .field( "val", &format_args!( "{}", core::any::type_name::< dyn AnyEntity< I = I > >() ) )
      // xxx
      .finish()
    }
  }

  impl PartialEq for KeyVal
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self.key == other.key
      // qqq : compare also by val
    }
  }

  // impl Instance for i8 {}
  // impl Instance for i16 {}
  // impl Instance for i32 {}
  // impl Instance for i64 {}
  // impl Instance for u8 {}
  // impl Instance for u16 {}
  // impl Instance for u32 {}
  // impl Instance for u64 {}
  // impl Instance for f32 {}
  // impl Instance for f64 {}
  // impl Instance for String {}
  // impl Instance for &'static str {}

  /// Implements Entity for a types.
  #[ macro_export ]
  macro_rules! impl_entity_for
  {

    (
      $( $Path : tt )*
    )
    =>
    {
      impl crate::reflect::Entity for crate::reflect::EntityDescriptor< $( $Path )* >
      {
        #[ inline( always ) ]
        fn type_name( &self ) -> &'static str
        {
          core::any::type_name::< $( $Path )* >()
        }
      }
    };

  }

  // impl Entity for EntityDescriptor< i8 >
  // {
  //   fn type_name( &self ) -> &'static str
  //   {
  //     core::any::type_name::< i8 >()
  //   }
  // }

  // impl_entity_for!( i8 );
  // impl_entity_for!( i16 );
  // impl_entity_for!( i32 );
  // impl_entity_for!( i64 );
  // impl_entity_for!( u8 );
  // impl_entity_for!( u16 );
  // impl_entity_for!( u32 );
  // impl_entity_for!( u64 );
  // impl_entity_for!( f32 );
  // impl_entity_for!( f64 );
  // impl_entity_for!( String );
  // impl_entity_for!( &'static str );

  impl AutoInstance for i8 {}
  impl AutoInstance for i16 {}
  impl AutoInstance for i32 {}
  impl AutoInstance for i64 {}
  impl AutoInstance for u8 {}
  impl AutoInstance for u16 {}
  impl AutoInstance for u32 {}
  impl AutoInstance for u64 {}
  impl AutoInstance for f32 {}
  impl AutoInstance for f64 {}
  impl AutoInstance for String {}
  impl AutoInstance for &'static str {}

  impl IsScalar for i8 {}
  impl IsScalar for i16 {}
  impl IsScalar for i32 {}
  impl IsScalar for i64 {}
  impl IsScalar for u8 {}
  impl IsScalar for u16 {}
  impl IsScalar for u32 {}
  impl IsScalar for u64 {}
  impl IsScalar for f32 {}
  impl IsScalar for f64 {}
  impl IsScalar for String {}
  impl IsScalar for &'static str {}

  impl< T, const N : usize > IsContainer for [ T ; N ] {}

//   impl< T : Entity, const N : usize > Entity for [ T ; N ]
//   {
//
//     #[ inline( always ) ]
//     fn is_container( &self ) -> bool
//     {
//       true
//     }
//
//     #[ inline( always ) ]
//     fn len( &self ) -> usize
//     {
//       N
//     }
//
//     #[ inline( always ) ]
//     fn elements( &self ) -> Box< dyn Iterator< Item = KeyVal > + '_ >
//     {
//
//       let result : [ KeyVal ; N ];
//       for ( k, e ) in self.iter().enumerate()
//       {
//         result[ k ] = KeyVal { key : "x", val : Box::new( (*e).clone() ) }
//       }
//
//       Box::new( result.into_iter() )
//     }
//
//   }

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
  pub use super::private::
  {
    IsContainer,
    IsScalar,
    Instance,
    AutoInstance,
    EntityDescriptor,
    Entity,
    KeyVal,
  };
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
