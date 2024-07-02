mod private
{
  use crate::*;

  use error::{ untyped::Context };

  ///
  /// Generate table.
  ///
  // qqq : typed error
  pub fn readme_health_table_renew() -> error::untyped::Result< () >
  {
    action::readme_health_table_renew
    (
      &std::env::current_dir()?
    )
    .context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// List packages.
  orphan use readme_health_table_renew;
}
