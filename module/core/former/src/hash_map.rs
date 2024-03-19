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
  K : core::cmp::Eq + core::hash::Hash,
  Self : Sized + Default,
{

  /// Inserts a key-value pair into the map.
  fn insert( &mut self, k : K, e : E ) -> Option< E >;

  /// Return former.
  #[ inline( always ) ]
  fn former( self )
  -> HashMapSubformer< K, E, Self, Self, impl ToSuperFormer< Self, Self > >
  {
    HashMapSubformer::begin( Some( self ), None, ReturnContainer )
  }

  /// Return former with a custom context.
  #[ inline( always ) ]
  fn former_begin< Context, End >( self, context : Context, end : End )
  -> HashMapSubformer< K, E, Self, Context, End >
  where End : ToSuperFormer< Self, Context >
  {
    HashMapSubformer::begin( Some( context ), Some( self ), end )
  }

}

impl< K, E > HashMapLike< K, E > for HashMap< K, E >
where
  K : core::cmp::Eq + core::hash::Hash,
  Self : Sized + Default,
{

  #[ inline( always ) ]
  fn insert( &mut self, k : K, e : E ) -> Option< E >
  {
    HashMap::insert( self, k, e )
  }

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
/// - `Container`: The hash map-like container being built.
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
pub struct HashMapSubformer< K, E, Container, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Container, Context >,
{
  container : core::option::Option< Container >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, Container, Context, End >
HashMapSubformer< K, E, Container, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Container, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn form( mut self ) -> Container
  {
    let container = if self.container.is_some()
    {
      self.container.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };
    container
  }

  /// Make a new HashMapSubformer. It should be called by a context generated for your structure.
  /// The context is returned after completion of forming by function `on_end``.
  #[ inline( always ) ]
  pub fn begin
  (
    context : core::option::Option< Context >,
    container : core::option::Option< Container >,
    on_end : End,
  ) -> Self
  {
    Self
    {
      context,
      container,
      on_end : Some( on_end ),
      _e_phantom : core::marker::PhantomData,
      _k_phantom : core::marker::PhantomData,
    }
  }

  /// Return context of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let container = self.form();
    on_end.call( container, context )
  }

  /// Set the whole container instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, container : Container ) -> Self
  {
    self.container = Some( container );
    self
  }

}

// impl< E, Container > VectorSubformer< E, Container, Container, crate::ReturnContainer >
// where
//   Container : VectorLike< E > + core::default::Default,

impl< K, E, Container >
HashMapSubformer< K, E, Container, Container, crate::ReturnContainer >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
{

  /// Create a new instance without context or on end processing. It just returns continaer on end of forming.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    HashMapSubformer::begin
    (
      None,
      None,
      crate::ReturnContainer,
    )
  }

}

impl< K, E, Container, Context, End >
HashMapSubformer< K, E, Container, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Container : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Container, Context >,
{

  /// Inserts a key-value pair into the container. If the container doesn't exist, it is created.
  ///
  /// # Parameters
  /// - `k`: The key for the value to be inserted. Will be converted into the container's key type.
  /// - `e`: The value to be inserted. Will be converted into the container's value type.
  ///
  /// # Returns
  /// Returns `self` for chaining further insertions or operations.
  ///
  #[ inline( always ) ]
  pub fn insert< K2, E2 >( mut self, k : K2, e : E2 ) -> Self
  where
    K2 : core::convert::Into< K >,
    E2 : core::convert::Into< E >,
  {
    if self.container.is_none()
    {
      self.container = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut container ) = self.container
    {
      container.insert( k.into(), e.into() );
    }
    self
  }

  /// Alias for insert.
  ///
  /// # Parameters
  /// - `k`: The key for the value to be inserted. Will be converted into the container's key type.
  /// - `e`: The value to be inserted. Will be converted into the container's value type.
  ///
  /// # Returns
  /// Returns `self` for chaining further insertions or operations.
  ///
  #[ inline( always ) ]
  pub fn push< K2, E2 >( self, k : K2, e : E2 ) -> Self
  where
    K2 : core::convert::Into< K >,
    E2 : core::convert::Into< E >,
  {
    self.insert( k, e )
  }

}

//
