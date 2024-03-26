#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/collection_tools/latest/collection_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  #[ cfg( all( feature = "collection_std", feature = "use_alloc" ) ) ]
  pub use ::hashbrown;
  #[ cfg( all( not( feature = "no_std" ), feature = "collection_constructors" ) ) ]
  pub use ::literally;

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;

  #[ cfg( feature = "use_alloc" ) ]
  extern crate alloc;
  #[ cfg( feature = "use_alloc" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use alloc::vec;
  #[ cfg( feature = "use_alloc" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use alloc::vec::Vec;
  #[ cfg( feature = "use_alloc" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use alloc::collections::{ BinaryHeap, BTreeMap, BTreeSet, LinkedList, VecDeque };
  #[ cfg( feature = "use_alloc" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use hashbrown::{ HashMap, HashSet };
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use std::collections::*;
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use std::vec;
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use std::vec::Vec;

}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "use_alloc" ) ]
  pub use super::alloc_macros::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( all( not( feature = "no_std" ), feature = "collection_constructors" ) ) ]
  pub use ::literally::*;
}

/// Macros used in `use_alloc` context
#[ cfg( all( feature = "enabled", feature = "use_alloc" ) ) ]
pub mod alloc_macros
{
  /// Literally just a BTreeMap literal with keys and values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let m : BTreeMap< String, String > = bmap!{ "key" => "value" };
  /// assert_eq!( m.get( "key" ), Some( &"value".to_string() ) )
  /// ```
  ///
  #[ macro_export( local_inner_macros ) ]
  macro_rules! bmap
  {
    (
      $( $key:expr => $value:expr ),* $( , )?
    )
    =>
    {{
      let mut _map = ::collection_tools::BTreeMap::new();
      $(
        let _ = _map.insert( $key.into(), $value.into() );
      )*
      _map
    }};
  }

  /// Literally just a BTreeSet literal with values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let s : BTreeSet< String > = bset!{ "value" };
  /// assert_eq!( s.get( "value" ), Some( &"value".to_string() ) )
  /// ```
  ///
  #[ macro_export( local_inner_macros ) ]
  macro_rules! bset
  {
    (
      $( $key:expr ),* $( , )?
    )
    =>
    {{
      let mut _set = ::collection_tools::BTreeSet::new();
      $(
        _set.insert( $key.into() );
      )*
      _set
    }};
  }

  /// Literally just a BinaryHeap literal with values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let l : BinaryHeap< String > = heap![ "value" ];
  /// assert_eq!( l.peek(), Some( &"value".to_string() ) )
  /// ```
  ///
  #[ macro_export( local_inner_macros ) ]
  macro_rules! heap
  {
    (
      $( $key:expr ),* $( , )?
    )
    =>
    {{
      let mut _heap = ::collection_tools::BinaryHeap::new();
      $(
        _heap.push( $key.into() );
      )*
      _heap
    }};
  }

  /// Literally just a HashMap literal with keys and values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let m : HashMap< String, String > = hmap!{ "key" => "value" };
  /// assert_eq!( m.get( "key" ), Some( &"value".to_string() ) )
  /// ```
  ///
  #[macro_export(local_inner_macros)]
  macro_rules! hmap
  {
    ( @single $( $x:tt )* ) => ( () );

    (
      @count $( $rest:expr ),*
    )
    =>
    (
      < [ () ] >::len( &[ $( hmap!( @single $rest ) ),* ] )
    );

    (
      $( $key:expr => $value:expr ),* $( , )?
    )
    =>
    {{
      let _cap = hmap!( @count $( $key ),* );
      let mut _map = ::collection_tools::HashMap::with_capacity( _cap );
      $(
        let _ = _map.insert( $key.into(), $value.into() );
      )*
      _map
    }};
  }

  /// Literally just a HashSet literal with values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let s : HashSet< String > = hset!{ "value" };
  /// assert_eq!( s.get( "value" ), Some( &"value".to_string() ) )
  /// ```
  ///
  #[ macro_export( local_inner_macros ) ]
  macro_rules! hset
  {
    ( @single $( $x:tt )* ) => ( () );
    (
      @count $( $rest:expr ),*
    )
    =>
    (
      < [ () ] >::len( &[ $( hset!( @single $rest ) ),* ] )
    );

    (
      $( $key:expr, )+
    )
    =>
    {
      hset!( $( $key ),+ )
    };

    (
      $( $key:expr ),*
    )
    =>
    {{
      let _cap = hset!( @count $( $key ),* );
      let mut _set = ::collection_tools::HashSet::with_capacity( _cap );
      $(
        let _ = _set.insert( $key.into() );
      )*
      _set
    }};
  }

  /// Literally just a LinkedList literal with values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let l : LinkedList< String > = list![ "value" ];
  /// assert_eq!( l.front(), Some( &"value".to_string() ) )
  /// ```
  ///
  #[ macro_export( local_inner_macros ) ]
  macro_rules! list
  {
    (
      $( $key:expr ),* $( , )?
    )
    =>
    {{
      let mut _lst = ::collection_tools::LinkedList::new();
      $(
        _lst.push_back( $key.into() );
      )*
      _lst
    }};
  }

  /// Literally just a VecDeque literal with values into'd.
  /// ```rust
  /// # use collection_tools::*;
  /// let s : VecDeque< String > = vecd![ "value" ];
  /// assert_eq!( s.get( 0 ), Some( &"value".to_string() ) )
  /// ```
  ///
  #[ macro_export( local_inner_macros ) ]
  macro_rules! vecd
  {
      (
        $( $key:expr ),* $( , )?
      )
      =>
      {
        ::collection_tools::VecDeque::from
        (
          ::collection_tools::vec![ $( $key.into() ),* ]
        )
      }
  }
}
