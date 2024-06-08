#[ allow( unused_imports ) ]
use super::*;

use std::
{
  fmt,
  // collections::HashMap,
  // borrow::Cow,
};

// ==

/// Marker type for using Debug formatting.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDebug;

/// Marker type for using Display formatting.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDisplay;

/// Marker type for usign Well formatting.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithWell;

// ==

/// Marker type for ToStringWithFallback.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct ToStringWithFallbackParams< How, Fallback >( ::core::marker::PhantomData< fn() -> ( How, Fallback ) > );

/// Trait to convert a type to a string with a fallback formatting.
pub trait ToStringWithFallback< How, Fallback >
{
  /// Converts the type to a string using the specified formatting or a fallback.
  fn to_string_with_fallback( self ) -> String
  ;
}

impl< T, How, Fallback > ToStringWithFallback< How, Fallback >
for Ref< '_, T, ToStringWithFallbackParams< How, Fallback > >
where
  T : ToStringWith< Fallback >,
{
  /// Converts the type to a string using the specified formatting.
  fn to_string_with_fallback( self ) -> String
  {
    < T as ToStringWith< Fallback > >::to_string_with( self.0 )
  }
}

impl< T, How, Fallback > ToStringWithFallback< How, Fallback >
for &Ref< '_, T, ToStringWithFallbackParams< How, Fallback > >
where
  T : ToStringWith< How >,
{
  /// Converts the type to a string using the fallback formatting.
  fn to_string_with_fallback( self ) -> String
  {
    < T as ToStringWith< How > >::to_string_with( self.0 )
  }
}

//

/// Macro to convert a value to a string using a specified formatting method with a fallback.
///
/// # Parameters
/// - `$how`: The primary formatting type (e.g., `WithDebug`, `WithDisplay`).
/// - `$fallback`: The fallback formatting type.
/// - `$src`: The source value to format.
///
/// # Example
/// ```rust
/// use core::fmt;
/// use assistant::
/// {
///   WithDebug,
///   WithDisplay,
///   to_string_with_fallback,
/// };
///
/// // Define a struct that implements both Debug and Display traits.
/// struct Both;
///
/// impl fmt::Debug for Both
/// {
///   fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
///   {
///     write!( f, "This is debug" )
///   }
/// }
///
/// impl fmt::Display for Both
/// {
///   fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
///   {
///     write!( f, "This is display" )
///   }
/// }
///
/// // Define a struct that implements only the Debug trait.
/// struct OnlyDebug;
///
/// impl fmt::Debug for OnlyDebug
/// {
///   fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
///   {
///     write!( f, "This is debug" )
///   }
/// }
///
/// // Example usage: Using Both which implements both Debug and Display.
/// let src = Both;
/// let got = to_string_with_fallback!( WithDisplay, WithDebug, src );
/// let exp = "This is display".to_string();
/// // The primary formatting method WithDisplay is used.
/// assert_eq!( got, exp );
///
/// // Example usage: Using OnlyDebug which implements only Debug.
/// let src = OnlyDebug;
/// let got = to_string_with_fallback!( WithDisplay, WithDebug, src );
/// let exp = "This is debug".to_string();
/// // The primary formatting method WithDisplay is not available, so the fallback WithDebug is used.
/// assert_eq!( got, exp );
/// ```

#[ macro_export( local_inner_macros ) ]
macro_rules! to_string_with_fallback
{
  ( $how : ty, $fallback : ty, $src : expr )
  =>
  {{
    use assistant::ToStringWithFallback;
    (
      &assistant::Ref::< '_, _, assistant::ToStringWithFallbackParams< $how, $fallback > >::from( &$src )
    )
    .to_string_with_fallback()
  }};
}

// ==

/// Trait to convert a type to a string using a specified formatting method.
pub trait ToStringWith< How >
{
  /// Converts the type to a string using the specified formatting method.
  fn to_string_with( &self ) -> String;
}

impl< T > ToStringWith< WithDebug > for T
where
  T : fmt::Debug,
{
  /// Converts the type to a string using Debug formatting.
  fn to_string_with( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl< T > ToStringWith< WithDisplay > for T
where
  T : fmt::Display,
{
  /// Converts the type to a string using Display formatting.
  fn to_string_with( &self ) -> String
  {
    format!( "{}", self )
  }
}
