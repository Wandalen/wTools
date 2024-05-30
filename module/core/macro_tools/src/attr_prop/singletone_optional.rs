//! A generic `Option< bool >` attribute property which consists of only keyword.
//! Defaults to `None`.
//!
//! This property can have three states: `None`, `Some( true )`, or `Some( false )`.
//! It parses `on` and `off` keywords to represent `Some( true )` and `Some( false )` respectively.
//!
//! # Example
//!
//! ```ignore
//! #[ attribute( on) ]
//! #[ attribute( off ) ]
//! ```
//!
//! This is useful for attributes that need to enable or disable features or flags.

use crate::*;
use former_types::Assign;

/// Default marker for `AttributePropertyOptionalSingletone`.
/// Used if no marker is defined as parameter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyOptionalSingletoneMarker;

/// A generic attribute property for switching on/off.
/// Has 3 states: `None`, `Some( true )`, `Some( false )`.
/// Defaults to `None`.
///
/// Unlike [`AttributePropertyOptionalBoolean`], it "understands" `on`, `off` keywords during parsing.
/// For example: `#[ attribute( on ) ]` and `#[ attribute( off )]`.
/// As a consequence, the property has two keywords.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyOptionalSingletone< Marker = AttributePropertyOptionalSingletoneMarker >
(
  Option< bool >,
  ::core::marker::PhantomData< Marker >,
);

impl< Marker > AttributePropertyOptionalSingletone< Marker >
{

  /// Return bool value: on/off, use argument as default if it's `None`.
  #[ inline ]
  pub fn value( self, default : bool ) -> bool
  {
    if self.0.is_none()
    {
      return default;
    }
    self.0.unwrap()
  }

  /// Unwraps and returns the internal optional boolean value.
  #[ inline( always ) ]
  pub fn internal( self ) -> Option< bool >
  {
    self.0
  }

  /// Returns a reference to the internal optional boolean value.
  #[ inline( always ) ]
  pub fn ref_internal( &self ) -> Option< &bool >
  {
    self.0.as_ref()
  }

}

impl< Marker, IntoT > Assign< AttributePropertyOptionalSingletone< Marker >, IntoT >
for AttributePropertyOptionalSingletone< Marker >
where
  IntoT : Into< AttributePropertyOptionalSingletone< Marker > >,
{
  /// Inserts value of another instance into the option if it is None, then returns a mutable reference to the contained value.
  /// If another instance does is None then do nothing.
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    match component.0
    {
      Some( val ) => { self.0 = Some( val ); },
      None => {},
    }
  }
}

impl< Marker > AttributePropertyComponent for AttributePropertyOptionalSingletone< Marker >
where
  Marker : AttributePropertyComponent,
{
  const KEYWORD : &'static str = Marker::KEYWORD;
}

impl< Marker > From< bool > for AttributePropertyOptionalSingletone< Marker >
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( Some( src ), Default::default() )
  }
}

impl< Marker > From< Option< bool > > for AttributePropertyOptionalSingletone< Marker >
{
  #[ inline( always ) ]
  fn from( src : Option< bool > ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< Marker > From< AttributePropertyOptionalSingletone< Marker > > for Option< bool >
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyOptionalSingletone< Marker > ) -> Self
  {
    src.0
  }
}

impl< Marker > core::ops::Deref for AttributePropertyOptionalSingletone< Marker >
{
  type Target = Option< bool >;

  #[ inline( always ) ]
  fn deref( &self ) -> &Option< bool >
  {
    &self.0
  }
}

impl< Marker > AsRef< Option< bool > > for AttributePropertyOptionalSingletone< Marker >
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &Option< bool >
  {
    &self.0
  }
}
