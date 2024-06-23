mod private
{
  use crate::*;

  use std::fs;
  use std::path::PathBuf;
  use wca::VerifiedCommand;

  use error::Result;
  // qqq : group dependencies
  // use path::AbsolutePath;

  #[ derive( former::Former ) ]
  struct PublishDiffProperties
  {
    keep_archive : Option< PathBuf >,
  }

  /// Command to display the differences between a local and remote package versions.
  ///
  /// # Arguments
  ///
  /// * `args` - Command line arguments.
  ///
  /// # Returns
  ///
  /// Returns a `Result` indicating success or failure.
  ///
  /// # Errors
  ///
  /// Returns an error if there is an issue with the command.

  // qqq : don't use 1-prameter Result
  pub fn publish_diff( o : VerifiedCommand ) -> Result< () >
  {
    let path : PathBuf = o.args.get_owned( 0 ).unwrap_or( std::env::current_dir()? );
    let PublishDiffProperties { keep_archive } = o.props.try_into()?;

    let mut o = action::PublishDiffOptions::former()
    .path( path );
    if let Some( k ) = keep_archive.clone() { o = o.keep_archive( k ); }
    let o = o.form();

    println!( "{}", action::publish_diff( o )? );
    if let Some( keep ) = keep_archive
    {
      let keep = AbsolutePath::try_from( fs::canonicalize( keep )? ).unwrap();
      println!( "Remote version of the package was saved at `{}`", keep.as_ref().display() );
    }

    Ok( () )
  }

  impl TryFrom< wca::Props > for PublishDiffProperties
  {
    type Error = error::untyped::Error;
    fn try_from( value : wca::Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value
      .get_owned( "keep_archive" )
      { this.keep_archive::< PathBuf >( v ) }
      else
      { this };

      Ok( this.form() )
    }
  }
}

//

crate::mod_interface!
{
  /// Publishes the difference between the local and published versions of a package.
  orphan use publish_diff;
}
