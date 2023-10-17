#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]
#![ forbid( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]

//!
//! Implement couple of derives of general-purpose.
//!

use proc_macro_tools::prelude::*;
pub use proc_macro_tools::Result;
// use proc_macro_tools::syn::spanned::Spanned;
// pub type Result< T > = std::result::Result< T, syn::Error >;

mod input;
use input::*;
mod as_mut;
mod as_ref;
mod deref;
mod deref_mut;
mod from_inner;
mod inner_from;

///
/// Derive macro to implement From converting inner type into outer when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( From ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl From< bool > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
/// ```

#[ proc_macro_derive( From ) ]
pub fn from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = from_inner::from_inner( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement From converting inner type into outer when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( FromInner ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl From< bool > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
/// ```

#[ proc_macro_derive( FromInner ) ]
pub fn from_inner( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = from_inner::from_inner( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement From converting outer type into inner when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( InnerFrom ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl From< IsTransparent > for bool
/// {
///   #[ inline( always ) ]
///   fn from( src : IsTransparent ) -> Self
///   {
///     src.0
///   }
/// }
/// ```

#[ proc_macro_derive( InnerFrom ) ]
pub fn inner_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = inner_from::inner_from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement Deref when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( Deref ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl core::ops::Deref for IsTransparent
/// {
///   type Target = bool;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.0
///   }
/// }
/// ```

#[ proc_macro_derive( Deref ) ]
pub fn deref( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = deref::deref( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement Deref when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( Deref, DerefMut ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl core::ops::Deref for IsTransparent
/// {
///   type Target = bool;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.0
///   }
/// }
/// impl core::ops::DerefMut for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     &mut self.0
///   }
/// }
///
/// ```

#[ proc_macro_derive( DerefMut ) ]
pub fn deref_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = deref_mut::deref_mut( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement AsRef when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( AsRef ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl AsRef< bool > for IsTransparent
/// {
///   fn as_ref( &self ) -> &bool
///   {
///     &self.0
///   }
/// }
/// ```

#[ proc_macro_derive( AsRef ) ]
pub fn as_ref( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = as_ref::as_ref( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement AsMut when-ever it's possible to do automatically.
///
/// ### Basic use-case. :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// use derives::*;
/// #[ derive( AsMut ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl AsMut< bool > for IsTransparent
/// {
///   fn as_mut( &mut self ) -> &mut bool
///   {
///     &mut self.0
///   }
/// }
///
/// ```

#[ proc_macro_derive( AsMut ) ]
pub fn as_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = as_mut::as_mut( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
