
use super::*;
use macro_tools::{ AttributePropertyComponent };

/*

AttributePropertyBoolean
AttributePropertyOptionalBoolean
AttributePropertyOptionalSyn

*/

// = AttributePropertyBoolean

/// Generics bolean attirbute property.
/// Defaults to `false`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyBoolean< Marker >( bool, ::core::marker::PhantomData< Marker > );

impl< Marker > AttributePropertyBoolean< Marker >
{
  /// Just unwrap, returning internal data.
  pub fn internal( self ) -> bool
  {
    self.0
  }
  /// Returns Option< &T > instead of &Option< T >
  pub fn ref_internal( &self ) -> &bool
  {
    &self.0
  }
}

impl< Marker > AttributePropertyComponent for AttributePropertyBoolean< Marker >
where
  Marker : AttributePropertyComponent,
{
  const KEYWORD : &'static str = Marker::KEYWORD;
}

impl< Marker > syn::parse::Parse for AttributePropertyBoolean< Marker >
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let value : syn::LitBool = input.parse()?;
    Ok( value.value.into() )
  }
}

impl< Marker > From< bool > for AttributePropertyBoolean< Marker >
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< Marker > From< AttributePropertyBoolean< Marker > > for bool
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyBoolean< Marker > ) -> Self
  {
    src.0
  }
}

impl< Marker > core::ops::Deref for AttributePropertyBoolean< Marker >
{
  type Target = bool;
  #[ inline( always ) ]
  fn deref( &self ) -> &bool
  {
    &self.0
  }
}

impl< Marker > AsRef< bool > for AttributePropertyBoolean< Marker >
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &bool
  {
    &self.0
  }
}

// = AttributePropertyOptionalBoolean

/// Generics optional bolean attirbute property: `Option< bool >`.
/// Defaults to `false`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyOptionalBoolean< Marker >( Option< bool >, ::core::marker::PhantomData< Marker > );

impl< Marker > AttributePropertyOptionalBoolean< Marker >
{
  /// Just unwrap, returning internal data.
  pub fn internal( self ) -> Option< bool >
  {
    self.0
  }
  /// Returns Option< &T > instead of &Option< T >
  pub fn ref_internal( &self ) -> Option< &bool >
  {
    self.0.as_ref()
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

// = AttributePropertyOptionalSyn

/// Property of an attribute which simply wrap one of standard of `syn` type and keep it optional.
#[ derive( Debug, Clone ) ]
pub struct AttributePropertyOptionalSyn< T, Marker >( Option< T >, ::core::marker::PhantomData< Marker > )
where
  T : syn::parse::Parse + quote::ToTokens,
;

impl< T, Marker > AttributePropertyOptionalSyn< T, Marker >
where
  T : syn::parse::Parse + quote::ToTokens,
{
  /// Just unwrap, returning internal data.
  pub fn internal( self ) -> Option< T >
  {
    self.0
  }
  /// Returns Option< &T > instead of &Option< T >
  pub fn ref_internal( &self ) -> Option< &T >
  {
    self.0.as_ref()
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
    let value : T = input.parse()?;
    Ok( value.into() )
  }
}

// xxx
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
