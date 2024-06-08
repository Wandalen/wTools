//!
//! Flexible ToString augmentation.
//!

/// Internal namespace.
pub( crate ) mod private
{

  pub use super::
  {
    aref::ToStringWithFallbackRef,
    params::ToStringWithFallbackParams,
  };

  use crate::ToStringWith;

  // ==

  /// Trait to convert a type to a string with a fallback formatting.
  pub trait _ToStringWithFallback< How, Fallback >
  {
    /// Converts the type to a string using the specified formatting or a fallback.
    fn to_string_with_fallback( self ) -> String
    ;
  }

  impl< T, How, Fallback > _ToStringWithFallback< How, Fallback >
  for ToStringWithFallbackRef< '_, T, ToStringWithFallbackParams< How, Fallback > >
  where
    T : ToStringWith< Fallback >,
  {
    /// Converts the type to a string using the specified formatting.
    fn to_string_with_fallback( self ) -> String
    {
      < T as ToStringWith< Fallback > >::to_string_with( self.0 )
    }
  }

  impl< T, How, Fallback > _ToStringWithFallback< How, Fallback >
  for &ToStringWithFallbackRef< '_, T, ToStringWithFallbackParams< How, Fallback > >
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
  /// use format_tools::
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
      use format_tools::_ToStringWithFallback;
      (
        &format_tools::ToStringWithFallbackRef::< '_, _, format_tools::ToStringWithFallbackParams< $how, $fallback > >::from( &$src )
      )
      .to_string_with_fallback()
    }};
  }

}

mod aref;
mod params;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    ToStringWithFallbackRef,
    ToStringWithFallbackParams,
    _ToStringWithFallback,
  };
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  use crate::to_string_with_fallback;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
