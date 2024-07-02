//!
//! A generic boolean attribute property.
//! Defaults to `false`.
//!

use crate::*;
// use former_types::Assign;

/// Default marker for `AttributePropertyBoolean`.
/// Used if no marker is defined as parameter.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyBooleanMarker;

/// A generic boolean attribute property.
/// Defaults to `false`.
///
/// # Example
///
/// ```rust
/// use macro_tools::AttributePropertyBoolean;
///
/// #[ derive( Debug, Default, Clone, Copy ) ]
/// pub struct DebugMarker;
///
/// #[ derive( Debug, Default, Clone, Copy ) ]
/// pub struct EnabledMarker;
///
/// pub trait AttributePropertyComponent
/// {
///   const KEYWORD : &'static str;
/// }
///
/// impl AttributePropertyComponent for DebugMarker
/// {
///   const KEYWORD : &'static str = "debug";
/// }
///
/// impl AttributePropertyComponent for EnabledMarker
/// {
///   const KEYWORD : &'static str = "enabled";
/// }
///
/// #[ derive( Debug, Default ) ]
/// struct MyAttributes
/// {
///   pub debug : AttributePropertyBoolean< DebugMarker >,
///   pub enabled : AttributePropertyBoolean< EnabledMarker >,
/// }
///
/// impl syn::parse::Parse for MyAttributes
/// {
///   fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
///   {
///     let mut debug = AttributePropertyBoolean::< DebugMarker >::default();
///     let mut enabled = AttributePropertyBoolean::< EnabledMarker >::default();
///
///     while !input.is_empty()
///     {
///       let lookahead = input.lookahead1();
///       if lookahead.peek( syn::Ident )
///       {
///         let ident : syn::Ident = input.parse()?;
///         match ident.to_string().as_str()
///         {
///           DebugMarker::KEYWORD => debug = input.parse()?,
///           EnabledMarker::KEYWORD => enabled = input.parse()?,
///           _ => return Err( lookahead.error() ),
///         }
///       }
///       else
///       {
///         return Err( lookahead.error() );
///       }
///
///       // Optional comma handling
///       if input.peek( syn::Token![,] )
///       {
///         input.parse::< syn::Token![,] >()?;
///       }
///     }
///
///     Ok( MyAttributes { debug, enabled } )
///   }
/// }
///
/// let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true ) ] );
/// let meta = match input.meta
/// {
///   syn::Meta::List( meta_list ) => meta_list,
///   _ => panic!( "Expected a Meta::List" ),
/// };
///
/// let nested_meta_stream : proc_macro2::TokenStream = meta.tokens;
/// let attrs : MyAttributes = syn::parse2( nested_meta_stream ).unwrap();
/// println!( "{:?}", attrs );
/// ```
///
/// In this example, the `AttributePropertyBoolean` struct is used to define attributes with boolean properties.
/// The `DebugMarker` and `EnabledMarker` structs act as markers to distinguish between different boolean attributes.
/// The `MyAttributes` struct aggregates these boolean attributes.
///
/// The `Parse` implementation for `MyAttributes` iterates through the attribute's key-value pairs,
/// identifying each by its marker's keyword and parsing the boolean value.
/// It uses the `ParseStream` to parse identifiers and their associated values,
/// matching them to the appropriate marker's keyword.
/// If an unrecognized identifier is encountered, it returns an error.
///
/// The `parse_quote!` macro is used to create a `syn::Attribute` instance with the attribute syntax,
/// which is then parsed into the `MyAttributes` struct. The resulting `MyAttributes` instance is printed to the console.

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyBoolean< Marker = AttributePropertyBooleanMarker >( bool, ::core::marker::PhantomData< Marker > );

impl< Marker > AttributePropertyBoolean< Marker >
{
  /// Just unwraps and returns the internal data.
  #[ inline( always ) ]
  pub fn internal( self ) -> bool
  {
    self.0
  }

  /// Returns a reference to the internal boolean value.
  #[ inline( always ) ]
  pub fn ref_internal( &self ) -> &bool
  {
    &self.0
  }
}

impl< Marker, IntoT > Assign< AttributePropertyBoolean< Marker >, IntoT >
for AttributePropertyBoolean< Marker >
where
  IntoT : Into< AttributePropertyBoolean< Marker > >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    *self = component.into();
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
    input.parse::< syn::Token![ = ] >()?;
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
