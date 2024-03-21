/// Internal namespace.
mod private
{
  use crate::*;

  use std::collections::HashSet;
  use std::path::PathBuf;
  use wca::{ Args, Props };
  use wtools::error::Result;
  use _path::AbsolutePath;
  use action::test::TestsCommandOptions;
  use former::Former;
  use channel::Channel;
  use error_tools::for_app::bail;
  use optimization::Optimization;

  #[ derive( Former, Debug ) ]
  struct TestsProperties
  {
    #[ default( true ) ]
    dry : bool,
    #[ default( true ) ]
    with_stable : bool,
    #[ default( false ) ]
    with_nightly : bool,
    #[ default( 0u32 ) ]
    concurrent : u32,
    #[ default( 1u32 ) ]
    power : u32,
    include : Vec< String >,
    #[ default ( [ "full".to_string(), "default".to_string() ] ) ]
    exclude : Vec< String >,
    #[ default( true ) ]
    temp : bool,
    enabled_features : Vec< String >,
    #[ default( true ) ]
    with_all_features : bool,
    #[ default( true ) ]
    with_none_features : bool,
    #[ default( true ) ]
    with_debug : bool,
    #[ default( false ) ]
    with_release : bool,
    #[ default( true ) ]
    with_progress : bool,
  }

  /// run tests in specified crate
  pub fn test( args : Args, properties : Props ) -> Result< () >
  {
    let path : PathBuf = args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = AbsolutePath::try_from( path )?;
    let TestsProperties
    {
      dry,
      with_stable,
      with_nightly,
      concurrent,
      power,
      include,
      exclude,
      temp,
      enabled_features,
      with_all_features,
      with_none_features,
      with_debug,
      with_release, 
      with_progress
    } = properties.try_into()?;

    let mut channels = HashSet::new();
    if with_stable { channels.insert( Channel::Stable ); }
    if with_nightly { channels.insert( Channel::Nightly ); }

    let mut optimizations = HashSet::new();
    if with_release { optimizations.insert( Optimization::Release ); }
    if with_debug { optimizations.insert( Optimization::Debug ); }

    if optimizations.is_empty()
    {
      bail!( "Cannot run tests if with_debug and with_release are both false. Set at least one of them to true." );
    }


    let args = TestsCommandOptions::former()
    .dir( path )
    .concurrent( concurrent )
    .channels( channels )
    .power( power )
    .exclude_features( exclude )
    .include_features( include )
    .temp( temp )
    .enabled_features( enabled_features )
    .with_all_features( with_all_features )
    .with_none_features( with_none_features )
    .optimizations( optimizations )
    .with_progress( with_progress )
    .form();

    match action::test( args, dry )
    {
      Ok( report ) =>
      {
        println!( "{report} ");

        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" );
        Err( e.context( "package test command" ) )
      }
    }
  }

  impl TryFrom< Props > for TestsProperties
  {
    type Error = wtools::error::for_app::Error;
    fn try_from( value : Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value.get_owned( "dry" ) { this.dry::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "temp" ) { this.dry::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_stable" ) { this.with_stable::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_nightly" ) { this.with_nightly::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "concurrent" ) { this.concurrent::< u32 >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "power" ) { this.power::< u32 >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "include" ) { this.include::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "exclude" ) { this.exclude::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_debug" ) { this.with_debug::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_release" ) { this.with_release::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_all_features" ) { this.with_all_features::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_none_features" ) { this.with_none_features::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "always" ) { this.enabled_features::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_progress" ) { this.with_progress::< bool >( v ) } else { this };

      Ok( this.form() )
    }
  }
}

crate::mod_interface!
{
  /// run tests in specified crate
  exposed use test;
}