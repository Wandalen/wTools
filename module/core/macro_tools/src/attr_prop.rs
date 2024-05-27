//!
//! Attribute's properties. Reuse them to define how to parse properties of an attribute.
//!
//! # Example
//!
//! ```rust
//! use macro_tools::AttributePropertyBoolean;
//!
//! #[ derive( Debug, Default, Clone, Copy ) ]
//! pub struct DebugMarker;
//!
//! #[ derive( Debug, Default, Clone, Copy ) ]
//! pub struct EnabledMarker;
//!
//! pub trait AttributePropertyComponent
//! {
//!   const KEYWORD : &'static str;
//! }
//!
//! impl AttributePropertyComponent for DebugMarker
//! {
//!   const KEYWORD : &'static str = "debug";
//! }
//!
//! impl AttributePropertyComponent for EnabledMarker
//! {
//!   const KEYWORD : &'static str = "enabled";
//! }
//!
//! #[ derive( Debug, Default ) ]
//! struct MyAttributes
//! {
//!   pub debug : AttributePropertyBoolean< DebugMarker >,
//!   pub enabled : AttributePropertyBoolean< EnabledMarker >,
//! }
//!
//! impl syn::parse::Parse for MyAttributes
//! {
//!   fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
//!   {
//!     let mut debug = AttributePropertyBoolean::< DebugMarker >::default();
//!     let mut enabled = AttributePropertyBoolean::< EnabledMarker >::default();
//!
//!     while !input.is_empty()
//!     {
//!       let lookahead = input.lookahead1();
//!       if lookahead.peek( syn::Ident )
//!       {
//!         let ident : syn::Ident = input.parse()?;
//!         input.parse::< syn::Token![=] >()?;
//!         match ident.to_string().as_str()
//!         {
//!           DebugMarker::KEYWORD => debug = input.parse()?,
//!           EnabledMarker::KEYWORD => enabled = input.parse()?,
//!           _ => return Err( lookahead.error() ),
//!         }
//!       }
//!       else
//!       {
//!         return Err( lookahead.error() );
//!       }
//!
//!       // Optional comma handling
//!       if input.peek( syn::Token![,] )
//!       {
//!         input.parse::< syn::Token![,] >()?;
//!       }
//!     }
//!
//!     Ok( MyAttributes { debug, enabled } )
//!   }
//! }
//!
//! let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true, debug = false ) ] );
//! let meta = match input.meta
//! {
//!   syn::Meta::List( meta_list ) => meta_list,
//!   _ => panic!( "Expected a Meta::List" ),
//! };
//!
//! let nested_meta_stream : proc_macro2::TokenStream = meta.tokens;
//! let attrs : MyAttributes = syn::parse2( nested_meta_stream ).unwrap();
//! println!( "{:?}", attrs );
//! ```
//!
//! In this example, the `AttributePropertyBoolean` struct is used to define attributes with boolean properties.
//! The `DebugMarker` and `EnabledMarker` structs act as markers to distinguish between different boolean attributes.
//! The `MyAttributes` struct aggregates these boolean attributes.
//!
//! The `Parse` implementation for `MyAttributes` iterates through the attribute's key-value pairs,
//! identifying each by its marker's keyword and parsing the boolean value.
//! It uses the `ParseStream` to parse identifiers and their associated values,
//! matching them to the appropriate marker's keyword.
//! If an unrecognized identifier is encountered, it returns an error.
//!
//! The `parse_quote!` macro is used to create a `syn::Attribute` instance with the attribute syntax,
//! which is then parsed into the `MyAttributes` struct. The resulting `MyAttributes` instance is printed to the console.

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  // = AttributePropertyBoolean

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
  ///         input.parse::< syn::Token![=] >()?;
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
  /// let input : syn::Attribute = syn::parse_quote!( #[ attribute( enabled = true, debug = false ) ] );
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
  pub struct AttributePropertyBoolean< Marker >( bool, ::core::marker::PhantomData< Marker > );

  impl< Marker > AttributePropertyBoolean< Marker >
  {
    /// Just unwraps and returns the internal data.
    pub fn internal( self ) -> bool
    {
      self.0
    }

    /// Returns a reference to the internal boolean value.
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

  /// A generic optional boolean attribute property: `Option< bool >`.
  /// Defaults to `false`.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyOptionalBoolean< Marker >( Option< bool >, ::core::marker::PhantomData< Marker > );

  impl< Marker > AttributePropertyOptionalBoolean< Marker >
  {
    /// Just unwraps and returns the internal data.
    pub fn internal( self ) -> Option< bool >
    {
      self.0
    }

    /// Returns a reference to the internal optional boolean value.
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

  // = AttributePropertySyn

  ///
  /// Property of an attribute which simply wraps one of the standard `syn` types.
  ///
  #[ derive( Debug, Clone ) ]
  pub struct AttributePropertySyn< T, Marker >( T, ::core::marker::PhantomData< Marker > )
  where
    T : syn::parse::Parse + quote::ToTokens;

  impl< T, Marker > AttributePropertySyn< T, Marker >
  where
    T : syn::parse::Parse + quote::ToTokens,
  {
    /// Just unwraps and returns the internal data.
    #[ allow( dead_code ) ]
    pub fn internal( self ) -> T
    {
      self.0
    }

    /// Returns a reference to the internal data.
    #[ allow( dead_code ) ]
    pub fn ref_internal( &self ) -> &T
    {
      &self.0
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

  // = AttributePropertyOptionalSyn

  ///
  /// Property of an attribute which simply wraps one of the standard `syn` types and keeps it optional.
  ///
  #[ derive( Debug, Clone ) ]
  pub struct AttributePropertyOptionalSyn< T, Marker >( Option< T >, ::core::marker::PhantomData< Marker > )
  where
    T : syn::parse::Parse + quote::ToTokens;

  impl< T, Marker > AttributePropertyOptionalSyn< T, Marker >
  where
    T : syn::parse::Parse + quote::ToTokens,
  {
    /// Just unwraps and returns the internal data.
    pub fn internal( self ) -> Option< T >
    {
      self.0
    }

    /// Returns an Option reference to the internal data.
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

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as attr_prop;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    AttributePropertyBoolean,
    AttributePropertyOptionalBoolean,
    AttributePropertySyn,
    AttributePropertyOptionalSyn,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
