use super::*;

use collection_tools::HashMap;

/// A trait for types that behave like hash maps, supporting insertion and custom forming behaviors.
///
/// This trait allows for generic operations on hash map-like data structures, enabling the insertion
/// of key-value pairs and the creation of formers for more complex construction patterns.
///
/// # Type Parameters
/// - `K`: The type of keys stored in the hash map. Must implement `Eq` and `Hash`.
/// - `E`: The type of elements (values) stored in the hash map.
pub trait HashMapLike< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Self : Sized + Default,
{

  /// Inserts a key-value pair into the map.
  fn insert( &mut self, k : K, e : E ) -> Option< E >;

  // /// Return former.
  // #[ inline( always ) ]
  // fn former< Descriptor : FormerDescriptor >( self )
  // ->
  // HashMapSubformer< K, E, Descriptor, (), impl FormingEnd< Self, Self > >
  // {
  //   HashMapSubformer::begin( Some( self ), None, ReturnStorage )
  // }
  // xxx : uncomment and cover by tests

}

impl< K, E > HashMapLike< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Self : Sized + Default,
{

  #[ inline( always ) ]
  fn insert( &mut self, k : K, e : E ) -> Option< E >
  {
    HashMap::insert( self, k, e )
  }

}

//

pub struct HashMapDescriptor< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  _phantom : ::core::marker::PhantomData< ( K, E ) >,
}

impl< K, E > HashMapDescriptor< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn new() -> Self
  {
    Self { _phantom : ::core::marker::PhantomData }
  }
}

impl< K, E > StoragePerform
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Formed = Self;
  fn preform( self ) -> Self::Formed
  {
    self
  }
}

impl< K, E > FormerDescriptor
for HashMapDescriptor< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashMap< K, E >;
  type Formed = HashMap< K, E >;
}

/// A builder for constructing hash map-like structures with a fluent interface.
///
/// `HashMapSubformer` leverages the `HashMapLike` trait to enable a flexible and customizable
/// way to build hash map-like structures. It supports the chaining of insert operations and
/// allows for the definition of custom end actions to finalize the building process.
///
/// # Type Parameters
/// - `K`: Key type, must implement `Eq` and `Hash`.
/// - `E`: Element (value) type.
/// - `Formed`: The hash map-like formed being built.
/// - `Context`: Type of the optional context used during the building process.
/// - `End`: End-of-forming action to be executed upon completion.
///
/// # Examples
/// ```
/// # #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
/// # {
/// # use test_tools::exposed::*;
///
/// #[ derive( Debug, PartialEq, former::Former ) ]
/// pub struct StructWithMap
/// {
///   #[ subformer( former::HashMapSubformer ) ]
///   map : std::collections::HashMap< &'static str, &'static str >,
/// }
///
/// let struct1 = StructWithMap::former()
/// .map()
///   .insert( "a", "b" )
///   .insert( "c", "d" )
///   .end()
/// .form()
/// ;
/// assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
///
/// # }
/// ```

#[ derive( Debug, Default ) ]
pub struct HashMapSubformer< K, E, Descriptor, Context, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  // Formed : HashMapLike< K, E > + ::core::default::Default,
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = ( K, E ) >,
{
  // xxx : rename
  formed : ::core::option::Option< Descriptor::Storage >,
  context : ::core::option::Option< Context >,
  on_end : ::core::option::Option< End >,
  _e_phantom : ::core::marker::PhantomData< E >,
  _k_phantom : ::core::marker::PhantomData< K >,
}

impl< K, E, Descriptor, Context, End >
HashMapSubformer< K, E, Descriptor, Context, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  // Formed : HashMapLike< K, E > + ::core::default::Default,
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = ( K, E ) >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn preform( mut self ) -> Descriptor::Storage
  {
    let formed = if self.formed.is_some()
    {
      self.formed.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };
    formed
    // formed.preform()
  }
  // xxx

  /// Make a new HashMapSubformer. It should be called by a context generated for your structure.
  /// The context is returned after completion of forming by function `on_end``.
  #[ inline( always ) ]
  pub fn begin
  (
    formed : ::core::option::Option< Descriptor::Storage >,
    context : ::core::option::Option< Context >,
    on_end : End,
  )
  -> Self
  {
    Self
    {
      formed,
      context,
      on_end : Some( on_end ),
      _e_phantom : ::core::marker::PhantomData,
      _k_phantom : ::core::marker::PhantomData,
    }
  }

  /// Return context of your struct moving formed there. Should be called after configuring the formed.
  #[ inline( always ) ]
  pub fn form( mut self ) -> Descriptor::Formed
  {
    self.end()
  }

  /// Return context of your struct moving formed there. Should be called after configuring the formed.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Descriptor::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let storage = self.preform();
    on_end.call( storage, context )
  }

  /// Set the whole formed instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, formed : Descriptor::Storage ) -> Self
  {
    self.formed = Some( formed );
    self
  }

}

impl< K, E, Descriptor >
HashMapSubformer< K, E, Descriptor, (), crate::ReturnStorage >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = ( K, E ) >,
  // Formed : HashMapLike< K, E > + ::core::default::Default,
{

  /// Create a new instance without context or on end processing. It just returns continaer on end of forming.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    HashMapSubformer::begin
    (
      None,
      None,
      crate::ReturnStorage,
    )
  }

}

impl< K, E, Descriptor, Context, End >
HashMapSubformer< K, E, Descriptor, Context, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  // Formed : HashMapLike< K, E > + ::core::default::Default,
  End : FormingEnd< Descriptor, Context >,
  Descriptor : FormerDescriptor,
  Descriptor::Storage : ContainerAdd< Element = ( K, E ) >,
{

  /// Inserts a key-value pair into the formed. If the formed doesn't exist, it is created.
  ///
  /// # Parameters
  /// - `k`: The key for the value to be inserted. Will be converted into the formed's key type.
  /// - `e`: The value to be inserted. Will be converted into the formed's value type.
  ///
  /// # Returns
  /// Returns `self` for chaining further insertions or operations.
  ///
  #[ inline( always ) ]
  pub fn insert< K2, E2 >( self, k : K2, e : E2 ) -> Self
  where
    K2 : ::core::convert::Into< K >,
    E2 : ::core::convert::Into< E >,
    // Descriptor::Storage : ContainerAdd< Element = ( K, E ) >,
  {
    if self.formed.is_none()
    {
      self.formed = ::core::option::Option::Some( Default::default() );
    }
    if let ::core::option::Option::Some( ref mut formed ) = self.formed
    {
      ContainerAdd::add( formed, ( k.into(), e.into() ) );
      // formed.insert( k.into(), e.into() );
    }
    self
  }

  /// Alias for insert.
  ///
  /// # Parameters
  /// - `k`: The key for the value to be inserted. Will be converted into the formed's key type.
  /// - `e`: The value to be inserted. Will be converted into the formed's value type.
  ///
  /// # Returns
  /// Returns `self` for chaining further insertions or operations.
  ///
  #[ inline( always ) ]
  pub fn push< K2, E2 >( self, k : K2, e : E2 ) -> Self
  where
    K2 : ::core::convert::Into< K >,
    E2 : ::core::convert::Into< E >,
  {
    self.insert( k, e )
  }

}

//
