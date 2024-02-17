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

  // ///
  // /// Represents a trait for entity reflection.
  // ///
  // /// This trait is designed to provide reflection capabilities to the implementing struct,
  // /// allowing runtime inspection of its properties, type name, and contained elements if any.
  // pub trait Instance : core::any::Any
  // {
  //   fn entity ->
  // }

  ///
  /// Represents a trait for entity reflection.
  ///
  /// This trait is designed to provide reflection capabilities to the implementing struct,
  /// allowing runtime inspection of its properties, type name, and contained elements if any.
  pub trait Entity : core::any::Any
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
    fn reflect_is_container( &self ) -> bool
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
    fn reflect_len( &self ) -> usize
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
    #[ inline( always ) ]
    fn reflect_type_name( &self ) -> &'static str
    {
      core::any::type_name::< Self >()
    }

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
    fn reflect_elements( &self ) -> Box< dyn Iterator< Item = KeyVal > + '_ >
    {
      // std::iter::empty()
      Box::new( [].into_iter() )
    }

  }

  /// A trait for entities that support dynamic type inspection and reflection.
  ///
  /// This trait extends both `core::any::Any` for type checking and downcasting capabilities,
  /// and `Entity` for reflection-based operations, enabling runtime inspection of
  /// entity properties and structures.
  pub trait AnyEntity : core::any::Any + Entity {}
  impl< T : core::any::Any + Entity > AnyEntity for T {}

  /// Represents a key-value pair where the key is a static string slice
  /// and the value is a boxed entity that implements the `AnyEntity` trait.
  ///
  /// This struct is typically used in the context of reflecting over the properties
  /// or members of a container entity, allowing for dynamic access and inspection
  /// of its contents.
  ///
  /// # Examples
  ///
  /// ```
  /// # use derive_tools::reflect::{ KeyVal, AnyEntity, Entity };
  /// # struct MyEntity;
  /// # impl Entity for MyEntity {}
  /// // Assuming `MyEntity` implements `AnyEntity`
  /// let my_entity = MyEntity;
  /// let reflect_kv = KeyVal
  /// {
  ///   key : "my_entity",
  ///   val : Box::new( my_entity ),
  /// };
  ///
  /// assert_eq!(reflect_kv.key, "my_entity");
  /// // Further operations can be performed on `reflect_kv.val` after downcasting it
  /// // to its concrete type.
  /// ```
  // #[ derive( PartialEq, Debug ) ]
  pub struct KeyVal
  {
    /// The key associated with the value in the key-value pair.
    ///
    /// This is a static string slice, implying that the key is known at compile time
    /// and does not change at runtime.
    pub key : &'static str,
    /// The value associated with the key in the key-value pair.
    ///
    /// It is boxed to allow for dynamic typing, enabling the storage of any entity
    /// that implements the `AnyEntity` trait. This facilitates runtime reflection
    /// and type inspection of the contained entity.
    pub val : Box< dyn AnyEntity >,
  }

  impl std::fmt::Debug for KeyVal
  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
      f
      .debug_struct( "KeyVal" )
      .field( "key", &self.key )
      .field( "val", &format_args!( "{}", core::any::type_name::< dyn AnyEntity >() ) )
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

  impl Entity for i8 {}
  impl Entity for i16 {}
  impl Entity for i32 {}
  impl Entity for i64 {}
  impl Entity for u8 {}
  impl Entity for u16 {}
  impl Entity for u32 {}
  impl Entity for u64 {}
  impl Entity for f32 {}
  impl Entity for f64 {}
  impl Entity for String {}
  impl Entity for &'static str {}

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
//     fn reflect_is_container( &self ) -> bool
//     {
//       true
//     }
//
//     #[ inline( always ) ]
//     fn reflect_len( &self ) -> usize
//     {
//       N
//     }
//
//     #[ inline( always ) ]
//     fn reflect_elements( &self ) -> Box< dyn Iterator< Item = KeyVal > + '_ >
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
    Entity,
    KeyVal,
    AnyEntity,
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
