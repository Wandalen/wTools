/// Internal namespace.
mod private
{
  use crate::*;

  use std::collections::HashSet;
  use std::path::PathBuf;


  use wca::{ Args, Props };
  use wtools::error::Result;
  use path::AbsolutePath;
  use endpoint::test::TestsCommandOptions;
  use former::Former;
  use cargo::Channel;

  #[ derive( Former ) ]
  struct TestsProperties
  {
    #[ default( true ) ]
    dry : bool,
    #[ default( true ) ]
    with_stable : bool,
    #[ default( true ) ]
    with_nightly : bool,
    #[ default( 0u32 ) ]
    concurrent : u32,
    #[ default( 1u32 ) ]
    power : u32,
    include : Vec< String >,
    exclude : Vec< String >,
  }

  /// run tests in specified crate
  pub fn test( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let path : PathBuf = args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = AbsolutePath::try_from( path )?;
    let TestsProperties { dry, with_stable, with_nightly, concurrent, power, include, exclude } = properties.try_into()?;

    let mut channels = HashSet::new();
    if with_stable { channels.insert( Channel::Stable ); }
    if with_nightly { channels.insert( Channel::Nightly ); }

    let args = TestsCommandOptions::former()
    .dir( path )
    .concurrent( concurrent )
    .channels( channels )
    .power( power )
    .exclude_features( exclude )
    .include_features( include )
    .form();

    match endpoint::test( args, dry )
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
      this = if let Some( v ) = value.get_owned( "with_stable" ) { this.with_stable::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_nightly" ) { this.with_nightly::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "concurrent" ) { this.concurrent::< u32 >( v ) } else { this };
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
  exposed use test;
}