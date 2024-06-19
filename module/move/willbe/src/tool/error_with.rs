mod private
{
  // xxx : aaa : for Nikita : for Petro : for Bohdan : good one, apply it to all code
  // added error trait
  /// The `ErrWith` trait provides a way to wrap errors in a custom error type, 
  /// allowing you to handle and transform
  /// errors in a more flexible and expressive way.
  pub trait ErrWith< V, R, E >
  {
    /// Takes a closure `f` that returns a value of type `V`, and uses it to wrap an error of type `(V, E1)`
    /// in the context of a `Result` of type `R`.
    fn err_with< F >( self, f : F ) -> std::result::Result< R, ( V, E ) >
    where
      F : FnOnce() -> V;
  }

  impl< V, R, E1, E2 > ErrWith< V, R, E1 > for Result< R, E2 >
  where
    E2 : Into< E1 >,
  {
    fn err_with< F >( self, f : F ) -> Result< R, ( V, E1 ) >
    where
      F : FnOnce() -> V,
    {
      self.map_err( | e | ( f(), e.into() ) )
    }
  }
}

crate::mod_interface!
{
  orphan use ErrWith;
}
