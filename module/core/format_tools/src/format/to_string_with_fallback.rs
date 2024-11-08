//!
//! Flexible ToString augmentation.
//!

/// Define a private namespace for all its items.
mod private
{
  use crate::*;

  pub use super::
  {
    aref::{ Ref, Ref2, Ref3 },
  };

  use std::
  {
    borrow::Cow,
  };

  // ==

  /// Trait to convert a type to a string with a fallback formatting.
  pub trait ToStringWithFallback< 'a, How, Fallback1, Fallback2 >
  where
    How : 'static,
    Fallback1 : 'static,
    Fallback2 : 'static,
  {
    /// Converts the type to a string using the specified formatting or a fallback.
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    ;
  }

  impl< 'a, T, How, Fallback1, Fallback2 > ToStringWithFallback< 'a, How, Fallback1, Fallback2 >
  for Ref< 'a, T, How, Fallback1, Fallback2 >
  where
    T : ToStringWith< How > + ?Sized,
    How : 'static,
    Fallback1 : 'static,
    Fallback2 : 'static,
  {
    /// Converts the type to a string using the fallback formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    where
    {
      self.0.0.0.to_string_with()
    }
  }

  impl< 'a, T, How, Fallback1, Fallback2 > ToStringWithFallback< 'a, How, Fallback1, Fallback2 >
  for Ref2< 'a, T, How, Fallback1, Fallback2 >
  where
    T : ToStringWith< Fallback1 > + ?Sized,
    How : 'static,
    Fallback1 : 'static,
    Fallback2 : 'static,
  {
    /// Converts the type to a string using the fallback formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    {
      self.0.0.to_string_with()
    }
  }

  impl< 'a, T, How, Fallback1, Fallback2 > ToStringWithFallback< 'a, How, Fallback1, Fallback2 >
  for Ref3< 'a, T, How, Fallback1, Fallback2 >
  where
    T : ToStringWith< Fallback2 > + ?Sized,
    How : 'static,
    Fallback1 : 'static,
    Fallback2 : 'static,
  {
    /// Converts the type to a string using the specified formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    {
      self.0.to_string_with()
    }
  }

  //

  /// Macro to convert a value to a string using a specified formatting method with a fallback.
  ///
  /// # Parameters
  /// - `$how`: The primary formatting type (e.g., `WithDebug`, `WithDisplay`).
  /// - `$fallback1`: The first fallback formatting type.
  /// - `$fallback2`: The second fallback formatting type (optional).
  /// - `$src`: The source value to format.
  ///
  /// # Example
  /// ```rust
  /// use core::fmt;
  /// use format_tools::
  /// {
  ///   WithRef,
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
  /// let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  /// let exp = "This is display".to_string();
  /// // The primary formatting method WithDisplay is used.
  /// assert_eq!( got, exp );
  ///
  /// // Example usage: Using OnlyDebug which implements only Debug.
  /// let src = OnlyDebug;
  /// let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  /// let exp = "This is debug".to_string();
  /// // The primary formatting method WithDisplay is not available, so the fallback WithDebug is used.
  /// assert_eq!( got, exp );
  ///
  /// // Example usage: Using a struct that might need a second fallback.
  /// struct OnlyDebugFallback;
  ///
  /// impl fmt::Debug for OnlyDebugFallback
  /// {
  ///   fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  ///   {
  ///     write!( f, "This is debug fallback" )
  ///   }
  /// }
  ///
  /// // Example usage: Using OnlyDebugFallback which implements only Debug.
  /// let src = OnlyDebugFallback;
  /// let got = to_string_with_fallback!( WithRef, WithDisplay, WithDebug, &src );
  /// let exp = "This is debug fallback".to_string();
  /// // The primary formatting method WithDisplay is not available, so the second fallback WithDebugFallback is used.
  /// assert_eq!( got, exp );
  /// ```

  #[ macro_export ]
  macro_rules! to_string_with_fallback
  {

    ( $how : ty, $fallback1 : ty, $src : expr )
    =>
    {{
      use $crate::ToStringWithFallback;
      $crate
      ::to_string_with_fallback
      ::Ref
      ::< '_, _, $how, $fallback1, $fallback1 >
      ::from( $src )
      .to_string_with_fallback()
    }};

    ( $how : ty, $fallback1 : ty, $fallback2 : ty, $src : expr )
    =>
    {{
      use $crate::ToStringWithFallback;
      $crate
      ::to_string_with_fallback
      ::Ref
      ::< '_, _, $how, $fallback1, $fallback2 >
      ::from( $src )
      .to_string_with_fallback()
    }};

  }

  pub use to_string_with_fallback;
}

mod aref;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::
  {
    Ref,
    Ref2,
    Ref3,
    to_string_with_fallback,
  };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use super::super::to_string_with_fallback;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
    ToStringWithFallback,
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
