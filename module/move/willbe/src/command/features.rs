mod private
{
  use crate::*;

  use action::features::FeaturesOptions;
  use std::path::PathBuf;
  use _path::AbsolutePath;
  use wca::VerifiedCommand;
  use wtools::error::Result;

  ///
  /// List features of a package.
  ///

  pub fn features( o : VerifiedCommand ) -> Result< () >
  {
    let path : PathBuf = o.args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = AbsolutePath::try_from( path )?;
    let with_features_deps = o.props.get_owned( "with_features_deps" ).unwrap_or( false );
    let options = FeaturesOptions::former()
    .manifest_dir( path )
    .with_features_deps( with_features_deps )
    .form();
    let report = action::features( options );
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

