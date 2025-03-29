mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  use error::{ untyped::Context };

  ///
  /// Generate table.
  ///
  /// # Errors
  /// qqq: doc
  // qqq : typed error
  pub fn cicd_renew() -> error::untyped::Result< () >
  {
    action::cicd_renew::action
    (
      &std::env::current_dir()?
    )
    .context( "Fail to generate workflow" )
  }
}

crate::mod_interface!
{
  /// List packages.
  exposed use cicd_renew;
}

