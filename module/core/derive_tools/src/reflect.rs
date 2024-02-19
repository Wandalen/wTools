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
  pub trait IsContainer : Instance
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
  pub trait IsScalar : Instance
  {
  }

  /// Represents a general-purpose data container that can hold various primitive types
  /// and strings. This enum is designed to encapsulate common data types in a unified
  /// format, simplifying the handling of different types of data in generic contexts.
  ///
  /// # Variants
  ///
  /// - `i8`, `i16`, `i32`, `i64`, `isize`: Signed integer types.
  /// - `u8`, `u16`, `u32`, `u64`, `usize`: Unsigned integer types.
  /// - `f32`, `f64`: Floating-point types.
  /// - `String`: A heap-allocated string (`String`).
  /// - `str`: A borrowed string slice (`&'static str`), typically used for string literals.
  /// - `binary`: A borrowed slice of bytes (`&'static [u8]`), useful for binary data.
  ///
  /// # Examples
  ///
  /// Creating a `Primitive` instance with an integer:
  ///
  /// ```
  /// # use derive_tools::Primitive;
  /// let num = Primitive::i32(42);
  /// ```
  ///
  /// Creating a `Primitive` instance with a string:
  ///
  /// ```
  /// # use derive_tools::Primitive;
  /// let greeting = Primitive::String( "Hello, world!".to_string() );
  /// ```
  ///
  /// Creating a `Primitive` instance with a binary slice:
  ///
  /// ```
  /// # use derive_tools::Primitive;
  /// let bytes = Primitive::binary(&[0xde, 0xad, 0xbe, 0xef]);
  /// ```
  #[ allow( non_camel_case_types ) ]
  #[ derive( Debug, PartialEq, Default ) ]
  pub enum Primitive
  {
    /// None
    #[ default ]
    None,
    /// Represents a signed 8-bit integer.
    i8( i8 ),
    /// Represents a signed 16-bit integer.
    i16( i16 ),
    /// Represents a signed 32-bit integer.
    i32( i32 ),
    /// Represents a signed 64-bit integer.
    i64( i64 ),
    /// Represents a machine-sized signed integer.
    isize( isize ),
    /// Represents an unsigned 8-bit integer.
    u8( u8 ),
    /// Represents an unsigned 16-bit integer.
    u16( u16 ),
    /// Represents an unsigned 32-bit integer.
    u32( u32 ),
    /// Represents an unsigned 64-bit integer.
    u64( u64 ),
    /// Represents a machine-sized unsigned integer.
    usize( usize ),
    /// Represents a 32-bit floating-point number.
    f32( f32 ),
    /// Represents a 64-bit floating-point number.
    f64( f64 ),
    /// Represents a dynamically allocated string.
    String( String ),
    /// Represents a statically allocated string slice.
    str( &'static str ),
    /// Represents a statically allocated slice of bytes.
    binary( &'static [ u8 ] ),
  }

  #[ allow( non_camel_case_types ) ]
  #[ derive( Debug, PartialEq ) ]
  pub enum Data< const N : usize = 0 >
  {
    /// None
    Primitive( Primitive ),
    // /// Array
    // array( &'a [ Data ; N ] ),
  }

  impl< const N : usize > Default for Data< N >
  {
    fn default() -> Self
    {
      Data::Primitive( Primitive::None )
    }
  }

  /// Provides a reflection of an instance that implements the `Instance` trait.
  ///
  /// This function is required to distinguish between instances of a type and references to an instance
  /// in contexts where `self` is used. Without this function, associated trait functions would not differentiate
  /// between `i32` and `&i32`, treating both identically.
  ///
  /// # Arguments
  ///
  /// * `src` - A reference to an instance that implements the `Instance` trait.
  ///
  /// # Returns
  ///
  /// Returns an entity descriptor that implements the `Entity` trait, providing
  /// runtime reflection capabilities for the given instance.
  pub fn reflect( src : &impl Instance ) -> impl Entity
  {
    src._reflect()
  }

  ///
  /// Represents a trait for enabling runtime reflection of entities.
  ///
  /// This trait is designed to equip implementing structs with the ability to introspect
  /// their properties, type names, and any contained elements. It facilitates runtime inspection
  /// and manipulation of entities in a dynamic manner.
  ///
  pub trait Instance
  {
    /// The entity descriptor associated with this instance.
    type Entity : Entity;
    /// Returns a descriptor for the current instance.
    ///
    /// Don't use manually.
    fn _reflect( &self ) -> Self::Entity
    {
      Self::Reflect()
    }
    /// Returns a descriptor for the type of the instance.
    #[ allow( non_snake_case ) ]
    fn Reflect() -> Self::Entity;
  }

  impl< T > Instance for T
  where
    EntityDescriptor< T > : Entity,
    T : InstanceMarker,
  {
    type Entity = EntityDescriptor::< Self >;
    #[ inline( always ) ]
    fn Reflect() -> Self::Entity
    {
      EntityDescriptor::< Self >::new()
    }
  }

  ///
  /// Type descriptor
  ///
  #[ derive( PartialEq, Default ) ]
  pub struct EntityDescriptor< I : Instance >
  {
    _phantom : core::marker::PhantomData< I >,
  }

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
  pub trait InstanceMarker {}

  impl< T > Entity for EntityDescriptor< T >
  where
    T : InstanceMarker + 'static,
  {
    #[ inline( always ) ]
    fn type_name( &self ) -> &'static str
    {
      core::any::type_name::< T >()
    }
    #[ inline( always ) ]
    fn type_id( &self ) -> core::any::TypeId
    {
      core::any::TypeId::of::< T >()
    }
  }

  impl< T > std::fmt::Debug for EntityDescriptor< T >
  where
    T : Instance + 'static,
    EntityDescriptor< T > : Entity,
  {
    fn fmt( &self, f: &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f
      .write_str( &format!( "{}#{:?}", self.type_name(), self.type_id() ) )
    }
  }

  ///
  /// Type descriptor
  ///
  pub trait Entity : core::fmt::Debug
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

    /// Retrieves the type name.
    ///
    /// # Returns
    ///
    /// Returns the type name of the implementing entity as a static string slice.
    ///
    /// This method leverages Rust's `type_name` function to provide the name at runtime,
    /// aiding in debugging and logging purposes.
    fn type_name( &self ) -> &'static str;

    /// Retrives the typ id.
    fn type_id( &self ) -> core::any::TypeId;

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

    /// Returns a descriptor for the type of the instance.
    ///
    /// # Returns
    ///
    /// Returns an entity descriptor that implements the `Entity` trait.
    #[ inline( always ) ]
    fn element( &self, i : usize ) -> KeyVal
    {
      debug_assert!( i < self.len() );
      self.elements().skip( i ).next().unwrap()
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
  // #[ derive( Default ) ]
  pub struct KeyVal
  {
    /// The key associated with the value in the key-value pair.
    pub key : Primitive,
    // pub key : &'static str,
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
      .field( "val", &format_args!( "{:?}", &self.val ) )
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

  impl InstanceMarker for i8 {}
  impl InstanceMarker for i16 {}
  impl InstanceMarker for i32 {}
  impl InstanceMarker for i64 {}
  impl InstanceMarker for u8 {}
  impl InstanceMarker for u16 {}
  impl InstanceMarker for u32 {}
  impl InstanceMarker for u64 {}
  impl InstanceMarker for f32 {}
  impl InstanceMarker for f64 {}
  impl InstanceMarker for String {}
  impl InstanceMarker for &'static str {}

  impl< T > InstanceMarker for &T
  where T : InstanceMarker
  {}

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

  // qqq : xxx : implement for slice, Vec, HashMap, HashSet

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
//       let mut result : [ KeyVal ; N ] = Default::default();
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
  pub use super::private::
  {
    reflect,
    Primitive,
    IsContainer,
    IsScalar,
    Instance,
    InstanceMarker,
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
