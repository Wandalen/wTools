/// Internal namespace.
pub( crate ) mod private
{
  pub use std::error::Error as ErrorInterface;

  ///
  /// Macro to generate an error descriptor.
  ///
  /// ### Basic use-case.
  /// ```rust
  /// # use error_tools::{ BasicError, err };
  /// fn f1() -> BasicError
  /// {
  ///   return err!( "No attr" );
  /// }
  /// ```
  ///

  #[ macro_export ]
  macro_rules! err
  {

    ( $msg : expr ) =>
    {
      $crate::BasicError::new( $msg ).into()
    };
    ( $msg : expr, $( $arg : expr ),+ $(,)? ) =>
    {
      $crate::BasicError::new( format!( $msg, $( $arg ),+ ) ).into()
    };

  }

  ///
  /// Macro to return an Err( error ) generating error descriptor.
  ///
  /// ### Basic use-case.
  /// ```rust
  /// # use error_tools::{ BasicError, return_err };
  /// fn f1() -> Result< (), BasicError >
  /// {
  ///   return_err!( "No attr" );
  /// }
  /// ```
  ///

  #[ macro_export ]
  macro_rules! return_err
  {

    ( $msg : expr ) =>
    {
      return Result::Err( $crate::err!( $msg ) )
    };
    ( $msg : expr, $( $arg : expr ),+ $(,)? ) =>
    {
      return Result::Err( $crate::err!( $msg, $( $arg ),+ ) )
    };

  }

  // xxx : deprecate maybe?
  /// baic implementation of generic BasicError

  #[ derive( core::fmt::Debug, core::clone::Clone, core::cmp::PartialEq, core::cmp::Eq ) ]
  pub struct BasicError
  {
    msg : String,
  }

  impl BasicError
  {
    /// Constructor expecting message with description.
    pub fn new< Msg : Into< String > >( msg : Msg ) -> BasicError
    {
      BasicError { msg : msg.into() }
    }
    /// Message with description getter.
    pub fn msg( &self ) -> &String
    {
      &self.msg
    }
  }

  impl core::fmt::Display for BasicError
  {
    fn fmt(&self, f: &mut core::fmt::Formatter< '_ >) -> core::fmt::Result
    {
      write!( f, "{}", self.msg )
    }
  }

  impl ErrorInterface for BasicError
  {
    fn description( &self ) -> &str
    {
      &self.msg
    }
  }

  impl< T > From< BasicError > for Result< T, BasicError >
  {
    /// Returns the argument unchanged.
    #[ inline( always ) ]
    fn from( src : BasicError ) -> Self
    {
      Result::Err( src )
    }
  }

  pub use err;
  pub use return_err;

  // qqq : write standard mod interface without using mod_interface /* aaa : Dmytro : added to each library file */
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  pub use super::private::err;
  pub use super::private::return_err;
  pub use super::private::ErrorInterface;
  pub use super::private::BasicError;
}
// xxx : review