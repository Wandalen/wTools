//!
//! Flexible ToString augmentation.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  pub use super::
  {
    aref::{ Ref, Ref2, Ref3, Ref4 },
  };

  use std::
  {
    borrow::Cow,
  };

  // ==

  /// Trait to convert a type to a string with a fallback formatting.
  pub trait ToStringWithFallback< 'a, How, Fallback >
  where
    How : 'static,
    Fallback : 'static,
  {
    /// Converts the type to a string using the specified formatting or a fallback.
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    where
      // Self : 'static,
    ;
  }

  impl< 'a, T, How, Fallback > ToStringWithFallback< 'a, How, Fallback >
  for Ref< 'a, T, How, Fallback >
  where
    T : ToStringWith< How > + ?Sized,
    // crate::to_string::Ref< 'a, T, How > : crate::to_string::ToStringWith< How >,
    // T : crate::to_string::ToStringWith< How >,
    How : 'static,
    Fallback : 'static,
  {
    /// Converts the type to a string using the fallback formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    where
    {
      // let r : crate::to_string::Ref< 'a, T, How > = crate::to_string::Ref::from( self.0.0.0.0 );
      // r.to_string_with()
      self.0.0.0.0.to_string_with()
    }
  }

  impl< 'a, T, How, Fallback > ToStringWithFallback< 'a, How, Fallback >
  for Ref2< 'a, T, How, Fallback >
  where
    T : ToStringWith< How > + ?Sized,
    // crate::to_string::Ref2< 'a, T, How > : crate::to_string::ToStringWith< How >,
    // T : crate::to_string::ToStringWith< How >,
    How : 'static,
    Fallback : 'static,
  {
    /// Converts the type to a string using the fallback formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    {
      self.0.0.0.to_string_with()
      // crate::to_string::Ref::from( self.0.0.0 ).0.to_string_with()
    }
  }

  impl< 'a, T, How, Fallback > ToStringWithFallback< 'a, How, Fallback >
  for Ref3< 'a, T, How, Fallback >
  where
    T : ToStringWith< Fallback > + ?Sized,
    // crate::to_string::Ref< 'a, T, Fallback > : crate::to_string::ToStringWith< Fallback >,
    // T : crate::to_string::ToStringWith< How >,
    How : 'static,
    Fallback : 'static,
  {
    /// Converts the type to a string using the specified formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    {
      // < T as ToStringWith< Fallback > >::to_string_with( self.0.0 )
      self.0.0.to_string_with()
      // crate::to_string::Ref::from( self.0.0 ).to_string_with()
    }
  }

  impl< 'a, T, How, Fallback > ToStringWithFallback< 'a, How, Fallback >
  for Ref4< 'a, T, How, Fallback >
  where
    T : ToStringWith< Fallback > + ?Sized,
    // crate::to_string::Ref2< 'a, T, Fallback > : crate::to_string::ToStringWith< Fallback >,
    // T : crate::to_string::ToStringWith< How >,
    How : 'static,
    Fallback : 'static,
  {
    /// Converts the type to a string using the specified formatting.
    #[ inline ]
    fn to_string_with_fallback( self ) -> Cow< 'a, str >
    {
      // < T as ToStringWith< Fallback > >::to_string_with( self.0 )
      self.0.to_string_with()
      // crate::to_string::Ref::from( self.0 ).0.to_string_with()
    }
  }

  // impl< T, How, Fallback > ToStringWithFallback< How, Fallback >
  // for &Ref< '_, T, How, Fallback >
  // where
  //   T : ToStringWith< How >,
  // {
  //   /// Converts the type to a string using the fallback formatting.
  //   fn to_string_with_fallback( self ) -> String
  //   {
  //     < T as ToStringWith< How > > ::to_string_with( self.0 )
  //   }
  // }

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
  /// ```

  // #[ macro_export( local_inner_macros ) ]
  #[ macro_export ]
  macro_rules! to_string_with_fallback
  {
    ( $how : ty, $fallback : ty, $src : expr )
    =>
    {{
      use $crate::ToStringWithFallback;
      $crate
      ::to_string_with_fallback
      ::Ref
      ::< '_, _, $how, $fallback >
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
    Ref4,
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
    // Ref,
    ToStringWithFallback,
    // to_string_with_fallback,
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
    // Ref,
    // ToStringWithFallback,
    // to_string_with_fallback,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
