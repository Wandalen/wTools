mod private
{
  use crate::*;
  
  use std::path::PathBuf;
  use former::Former;
  use process::CmdReport;
  use wtools::error::Result;

  /// Represents pack options
  #[ derive( Debug, Former ) ]
  pub struct PackOptions
  {
    path : PathBuf,
    temp_path : Option< PathBuf >,
    dry : bool,
  }
  
  impl PackOptionsFormer
  {
    pub fn option_temp_path( mut self, value : impl Into< Option< PathBuf > > ) -> Self
    {
      self.container.temp_path = value.into();
      self
    }
  }
  
  impl PackOptions
  {
    fn to_pack_args( &self ) -> Vec< String >
    {
      [ "package".to_string() ]
      .into_iter()
      .chain( self.temp_path.clone().map( | p | vec![ "--target-dir".to_string(), p.to_string_lossy().into() ] ).into_iter().flatten() )
      .collect()
    }
  }
  
  ///
  /// Assemble the local package into a distributable tarball.
  ///
  /// # Args :
  /// - `path` - path to the package directory
  /// - `dry` - a flag that indicates whether to execute the command or not
  ///
  pub fn pack( args : PackOptions ) -> Result< CmdReport >
  {
    let ( program, options ) = ( "cargo", args.to_pack_args() );

    if args.dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", options.join( " " ) ),
          path : args.path.to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::run(program, options, args.path )
    }
  }


  /// Represents the arguments for the publish.
  #[ derive( Debug, Former, Clone, Default ) ]
  pub struct PublishOptions
  {
    path : PathBuf,
    temp_path : Option< PathBuf >,
    dry : bool, 
  }
  
  impl PublishOptionsFormer
  {
    pub fn option_temp_path( mut self, value : impl Into< Option< PathBuf > > ) -> Self
    {
      self.container.temp_path = value.into();
      self
    }
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
  pub fn publish( args : PublishOptions ) -> Result< CmdReport >
  {
    let ( program, arguments) = ( "cargo", args.as_publish_args() );

    if args.dry
    {
      Ok
        (
          CmdReport
          {
            command : format!( "{program} {}", arguments.join( " " ) ),
            path : args.path.to_path_buf(),
            out : String::new(),
            err : String::new(),
          }
        )
    }
    else
    {
      process::run(program, arguments, args.path )
    }
  }
}

//

crate::mod_interface!
{
  protected use pack;
  protected use publish;

  protected use PublishOptions;
  protected use PackOptions;

}
