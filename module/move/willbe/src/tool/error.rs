/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use ::error_tools::protected::*;

  // qqq : for for Petro : for Bohdan : good one, apply it to all code

  /// This trait can be used to add extra information to an error, creating a tuple of the additional
  /// context and the original error. This can be particularly useful for error handling where you
  /// want to include more context or details in the error without losing the original error value.
  pub trait ErrWith< V, R, E >
  {
    /// Takes a closure `f` that returns a value of type `V`, and uses it to wrap an error of type `(V, E1)`
    /// in the context of a `Result` of type `R`.
    fn err_with< F >( self, f : F ) -> std::result::Result< R, ( V, E ) >
    where
      F : FnOnce() -> V;
  }

  impl< V, R, E1, E2 > ErrWith< V, R, E1 > for std::result::Result< R, E2 >
  where
    E2 : Into< E1 >,
  {
    fn err_with< F >( self, f : F ) -> std::result::Result< R, ( V, E1 ) >
    where
      F : FnOnce() -> V,
    {
      self.map_err( | e | ( f(), e.into() ) )
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
  exposed use ::error_tools::Result;

}
