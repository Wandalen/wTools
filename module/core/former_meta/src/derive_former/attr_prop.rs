
use super::*;
use macro_tools::{ AttributePropertyComponent };

/// Generics bolean attirbute property.
/// Defaults to `false`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyBoolean< T >( bool, core::marker::PhantomData< T > );

impl< T > AttributePropertyComponent for AttributePropertyBoolean< T >
{
  const KEYWORD : &'static str = "custom";
}

impl< T > syn::parse::Parse for AttributePropertyBoolean< T >
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let value : syn::LitBool = input.parse()?;
    Ok( value.value.into() )
  }
}

impl< T > From< bool > for AttributePropertyBoolean< T >
{
  #[ inline( always ) ]
  fn from( src : bool ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< T > From< AttributePropertyBoolean< T > > for bool
{
  #[ inline( always ) ]
  fn from( src : AttributePropertyBoolean< T > ) -> Self
  {
    src.0
  }
}

impl< T > core::ops::Deref for AttributePropertyBoolean< T >
{
  type Target = bool;
  #[ inline( always ) ]
  fn deref( &self ) -> &bool
  {
    &self.0
  }
}

impl< T > AsRef< bool > for AttributePropertyBoolean< T >
{
  #[ inline( always ) ]
  fn as_ref( &self ) -> &bool
  {
    &self.0
  }
}
