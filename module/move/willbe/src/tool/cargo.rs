mod private
{
  use std::ffi::OsString;
  use crate::*;

  use std::path::PathBuf;
  use error_tools::err;
  use error_tools::for_app::format_err;
  use former::Former;
  use process_tools::process::*;
  use wtools::error::Result;

  /// Represents pack options
  #[ derive( Debug, Former, Clone ) ]
  pub struct PackOptions
  {
    pub( crate ) path : PathBuf,
    #[ former( default = false ) ]
    pub( crate ) allow_dirty : bool,
    #[ former( default = false ) ]
    pub( crate ) no_verify : bool,
    pub( crate ) temp_path : Option< PathBuf >,
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
      [ "package".to_string() ]
      .into_iter()
      .chain( if self.allow_dirty { Some( "--allow-dirty".to_string() ) } else { None } )
      .chain( if self.no_verify { Some( "--no-verify".to_string() ) } else { None } )
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
  #[ cfg_attr
  (
    feature = "tracing",
    track_caller,
    tracing::instrument( fields( caller = ?{ let x = std::panic::Location::caller(); ( x.file(), x.line() ) } ) )
  )]
  pub fn pack( args : PackOptions ) -> Result< Report >
  {
    let ( program, options ) = ( "cargo", args.to_pack_args() );

    if args.dry
    {
      Ok
      (
        Report
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
      Run::former()
      .bin_path( program )
      .args( options.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .current_path( args.path )
      .run().map_err( | report | err!( report.to_string() ) )
    }
  }


  /// Represents the options for the publish.
  #[ derive( Debug, Former, Clone, Default ) ]
  pub struct PublishOptions
  {
    pub( crate ) path : PathBuf,
    pub( crate ) temp_path : Option< PathBuf >,
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
      [ "publish".to_string() ].into_iter().chain( target_dir.into_iter().flatten() ).collect::< Vec< String > >()
    }
  }

 /// Upload a package to the registry
  #[ cfg_attr
  (
    feature = "tracing",
    track_caller,
    tracing::instrument( fields( caller = ?{ let x = std::panic::Location::caller(); ( x.file(), x.line() ) } ) )
  )]
  pub fn publish( args : PublishOptions ) -> Result< Report >
  {
    let ( program, arguments) = ( "cargo", args.as_publish_args() );

    if args.dry
    {
      Ok
        (
          Report
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
      let run_args =  arguments.into_iter().map( OsString::from ).collect::< Vec< _ > >();
      for _ in 0 .. args.retry_count + 1
      {
        let result = Run::former()
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
        Err( format_err!( "It took {} attempts, but still failed. Here are the errors:\n{}", args.retry_count + 1, results.into_iter().map( | r | format!( "- {r}" ) ).collect::< Vec< _ > >().join( "\n" ) ) )
      }
      else
      {
        Err( results.remove( 0 ) ).map_err( | report  | err!( report.to_string() ) )
      }
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
