//! A generic boolean attribute property.
//! Defaults to `None`.
//!
//! This property can have two states: `true`, or `false`.
//!
//! # Example
//!
//! ```ignore
//! #[ attribute( some ) ]
//! ```
//!
//! This is useful for attributes that need to enable or disable features or flags.

use crate::*;

/// Default marker for `AttributePropertySingletone`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertySingletoneMarker;

/// A generic attribute property for switching on/off.
/// This property can have two states: `true`, or `false`.
/// Defaults to `false`.
///
/// Unlike other properties, it does not implement parse, because it consists only of keyword which should be parsed outside of the property.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertySingletone< Marker = AttributePropertySingletoneMarker >
(
  bool,
  ::core::marker::PhantomData< Marker >
);

impl< Marker > AttributePropertySingletone< Marker >
{

  /// Unwraps and returns the internal optional boolean value.
  #[ inline( always ) ]
  pub fn internal( self ) -> bool
  {
    self.0
  }

  /// Returns a reference to the internal optional boolean value.
  #[ inline( always ) ]
  pub fn ref_internal( &self ) -> &bool
  {
    &self.0
  }

}

impl< Marker > AttributePropertyComponent for AttributePropertySingletone< Marker >
where
  Marker : AttributePropertyComponent,
{
  const KEYWORD : &'static str = Marker::KEYWORD;
}

impl< Marker > From< bool > for AttributePropertySingletone< Marker >
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< Marker > From< AttributePropertySingletone< Marker > > for bool
{
  #[ inline( always ) ]
  fn from( src : AttributePropertySingletone< Marker > ) -> Self
  {
    src.0
  }
}

impl< Marker > core::ops::Deref for AttributePropertySingletone< Marker >
{
  type Target = bool;

  #[ inline( always ) ]
  fn deref( &self ) -> &bool
  {
    &self.0
  }
}

impl< Marker > AsRef< bool > for AttributePropertySingletone< Marker >
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &bool
  {
    &self.0
  }
}
