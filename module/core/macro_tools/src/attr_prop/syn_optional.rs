//!
//! Property of an attribute which simply wraps one of the standard `syn` types and keeps it optional.
//!

use crate::*;
use former_types::Assign;

/// Default marker for `AttributePropertyOptionalSyn`.
/// Used if no marker is defined as parameter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyOptionalSynMarker;

///
/// Property of an attribute which simply wraps one of the standard `syn` types and keeps it optional.
///

#[ derive( Debug, Clone ) ]
pub struct AttributePropertyOptionalSyn< T, Marker = AttributePropertyOptionalSynMarker >( Option< T >, ::core::marker::PhantomData< Marker > )
where
  T : syn::parse::Parse + quote::ToTokens;

impl< T, Marker > AttributePropertyOptionalSyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  /// Just unwraps and returns the internal data.
  #[ inline( always ) ]
  pub fn internal( self ) -> Option< T >
  {
    self.0
  }

  /// Returns an Option reference to the internal data.
  #[ inline( always ) ]
  pub fn ref_internal( &self ) -> Option< &T >
  {
    self.0.as_ref()
  }
}

impl< T, Marker, IntoT > Assign< AttributePropertyOptionalSyn< T, Marker >, IntoT >
for AttributePropertyOptionalSyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
  IntoT : Into< AttributePropertyOptionalSyn< T, Marker > >,
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

impl< T, Marker > AttributePropertyComponent for AttributePropertyOptionalSyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
  Marker : AttributePropertyComponent,
{
  const KEYWORD : &'static str = Marker::KEYWORD;
}

impl< T, Marker > Default for AttributePropertyOptionalSyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  fn default() -> Self
  {
    Self( None, Default::default() )
  }
}

impl< T, Marker > syn::parse::Parse for AttributePropertyOptionalSyn< T, Marker >
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

impl< T, Marker > quote::ToTokens for AttributePropertyOptionalSyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.0.to_tokens( tokens );
  }
}

impl< T, Marker > core::ops::Deref for AttributePropertyOptionalSyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  type Target = Option< T >;
  #[ inline( always ) ]
  fn deref( &self ) -> &Option< T >
  {
    &self.0
  }
}

impl< T, Marker > AsRef< Option< T > > for AttributePropertyOptionalSyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &Option< T >
  {
    &self.0
  }
}

impl< T, Marker > From< T > for AttributePropertyOptionalSyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    Self( Some( src ), Default::default() )
  }
}

impl< T, Marker > From< Option< T > > for AttributePropertyOptionalSyn< T, Marker >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : Option< T > ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< T, Marker > From< AttributePropertyOptionalSyn< T, Marker > > for Option< T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyOptionalSyn< T, Marker > ) -> Self
  {
    src.0
  }
}

impl< 'a, T, Marker > From< &'a AttributePropertyOptionalSyn< T, Marker > > for Option< &'a T >
where T : syn::parse::Parse + quote::ToTokens
{
  #[ inline( always ) ]
  fn from( src : &'a AttributePropertyOptionalSyn< T, Marker > ) -> Self
  {
    src.0.as_ref()
  }
}
