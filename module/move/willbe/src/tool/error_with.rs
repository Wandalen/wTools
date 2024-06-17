mod private
{
	// xxx : aaa : for Nikita : for Petro : for Bohdan : good one, apply it to all code
  // added error trait
	pub trait ErrWith< V, R, E >
  {
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