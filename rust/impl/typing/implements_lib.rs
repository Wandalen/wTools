#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Macro to answer the question: does it implement a trait?
//!
//! This solution has a limitation:
//! - In case enity is a function and trat is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// #[ macro_use ]
mod implements_impl;

///
/// Macro `implements` to answer the question: does it implement a trait?
///
/// ### Sample
/// ```
/// use implements::*;
///
/// dbg!( implements!( 13_i32 => Copy ) );
/// // < implements!( 13_i32 => Copy ) : true
/// dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
/// // < implements!( 13_i32 => Copy ) : false
/// ```

#[ macro_export ]
macro_rules! implements
{
  ( $( $arg : tt )+ ) =>
  {
    $crate::_implements!( $( $arg )+ );
  }
}

///
/// Macro `instance_of` to answer the question: does it implement a trait? Alias of the macro `implements`.
///
/// ### Sample
/// ```
/// use implements::instance_of;
///
/// dbg!( instance_of!( 13_i32 => Copy ) );
/// // < instance_of!( 13_i32 => Copy ) : true
/// dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
/// // < instance_of!( 13_i32 => Copy ) : false
/// ```

#[ macro_export ]
macro_rules! instance_of
{
  ( $( $arg : tt )+ ) =>
  {
    $crate::_implements!( $( $arg )+ );
  }
}
