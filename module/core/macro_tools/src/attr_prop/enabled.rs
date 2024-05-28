//! A generic boolean attribute property.
//! Defaults to `None`.
//!
//! This property can have three states: `None`, `Some(true)`, or `Some(false)`.
//! It parses `on` and `off` keywords to represent `Some(true)` and `Some(false)` respectively.
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

/// Default marker for `AttributePropertyEnabled`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyEnabledMarker;

/// A generic attribute property for switching on/off.
/// Has 3 states: `None`, `Some( true )`, `Some( false )`.
/// Defaults to `None`.
///
/// Unlike [`AttributePropertyOptionalBoolean`], it "understands" `on`, `off` keywords during parsing.
/// For example: `#[ attribute( on ) ]` and `#[ attribute( off )]`.
/// As a consequence, the property has two keywords.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyEnabled< Marker = AttributePropertyEnabledMarker >
(
  Option< bool >,
  ::core::marker::PhantomData< Marker >
);

impl< Marker > AttributePropertyEnabled< Marker >
{
  /// Keywords for parsing this attribute property.
  pub const KEYWORDS : [& 'static str ; 2] = [ "on", "off" ];
  /// Keywords for parsing this attribute property.
  pub const KEYWORD_OFF : & 'static str = "off";
  /// Keywords for parsing this attribute property.
  pub const KEYWORD_ON : & 'static str = "on";

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

}

impl< Marker > AttributePropertyEnabled< Marker >
{
  /// Unwraps and returns the internal optional boolean value.
  pub fn internal( self ) -> Option< bool >
  {
    self.0
  }

  /// Returns a reference to the internal optional boolean value.
  pub fn ref_internal( & self ) -> Option< & bool >
  {
    self.0.as_ref()
  }
}

impl< Marker > AttributePropertyComponent for AttributePropertyEnabled< Marker >
where
  Marker : AttributePropertyComponent,
{
  const KEYWORD : & 'static str = Marker::KEYWORD;
}

impl< Marker > From< bool > for AttributePropertyEnabled< Marker >
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( Some( src ), Default::default() )
  }
}

impl< Marker > From< Option< bool > > for AttributePropertyEnabled< Marker >
{
  #[ inline( always ) ]
  fn from( src : Option< bool > ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< Marker > From< AttributePropertyEnabled< Marker > > for Option< bool >
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyEnabled< Marker > ) -> Self
  {
    src.0
  }
}

impl< Marker > core::ops::Deref for AttributePropertyEnabled< Marker >
{
  type Target = Option< bool >;

  #[ inline( always ) ]
  fn deref( & self ) -> & Option< bool >
  {
    & self.0
  }
}

impl< Marker > AsRef< Option< bool > > for AttributePropertyEnabled< Marker >
{
  #[ inline( always ) ]
  fn as_ref( & self ) -> & Option< bool >
  {
    & self.0
  }
}
