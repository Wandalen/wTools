/// Internal namespace.
pub( crate ) mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use ::error_tools::protected::*;

  pub trait ErrWith< V, R, E >
  {
    fn err_with( self, v : V ) -> std::result::Result< R, ( V, E ) >;
  }

  impl< V, R, E > ErrWith< V, R, E > for std::result::Result< R, E >
  {
    fn err_with( self, v : V ) -> std::result::Result< R, ( V, E ) >
    {
      self.map_err( | e | ( v, e ) )
    }
  }

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
