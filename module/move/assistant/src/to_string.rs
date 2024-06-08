#[ allow( unused_imports ) ]
use super::*;

use std::
{
  fmt,
  // collections::HashMap,
  // borrow::Cow,
};

// ==

/// Marker type for using Debug formatting.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDebug;

/// Marker type for using Display formatting.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDisplay;

/// Marker type for usign Well formatting.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithWell;

// ==

/// Marker type for ToStringWithFallback.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct ToStringWithFallbackMarker;

/// Trait to convert a type to a string with a fallback formatting.
pub trait ToStringWithFallback< How, Fallback >
{
  /// Converts the type to a string using the specified formatting or a fallback.
  fn to_string_with_fallback( self ) -> String;
}

impl< T, How, Fallback > ToStringWithFallback< How, Fallback > for Ref< '_, T, ToStringWithFallbackMarker >
where
  T : ToStringWith< Fallback >,
{
  /// Converts the type to a string using the specified formatting.
  fn to_string_with_fallback( self ) -> String
  {
    < T as ToStringWith< Fallback > >::to_string_with( self.0 )
  }
}

impl< T, How, Fallback > ToStringWithFallback< How, Fallback > for &Ref< '_, T, ToStringWithFallbackMarker >
where
  T : ToStringWith< How >,
{
  /// Converts the type to a string using the fallback formatting.
  fn to_string_with_fallback( self ) -> String
  {
    < T as ToStringWith< How > >::to_string_with( self.0 )
  }
}

// pub fn to_string_with_fallback< T, How, Fallback >( src : &T ) -> String
// where
//   T : ToStringWithFallback< How, Fallback >,
// {
//   ToStringWithFallback::< How, Fallback >::to_string_with_fallback( &( src, ) )
// }

// ==

/// Trait to convert a type to a string using a specified formatting method.
pub trait ToStringWith< How >
{
  /// Converts the type to a string using the specified formatting method.
  fn to_string_with( &self ) -> String;
}

impl< T > ToStringWith< WithDebug > for T
where
  T : fmt::Debug,
{
  /// Converts the type to a string using Debug formatting.
  fn to_string_with( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl< T > ToStringWith< WithDisplay > for T
where
  T : fmt::Display,
{
  /// Converts the type to a string using Display formatting.
  fn to_string_with( &self ) -> String
  {
    format!( "{}", self )
  }
}