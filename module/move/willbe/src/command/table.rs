mod private
{
  use crate::*;
  use wtools::error;
  use wtools::error::for_app::Context;

  ///
  /// Generate table.
  ///
  pub fn table_generate( ( _, _ ) : ( wca::Args, wca::Props ) ) -> error::Result< () >
  {
    endpoint::table_create().context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// List packages.
  orphan use table_generate;
}
