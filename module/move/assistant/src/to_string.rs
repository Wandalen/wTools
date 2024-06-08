#[ allow( unused_imports ) ]
use super::*;

use std::
{
  fmt,
  // collections::HashMap,
  // borrow::Cow,
};

// =

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDebug;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDisplay;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithWell;

pub trait ToStringWithFallback< How, Fallback >
{
  fn to_string_with_fallback( &self ) -> String;
}

impl< T, How, Fallback > ToStringWithFallback< How, Fallback > for ( &T, )
where
  T : ToStringWith< How >,
{
  fn to_string_with_fallback( &self ) -> String
  {
    < T as ToStringWith< How > >::to_string_with( self.0 )
  }
}

impl< T, How, Fallback > ToStringWithFallback< How, Fallback > for &( &T, )
where
  T : ToStringWith< Fallback >,
{
  fn to_string_with_fallback( &self ) -> String
  {
    < T as ToStringWith< Fallback > >::to_string_with( self.0 )
  }
}

pub trait ToStringWith< How >
{
  fn to_string_with( &self ) -> String;
}

impl< T > ToStringWith< WithDebug > for T
where
  T : fmt::Debug,
{
  fn to_string_with( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl< T > ToStringWith< WithDisplay > for T
where
  T : fmt::Display,
{
  fn to_string_with( &self ) -> String
  {
    format!( "{}", self )
  }
}
