mod private
{
  use crate::*;

  use std::
  {
    path::Path,
  };
  use std::path::PathBuf;
  use former::Former;
  use process::CmdReport;
  use wtools::error::Result;

  ///
  /// Assemble the local package into a distributable tarball.
  ///
  /// # Args :
  /// - `path` - path to the package directory
  /// - `dry` - a flag that indicates whether to execute the command or not
  ///
  pub fn pack< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, options ) = ( "cargo", [ "package" ] );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", options.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::run(program, options, path )
    }
  }


  /// Represents the arguments for the publish.
  #[ derive( Debug, Former, Clone, Default ) ]
  pub struct PublishOptions
  {
    temp_path : Option< PathBuf >,
  }

  impl PublishOptions
  {
    fn as_publish_args( &self ) -> Vec< String >
    {
      let target_dir = self.temp_path.clone().map( | p | vec![ "--target-dir".to_string(), p.to_string_lossy().into() ] );
      [ "publish".to_string() ].into_iter().chain( target_dir.into_iter().flatten() ).collect::< Vec< String > >()
    }
  }
  
 /// Upload a package to the registry
  pub fn publish< P >(path : P, args : PublishOptions, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, arguments) = ( "cargo", args.as_publish_args() );

    if dry
    {
      Ok
        (
          CmdReport
          {
            command : format!( "{program} {}", arguments.join( " " ) ),
            path : path.as_ref().to_path_buf(),
            out : String::new(),
            err : String::new(),
          }
        )
    }
    else
    {
      process::run(program, arguments, path )
    }
  }
}

//

crate::mod_interface!
{
  protected use pack;
  protected use publish;
  
  protected use PublishOptions;

}
