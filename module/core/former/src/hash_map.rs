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
    HashMapSubformer::begin( Some( self ), None, ReturnFormed )
  }

  // /// Return former with a custom context.
  // #[ inline( always ) ]
  // fn former_begin< Context, End >( self, context : Context, end : End )
  // -> HashMapSubformer< K, E, Self, Context, End >
  // where End : ToSuperFormer< Self, Context >
  // {
  //   HashMapSubformer::begin( Some( self ), Some( context ), end )
  // }

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
pub struct HashMapSubformer< K, E, Formed, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Formed : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Formed, Context >,
{
  formed : core::option::Option< Formed >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
  _e_phantom : core::marker::PhantomData< E >,
  _k_phantom : core::marker::PhantomData< K >,
}

impl< K, E, Formed, Context, End >
HashMapSubformer< K, E, Formed, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Formed : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Formed, Context >,
{

  /// Form current former into target structure.
  #[ inline( always ) ]
  pub fn form( mut self ) -> Formed
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
  }

  /// Make a new HashMapSubformer. It should be called by a context generated for your structure.
  /// The context is returned after completion of forming by function `on_end``.
  #[ inline( always ) ]
  pub fn begin
  (
    formed : core::option::Option< Formed >,
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    Self
    {
      formed,
      context,
      on_end : Some( on_end ),
      _e_phantom : core::marker::PhantomData,
      _k_phantom : core::marker::PhantomData,
    }
  }

  /// Return context of your struct moving formed there. Should be called after configuring the formed.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let formed = self.form();
    on_end.call( formed, context )
  }

  /// Set the whole formed instead of setting each element individually.
  #[ inline( always ) ]
  pub fn replace( mut self, formed : Formed ) -> Self
  {
    self.formed = Some( formed );
    self
  }

}

// impl< E, Formed > VectorSubformer< E, Formed, Formed, crate::ReturnFormed >
// where
//   Formed : VectorLike< E > + core::default::Default,

impl< K, E, Formed >
HashMapSubformer< K, E, Formed, Formed, crate::ReturnFormed >
where
  K : core::cmp::Eq + core::hash::Hash,
  Formed : HashMapLike< K, E > + core::default::Default,
{

  /// Create a new instance without context or on end processing. It just returns continaer on end of forming.
  #[ inline( always ) ]
  pub fn new() -> Self
  {
    HashMapSubformer::begin
    (
      None,
      None,
      crate::ReturnFormed,
    )
  }

}

impl< K, E, Formed, Context, End >
HashMapSubformer< K, E, Formed, Context, End >
where
  K : core::cmp::Eq + core::hash::Hash,
  Formed : HashMapLike< K, E > + core::default::Default,
  End : ToSuperFormer< Formed, Context >,
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
  pub fn insert< K2, E2 >( mut self, k : K2, e : E2 ) -> Self
  where
    K2 : core::convert::Into< K >,
    E2 : core::convert::Into< E >,
  {
    if self.formed.is_none()
    {
      self.formed = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut formed ) = self.formed
    {
      formed.insert( k.into(), e.into() );
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
    K2 : core::convert::Into< K >,
    E2 : core::convert::Into< E >,
  {
    self.insert( k, e )
  }

}

//
