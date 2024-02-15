/// Internal namespace.
mod private 
{
  use crate::*;

  use std::collections::HashSet;
  use std::path::PathBuf;


  use wca::{ Args, Props };
  use wtools::error::Result;
  use path::AbsolutePath;
  use endpoint::run_tests::TestsArgs;
  use former::Former;
  use cargo::Channel;

  #[ derive( Former ) ]
  struct RunTestsProperties
  {
    #[ default( true ) ]
    with_stable : bool,
    #[ default( false ) ]
    with_nightly : bool,
    #[ default( true ) ]
    parallel : bool,
    #[ default( 1u32 ) ]
    power : u32,
    include : Vec< String >,
    exclude : Vec< String >,
  }

  /// run tests in specified crate
	pub fn run_tests( ( args, properties ) : ( Args, Props ) ) -> Result< () >
	{
    let path : PathBuf = args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = AbsolutePath::try_from( path )?;
    let RunTestsProperties { with_stable, with_nightly, parallel, power, include, exclude } = properties.try_into()?;

    let crate_dir = CrateDir::try_from( path )?;

    let mut channels = HashSet::new();
    if with_stable { channels.insert( Channel::Stable ); }
    if with_nightly { channels.insert( Channel::Nightly ); }

    let args = TestsArgs::former()
    .dir( crate_dir )
    .parallel( parallel)
    .channels( channels )
    .power( power )
    .exclude_features( exclude )
    .include_features( include )
    .form();

    match endpoint::run_tests( args )
    {
      Ok( report ) =>
      {
        println!( "{report} ");
      }
      Err( e ) =>
      {
        return Err( e.context( "package test command" ) );
      }
    }

	 	Ok(())
	}

  impl TryFrom< Props > for RunTestsProperties
  {
    type Error = wtools::error::for_app::Error;
    fn try_from( value : Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value.get_owned( "with_stable" ) { this.with_stable::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_nightly" ) { this.with_nightly::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "parallel" ) { this.parallel::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "power" ) { this.power::< u32 >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "include" ) { this.include::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "exclude" ) { this.exclude::< Vec< String > >( v ) } else { this };

      Ok( this.form() )
    }
  }
}

crate::mod_interface!
{
  /// run tests in specified crate
  prelude use run_tests;
}