/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use ::error_tools::protected::*;

  /// This trait can be used to add extra information to an error, creating a tuple of the additional
  /// context and the original error. This can be particularly useful for error handling where you
  /// want to include more context or details in the error without losing the original error value.
  pub trait ErrWith< V, R, E >
  {
    /// Adds additional context to an error, converting the error into a tuple of the context and the error.
    fn err_with( self, v : V ) -> std::result::Result< R, ( V, E ) >;
  }

  impl< V, R, E > ErrWith< V, R, E > for std::result::Result< R, E >
  {
    fn err_with( self, v : V ) -> std::result::Result< R, ( V, E ) >
    {
      self.map_err( | e | ( v, e ) )
    }
  }

  /// A type alias for a `Result` that contains an error which is a tuple of a report and an original error.
  ///
  /// This is useful when you want to report additional information along with an error. The `ResultWithReport` type
  /// helps in defining such results more concisely.
  pub type ResultWithReport< Report, Error > = Result< Report, ( Report, Error ) >;


}

crate::mod_interface!
{
  // #![ debug ]

  use ::error_tools;
  protected use ::error_tools::protected::*;

  exposed use ErrWith;
  exposed use ResultWithReport;

}
