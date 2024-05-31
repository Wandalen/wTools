//!
//! Property of an attribute which simply wraps one of the standard `syn` types.
//!

use crate::*;
use former_types::Assign;

/// Default marker for `AttributePropertySyn`.
/// Used if no marker is defined as parameter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertySynMarker;

///
/// Property of an attribute which simply wraps one of the standard `syn` types.
///

#[ derive( Debug, Clone ) ]
pub struct AttributePropertySyn< T, Marker = AttributePropertySynMarker >( T, ::core::marker::PhantomData< Marker > )
where
  T : syn::parse::Parse + quote::ToTokens;

impl< T, Marker > AttributePropertySyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  /// Just unwraps and returns the internal data.
  // #[ allow( dead_code ) ]
  #[ inline( always ) ]
  pub fn internal( self ) -> T
  {
    self.0
  }

  /// Returns a reference to the internal data.
  // #[ allow( dead_code ) ]
  #[ inline( always ) ]
  pub fn ref_internal( &self ) -> &T
  {
    &self.0
  }
}

impl< T, Marker, IntoT > Assign< AttributePropertySyn< T, Marker >, IntoT >
for AttributePropertySyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
  IntoT : Into< AttributePropertySyn< T, Marker > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    *self = component.into();
  }
}

impl< T, Marker > AttributePropertyComponent for AttributePropertySyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
  Marker : AttributePropertyComponent,
{
  const KEYWORD : &'static str = Marker::KEYWORD;
}

impl< T, Marker > syn::parse::Parse for AttributePropertySyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    input.parse::< syn::Token![ = ] >()?;
    let value : T = input.parse()?;
    Ok( value.into() )
  }
}

impl< T, Marker > quote::ToTokens for AttributePropertySyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.0.to_tokens( tokens );
  }
}

impl< T, Marker > core::ops::Deref for AttributePropertySyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  type Target = T;
  #[ inline( always ) ]
  fn deref( &self ) -> &T
  {
    &self.0
  }
}

impl< T, Marker > AsRef< T > for AttributePropertySyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< T, Marker > From< T > for AttributePropertySyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    Self( src, Default::default() )
  }
}
