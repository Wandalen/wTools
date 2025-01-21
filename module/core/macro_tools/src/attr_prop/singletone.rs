//! A generic `bool` attribute property which consists of only keyword.
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

use core::marker::PhantomData;
#[ allow( clippy::wildcard_imports ) ]
use crate::*;
// use former_types::Assign;

/// Default marker for `AttributePropertySingletone`.
/// Used if no marker is defined as parameter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertySingletoneMarker;

/// A generic boolean attribute property which consists of only keyword.
/// This property can have two states: `true`, or `false`.
/// Defaults to `false`.
///
/// Unlike other properties, it does not implement parse, because it consists only of keyword which should be parsed outside of the property.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertySingletone< Marker = AttributePropertySingletoneMarker >
(
  bool,
  ::core::marker::PhantomData< Marker >,
);

impl< Marker > AttributePropertySingletone< Marker >
{

  /// Unwraps and returns the internal optional boolean value.
  #[ must_use ]
  #[ inline( always ) ]
  pub fn internal( self ) -> bool
  {
    self.0
  }

  /// Returns a reference to the internal optional boolean value.
  #[ must_use ]
  #[ inline( always ) ]
  pub fn ref_internal( &self ) -> &bool
  {
    &self.0
  }

}

impl< Marker, IntoT > Assign< AttributePropertySingletone< Marker >, IntoT >
for AttributePropertySingletone< Marker >
where
  IntoT : Into< AttributePropertySingletone< Marker > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    *self = component.into();
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
  #[ allow( clippy::default_constructed_unit_structs ) ]
  fn from( src : bool ) -> Self
  {
    Self( src, PhantomData::default() )
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
