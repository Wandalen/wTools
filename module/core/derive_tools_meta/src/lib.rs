// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn_meta/latest/clone_dyn_meta/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]
#![ warn( clippy::undocumented_unsafe_blocks ) ]
#![ allow( non_snake_case ) ]
#![ allow( non_upper_case_globals ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Derive to clone dyn structures.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// #[ cfg( feature = "enabled" ) ]
// use macro_tools::prelude::*;

#[ cfg
(
	any
	(
		feature = "derive_as_mut",
		feature = "derive_as_ref",
		feature = "derive_deref",
		feature = "derive_deref_mut",
		feature = "derive_from",
		feature = "derive_inner_from",
		feature = "derive_variadic_from",
	)
)]
#[ cfg( feature = "enabled" ) ]
mod implementation;
#[ cfg
(
	any
	(
		feature = "derive_as_mut",
		feature = "derive_as_ref",
		feature = "derive_deref",
		feature = "derive_deref_mut",
		feature = "derive_from",
		feature = "derive_inner_from",
		feature = "derive_variadic_from",
	)
)]
#[ cfg( feature = "enabled" ) ]
use implementation::*;

///
/// Derive macro to implement From converting inner type into outer when-ever it's possible to do automatically.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_from" ) ]
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
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_from" ) ]
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
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_inner_from" ) ]
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
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_deref" ) ]
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
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_deref_mut" ) ]
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
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_as_ref" ) ]
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
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
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

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_as_mut" ) ]
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

///
/// Derive macro to implement default constructors `From_0`, `From_1`, `From_2`, `From_3`.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust, ignore, no_run
/// # use derive_tools::*;
/// #[ derive( Make ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust, ignore, no_run
/// pub struct IsTransparent( bool );
/// impl From_0 for IsTransparent
/// {
///   fn make0() -> Self
///   {
///     Self::default();
///   }
/// }
/// impl From_1 for IsTransparent
/// {
///   fn make1( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
///
/// ```

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_variadic_from" ) ]
#[ proc_macro_derive( VariadicFrom ) ]
pub fn derive_variadic_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = variadic_from::variadic_from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}
