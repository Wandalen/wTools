mod private
{
  use crate::*;

  use std::path::PathBuf;
  use _path::AbsolutePath;
  use wca::VerifiedCommand;
  use wtools::error::Result;

  ///
  /// List features.
  ///

  pub fn features( o : VerifiedCommand ) -> Result< () >
  {
    let path : PathBuf = o.args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = AbsolutePath::try_from( path )?;
    let report = action::features( path );
    match report
    {
      Ok(success) => println!("{success}"),
      Err(failure) => eprintln!("{failure}"),
    }
    Ok( () )
  }

}

crate::mod_interface!
{
  /// List features.
  orphan use features;
}

