/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use ::error_tools::protected::*;

  /// This trait allows adding extra context or information to an error, creating a tuple of the additional
  /// context and the original error. This is particularly useful for error handling when you want to include
  /// more details in the error without losing the original error value.
  ///
  /// The `ErrWith` trait provides methods to wrap an error with additional context, either by using a closure
  /// that generates the context or by directly providing the context.
  ///
  /// ```
  pub trait ErrWith< ReportErr, ReportOk, E >
  {
    /// Takes a closure `f` that returns a value of type `ReportErr`, and uses it to wrap an error of type `(ReportErr, E)`
    /// in the context of a `Result` of type `ReportOk`.
    ///
    /// This method allows you to add additional context to an error by providing a closure that generates the context.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that returns the additional context of type `ReportErr`.
    ///
    /// # Returns
    ///
    /// A `Result` of type `ReportOk` if the original result is `Ok`, or a tuple `(ReportErr, E)` containing the additional
    /// context and the original error if the original result is `Err`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result : Result< (), io::Error > = Err( io::Error::new( io::ErrorKind::Other, "an error occurred" ) );
    /// let result_with_context = result.err_with( || "additional context" );
    /// ```
    fn err_with< F >( self, f : F ) -> std::result::Result< ReportOk, ( ReportErr, E ) >
    where
      F : FnOnce() -> ReportErr;

    /// Takes a reference to a `ReportErr` value and uses it to wrap an error of type `(ReportErr, E)`
    /// in the context of a `Result` of type `ReportOk`.
    ///
    /// This method allows you to add additional context to an error by providing a reference to the context.
    ///
    /// # Arguments
    ///
    /// * `report` - A reference to the additional context of type `ReportErr`.
    ///
    /// # Returns
    ///
    /// A `Result` of type `ReportOk` if the original result is `Ok`, or a tuple `(ReportErr, E)` containing the additional
    /// context and the original error if the original result is `Err`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result : Result< (), io::Error > = Err( io::Error::new( io::ErrorKind::Other, "an error occurred" ) );
    /// let report = "additional context";
    /// let result_with_report = result.err_with_report( &report )?;
    /// ```
    fn err_with_report( self, report : &ReportErr ) -> std::result::Result< ReportOk, ( ReportErr, E ) >
    where
      ReportErr : Clone;
  }

  impl< ReportErr, ReportOk, E, IntoError > ErrWith< ReportErr, ReportOk, E >
  for std::result::Result< ReportOk, IntoError >
  where
    IntoError : Into< E >,
  {

    fn err_with< F >( self, f : F ) -> std::result::Result< ReportOk, ( ReportErr, E ) >
    where
      F : FnOnce() -> ReportErr,
    {
      self.map_err( | e | ( f(), e.into() ) )
    }

    #[ inline( always ) ]
    fn err_with_report( self, report : &ReportErr ) -> std::result::Result< ReportOk, ( ReportErr, E ) >
    where
      ReportErr : Clone,
      Self : Sized,
    {
      self.map_err( | e | ( report.clone(), e.into() ) )
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
