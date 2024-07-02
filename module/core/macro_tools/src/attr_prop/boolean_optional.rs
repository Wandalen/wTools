//!
//! A generic optional boolean attribute property: `Option< bool >`.
//! Defaults to `false`.
//!

use crate::*;
use components::Assign;

/// Default marker for `AttributePropertyOptionalSingletone`.
/// Used if no marker is defined as parameter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyOptionalBooleanMarker;

/// A generic optional boolean attribute property: `Option< bool >`.
/// Defaults to `false`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyOptionalBoolean< Marker = AttributePropertyOptionalBooleanMarker >( Option< bool >, ::core::marker::PhantomData< Marker > );

impl< Marker > AttributePropertyOptionalBoolean< Marker >
{
  /// Just unwraps and returns the internal data.
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

impl< Marker, IntoT > Assign< AttributePropertyOptionalBoolean< Marker >, IntoT >
for AttributePropertyOptionalBoolean< Marker >
where
  IntoT : Into< AttributePropertyOptionalBoolean< Marker > >,
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

impl< Marker > AttributePropertyComponent for AttributePropertyOptionalBoolean< Marker >
where
  Marker : AttributePropertyComponent,
{
  const KEYWORD : &'static str = Marker::KEYWORD;
}

impl< Marker > syn::parse::Parse for AttributePropertyOptionalBoolean< Marker >
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    input.parse::< syn::Token![ = ] >()?;
    let value : syn::LitBool = input.parse()?;
    Ok( value.value.into() )
  }
}

impl< Marker > From< bool > for AttributePropertyOptionalBoolean< Marker >
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( Some( src ), Default::default() )
  }
}

impl< Marker > From< Option< bool > > for AttributePropertyOptionalBoolean< Marker >
{
  #[ inline( always ) ]
  fn from( src : Option< bool > ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< Marker > From< AttributePropertyOptionalBoolean< Marker > > for Option< bool >
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyOptionalBoolean< Marker > ) -> Self
  {
    src.0
  }
}

impl< Marker > core::ops::Deref for AttributePropertyOptionalBoolean< Marker >
{
  type Target = Option< bool >;
  #[ inline( always ) ]
  fn deref( &self ) -> &Option< bool >
  {
    &self.0
  }
}

impl< Marker > AsRef< Option< bool > > for AttributePropertyOptionalBoolean< Marker >
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &Option< bool >
  {
    &self.0
  }
}
