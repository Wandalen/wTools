mod private
{

  use crate::*;

  use action::features::FeaturesOptions;
  use std::fs;
  use std::path::PathBuf;
  // // use pth::AbsolutePath;
  use wca::VerifiedCommand;
  // use error::Result;
  // qqq : group dependencies
  // Explicit import for Result and its variants for pattern matching
  use std::result::Result::{Ok, Err};

  ///
  /// List features of a package.
  ///
  /// # Errors
  /// qqq: doc
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn features( o : VerifiedCommand ) -> error::untyped::Result< () > // qqq : use typed error
  {
    let path : PathBuf = o.args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    // qqq : dont use canonicalizefunction. path does not have exist
    let crate_dir = CrateDir::try_from( fs::canonicalize( path )? )?;
    let with_features_deps = o
    .props
    .get_owned( "with_features_deps" )
    .unwrap_or( false );
    let o = FeaturesOptions::former()
    .crate_dir( crate_dir )
    .with_features_deps( with_features_deps )
    .form();
    let report = action::features( o );
    match report
    {
      Ok( success ) => println!( "{success}" ),
      Err( failure ) => eprintln!( "{failure}" ),
    }
    Ok( () )
  }

}

crate::mod_interface!
{
  /// List features.
  orphan use features;
}

