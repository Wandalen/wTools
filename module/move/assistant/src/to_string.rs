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

pub trait ToStringWithFallback< 'a, How, Fallback >
{
  fn to_string_with_fallback( &'a self ) -> String;
}

impl< 'a, T, How, Fallback > ToStringWithFallback< 'a, How, Fallback > for ( &T, )
where
  T : ToStringWith< 'a, How >,
{
  fn to_string_with_fallback( &'a self ) -> String
  {
    < T as ToStringWith< 'a, How > >::to_string_with( self.0 )
  }
}

impl< 'a, T, How, Fallback > ToStringWithFallback< 'a, How, Fallback > for &( &T, )
where
  T : ToStringWith< 'a, Fallback >,
{
  fn to_string_with_fallback( &'a self ) -> String
  {
    < T as ToStringWith< 'a, Fallback > >::to_string_with( self.0 )
  }
}

pub trait ToStringWith< 'a, How >
{
  fn to_string_with( &'a self ) -> String;
}

impl< 'a, T > ToStringWith< 'a, WithDebug > for T
where
  T : fmt::Debug,
{
  fn to_string_with( &'a self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl< 'a, T > ToStringWith< 'a, WithDisplay > for T
where
  T : fmt::Display,
{
  fn to_string_with( &'a self ) -> String
  {
    format!( "{}", self )
  }
}
