//!
//! Mechanism for reflection.
//!

use super::*;

/// Internal namespace.
pub( crate ) mod private
{
  use super::*;

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
  /// The `Entity` trait defines a common interface for entities within a system, enabling
  /// runtime reflection, inspection, and manipulation of their properties and elements. It
  /// serves as a foundational component for dynamic entity handling, where entities can
  /// represent data structures, components, or other logical units with introspectable
  /// and manipulable state.
  ///
  /// ## Usage
  ///
  /// Implementing the `Entity` trait allows a type to be integrated into systems that require
  /// dynamic type inspection and manipulation, such as serialization frameworks, object-relational
  /// mapping (ORM) systems, or generic containers and algorithms that operate on heterogeneous
  /// entity collections.
  ///
  /// ## Key Concepts
  ///
  /// - **Containment**: Entities can act as containers for other entities, enabling hierarchical
  ///   or composite data models.
  ///
  /// - **Ordering**: The trait distinguishes between ordered and unordered entities, affecting
  ///   how their elements are iterated over or accessed.
  ///
  /// - **Reflection**: Through type metadata and element access methods, entities support
  ///   reflection, allowing programmatic querying and manipulation of their structure and state.
  ///
  /// ## Implementing `Entity`
  ///
  /// To implement the `Entity` trait, a type must provide implementations for all non-default
  /// methods (`type_name`, `type_id`). The default method implementations assume non-container
  /// entities with no elements and predictable ordering. Implementers should override these
  /// defaults as appropriate to accurately reflect their specific semantics and behavior.
  ///
  /// ## Example
  ///
  /// ```
  /// # use derive_tools::reflect::Entity;
  ///
  /// #[derive(Debug)]
  /// struct MyEntity
  /// {
  ///   // Entity fields
  /// }
  ///
  /// impl Entity for MyEntity
  /// {
  ///
  ///   #[ inline ]
  ///   fn type_name( &self ) -> &'static str
  ///   {
  ///     "MyEntity"
  ///   }
  ///
  ///   #[ inline ]
  ///   fn type_id(&self) -> core::any::TypeId
  ///   {
  ///     core::any::TypeId::of::< MyEntity >()
  ///   }
  ///
  ///   // Additional method implementations as necessary...
  /// }
  /// ```
  ///
  /// This trait is designed to be flexible and extensible, accommodating a wide variety of entity
  /// types and use cases. Implementers are encouraged to leverage Rust's type system and trait
  /// mechanisms to provide rich, dynamic behavior in a type-safe manner.
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

    /// Determines if the elements of the container are maintained in a specific order.
    ///
    /// This method indicates whether the container preserves a specific order of its elements.
    /// The concept of "order" can refer to:
    /// - **Sorted Order**: Where elements are arranged based on a sorting criterion, typically
    ///   through comparison operations.
    /// - **Insertion Order**: Where elements retain the order in which they were added to the container.
    ///
    /// It is important to distinguish this property in collections to understand how iteration over
    /// the elements will proceed and what expectations can be held about the sequence of elements
    /// when accessed.
    ///
    /// # Returns
    ///
    /// - `true` if the container maintains its elements in a predictable order. This is typically
    ///   true for data structures like arrays, slices, and vectors, where elements are accessed
    ///   sequentially or are sorted based on inherent or specified criteria.
    /// - `false` for collections where the arrangement of elements does not follow a predictable
    ///   sequence from the perspective of an observer, such as sets and maps implemented via hashing.
    ///   In these structures, the order of elements is determined by their hash and internal state,
    ///   rather than the order of insertion or sorting.
    ///
    /// By default, this method returns `true`, assuming that the entity behaves like an array, slice,
    /// or vector, where the order of elements is consistent and predictable. Implementers should override
    /// this behavior for collections where element order is not maintained or is irrelevant.
    #[ inline( always ) ]
    fn is_ordered( &self ) -> bool
    {
      true
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

  ///
  /// Type descriptor
  ///
  #[ derive( PartialEq, Default, Copy, Clone ) ]
  pub struct EntityDescriptor< I : Instance >
  {
    /// Container description.
    pub container_info : Option< usize >,
    _phantom : core::marker::PhantomData< I >,
  }

  impl< I : Instance > EntityDescriptor< I >
  {
    /// Constructor of the descriptor.
    #[ inline( always ) ]
    pub fn new() -> Self
    {
      let _phantom = core::marker::PhantomData::< I >;
      Self { _phantom, container_info : None }
    }

    /// Constructor of the descriptor of container type.
    pub fn new_container( size : usize ) -> Self
    {
      let _phantom = core::marker::PhantomData::< I >;
      Self { _phantom, container_info : Some( size ) }
    }
  }

  /// Auto-implement descriptor for this type.
  trait InstanceMarker {}

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
    pub key : primitive::Primitive,
    // pub key : &'static str,
    /// The value associated with the key in the key-value pair.
    pub val : Box< dyn Entity >,
  }

  impl Default for KeyVal
  {
    fn default() -> Self
    {
      Self
      {
        key : primitive::Primitive::default(),
        val : Box::new( EntityDescriptor::< i8 >::new() ) as Box::< dyn Entity >,
      }
    }
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

  impl< T : Instance + 'static, const N : usize > IsContainer for [ T ; N ] {}
  impl< T : Instance > IsContainer for &'static [ T ]
  {

  }
  impl< T : Instance + 'static > IsContainer for Vec< T >
  {

  }
  impl< K : IsScalar + 'static, V : Instance + 'static > IsContainer for std::collections::HashMap< K, V >
  {

  }
  impl< V : Instance + 'static > IsContainer for std::collections::HashSet< V > {}

  // qqq : xxx : implement for slice
  // qqq : xxx : implement for Vec
  // qqq : xxx : implement for HashMap
  // qqq : xxx : implement for HashSet

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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    // reflect,
    IsContainer,
    IsScalar,
    Instance,
    // InstanceMarker,
    Entity,
    EntityDescriptor,
    KeyVal,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    reflect,
  };
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
