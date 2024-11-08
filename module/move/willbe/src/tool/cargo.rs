/// Define a private namespace for all its items.
mod private
{
  use crate::*;

  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use std::ffi::OsString;
  use std::path::PathBuf;
  // use error::err;
  // use error::untyped::format_err;
  use former::Former;
  use process_tools::process;
  // use process_tools::process::*;
  // qqq : for Bohdan : bad
  // use error::Result;
  // qqq : group dependencies

  // qqq : for Bohdan : bad : tools can't depend on entitties!
  use crate::channel::Channel;

  // aaa : documentation /// aaa : documented

  /// Represents options for packaging a project.
  ///
  /// The `PackOptions` struct encapsulates various options that can be configured when packaging a project,
  /// including the path to the project, the distribution channel, and various flags for controlling the behavior of the packaging process.
  #[ derive( Debug, Former, Clone ) ]
  pub struct PackOptions
  {
    /// The path to the project to be packaged.
    ///
    /// This field specifies the file system path where the project is located.
    pub( crate ) path : PathBuf,
    /// The distribution channel for the packaging project.
    ///
    /// This field specifies the channel through which the packaged project will be distributed.
    ///
    pub( crate ) channel : Channel,
    /// Flag indicating whether to allow packaging even if the working directory is dirty.
    ///
    /// This field is set to `true` by default, meaning that packaging will proceed even if there are uncommitted changes.
    #[ former( default = true ) ]
    pub( crate ) allow_dirty : bool,
    // qqq : rename to checking_changes
    /// Flag indicating whether to skip verification checks.
    #[ former( default = false ) ]
    // aaa : don't abuse negative form, rename to checking_consistency
    // renamed and changed logic
    pub( crate ) checking_consistency : bool,
    /// Setting this option to true will temporarily remove development dependencies before executing the command, then restore them afterward.
    #[ former( default = true ) ]
    pub( crate ) exclude_dev_dependencies : bool,
    /// An optional temporary path to be used during packaging.
    ///
    /// This field may contain a path to a temporary directory that will be used during the packaging process.
    pub( crate ) temp_path : Option< PathBuf >,
    /// Flag indicating whether to perform a dry run.
    ///
    /// This field specifies whether the packaging process should be a dry run, meaning that no actual changes will be made.
    pub( crate ) dry : bool,
  }

  impl PackOptionsFormer
  {
    pub fn option_temp_path( mut self, value : impl Into< Option< PathBuf > > ) -> Self
    {
      self.storage.temp_path = value.into();
      self
    }
  }

  impl PackOptions
  {
    fn to_pack_args( &self ) -> Vec< String >
    {
      [ "run".to_string(), self.channel.to_string(), "cargo".into(), "package".into() ]
      .into_iter()
      .chain( if self.allow_dirty { Some( "--allow-dirty".to_string() ) } else { None } )
      .chain( if !self.checking_consistency { Some( "--no-verify".to_string() ) } else { None } )
      .chain( self.temp_path.clone().map( | p | vec![ "--target-dir".to_string(), p.to_string_lossy().into() ] ).into_iter().flatten() )
      .collect()
    }
  }

  #[ derive( Debug ) ]
  struct TemporaryManifestFile
  {
    original : PathBuf,
    temporary : PathBuf,
  }

  impl TemporaryManifestFile
  {
    /// Creates a backup copy of the original file, allowing the original file location to serve as a temporary workspace.
    /// When the object is dropped, the temporary file at the original location is replaced by the backup, restoring the original file.
    fn new( path : impl Into< PathBuf > ) -> error::untyped::Result< Self >
    {
      let path = path.into();
      if !path.ends_with( "Cargo.toml" )
      {
        error::untyped::bail!( "Wrong path to temporary manifest" );
      }

      let mut index = 0;
      let original = loop
      {
        let temp_path = PathBuf::from( format!( "{}.temp_{index}", path.display() ) );
        if !temp_path.exists()
        {
          _ = std::fs::copy( &path, &temp_path )?;
          break temp_path;
        }
        index += 1;
      };

      Ok( Self
      {
        original,
        temporary : path,
      })
    }
  }

  impl Drop for TemporaryManifestFile
  {
    fn drop( &mut self )
    {
      _ = std::fs::rename( &self.original, &self.temporary ).ok();
    }
  }

  ///
  /// Assemble the local package into a distributable tarball.
  ///
  /// # Args :
  /// - `path` - path to the package directory
  /// - `dry` - a flag that indicates whether to execute the command or not
  ///
  #[ cfg_attr
  (
    feature = "tracing",
    track_caller,
    tracing::instrument( fields( caller = ?{ let x = std::panic::Location::caller(); ( x.file(), x.line() ) } ) )
  )]
  // qqq : should be typed error, apply err_with
  // qqq : use typed error
  pub fn pack( args : PackOptions ) -> error::untyped::Result< process::Report >
  {
    let _temp = if args.exclude_dev_dependencies
    {
      let manifest = TemporaryManifestFile::new( args.path.join( "Cargo.toml" ) )?;
      let mut file = Manifest::try_from( ManifestFile::try_from( &manifest.temporary )? )?;
      let data = file.data();

      _ = data.remove( "dev-dependencies" );
      file.store()?;

      Some( manifest )
    } else { None };
    let ( program, options ) = ( "rustup", args.to_pack_args() );

    if args.dry
    {
      Ok
      (
        process::Report
        {
          command : format!( "{program} {}", options.join( " " ) ),
          out : String::new(),
          err : String::new(),
          current_path: args.path.to_path_buf(),
          error: Ok( () ),
        }
      )
    }
    else
    {
      process::Run::former()
      .bin_path( program )
      .args( options.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .current_path( args.path )
      .run().map_err( | report | error::untyped::format_err!( report.to_string() ) )
    }
  }


  /// Represents the options for the publish.
  #[ derive( Debug, Former, Clone, Default ) ]
  pub struct PublishOptions
  {
    pub( crate ) path : PathBuf,
    pub( crate ) temp_path : Option< PathBuf >,
    pub( crate ) exclude_dev_dependencies : bool,
    #[ former( default = 0usize ) ]
    pub( crate ) retry_count : usize,
    pub( crate ) dry : bool,
  }

  impl PublishOptionsFormer
  {
    pub fn option_temp_path( mut self, value : impl Into< Option< PathBuf > > ) -> Self
    {
      self.storage.temp_path = value.into();
      self
    }
  }

  impl PublishOptions
  {
    fn as_publish_args( &self ) -> Vec< String >
    {
      let target_dir = self.temp_path.clone().map( | p | vec![ "--target-dir".to_string(), p.to_string_lossy().into() ] );
      [ "publish".to_string() ].into_iter().chain( target_dir.into_iter().flatten() ).collect()
    }
  }

 /// Upload a package to the registry
  #[ cfg_attr
  (
    feature = "tracing",
    track_caller,
    tracing::instrument( fields( caller = ?{ let x = std::panic::Location::caller(); ( x.file(), x.line() ) } ) )
  )]
  pub fn publish( args : PublishOptions ) -> error::untyped::Result< process::Report >
  // qqq : use typed error
  {
    let _temp = if args.exclude_dev_dependencies
    {
      let manifest = TemporaryManifestFile::new( args.path.join( "Cargo.toml" ) )?;
      let mut file = Manifest::try_from( ManifestFile::try_from( &manifest.temporary )? )?;
      let data = file.data();

      _ = data.remove( "dev-dependencies" );
      file.store()?;

      Some( manifest )
    } else { None };
    let ( program, arguments) = ( "cargo", args.as_publish_args() );

    if args.dry
    {
      Ok
        (
          process::Report
          {
            command : format!( "{program} {}", arguments.join( " " ) ),
            out : String::new(),
            err : String::new(),
            current_path: args.path.to_path_buf(),
            error: Ok( () ),
          }
        )
    }
    else
    {
      let mut results = Vec::with_capacity( args.retry_count + 1 );
      let run_args : Vec< _ > =  arguments.into_iter().map( OsString::from ).collect();
      for _ in 0 .. args.retry_count + 1
      {
        let result = process::Run::former()
        .bin_path( program )
        .args( run_args.clone() )
        .current_path( &args.path )
        .run();
        match result
        {
          Ok( report ) => return Ok( report ),
          Err( e ) => results.push( e ),
        }
      }
      if args.retry_count > 0
      {
        Err( error::untyped::format_err!( "It took {} attempts, but still failed. Here are the errors:\n{}", args.retry_count + 1, results.into_iter().map( | r | format!( "- {r}" ) ).collect::< Vec< _ > >().join( "\n" ) ) )
      }
      else
      {
        Err( results.remove( 0 ) ).map_err( | report | error::untyped::format_err!( report.to_string() ) )
      }
    }
  }
}

//

crate::mod_interface!
{
  own use pack;
  own use publish;

  own use PublishOptions;
  own use PackOptions;

}