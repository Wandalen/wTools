/// Internal namespace.
mod private
{
  use crate::*;

  use collection::HashSet;
  use std::fs;
  use colored::Colorize;
  use wca::VerifiedCommand;
  use error::Result;
  // qqq : group dependencies
  use path::{ AbsolutePath, PathBuf };
  use action::test::TestsCommandOptions;
  use former::Former;
  use channel::Channel;
  use error::untyped::bail;
  use optimization::Optimization;

  #[ derive( Former, Debug ) ]
  struct TestsProperties
  {
    #[ former( default = true ) ]
    dry : bool,
    #[ former( default = true ) ]
    with_stable : bool,
    #[ former( default = false ) ]
    with_nightly : bool,
    #[ former( default = 0u32 ) ]
    concurrent : u32,
    #[ former( default = 1u32 ) ]
    power : u32,
    include : Vec< String >,
    #[ former( default  = [ "full".to_string(), "default".to_string() ] ) ]
    exclude : Vec< String >,
    #[ former( default = true ) ]
    temp : bool,
    enabled_features : Vec< String >,
    #[ former( default = true ) ]
    with_all_features : bool,
    #[ former( default = true ) ]
    with_none_features : bool,
    #[ former( default = true ) ]
    with_debug : bool,
    #[ former( default = false ) ]
    with_release : bool,
    #[ former( default = true ) ]
    with_progress : bool,
  }

  /// run tests in specified crate
  // qqq : don't use 1-prameter Result
  pub fn test( o : VerifiedCommand ) -> Result< () >
  {
    let args_line = format!
    (
      "{}",
      o
      .args
      .get_owned( 0 )
      .unwrap_or( std::path::PathBuf::from( "" ) )
      .display()
    );
    let prop_line = format!
    (
      "{}",
      o
      .props
      .iter()
      .map( | p | format!( "{}:{}", p.0, p.1.to_string() ) )
      .collect::< Vec< _ > >().join(" ")
    );

    let path : PathBuf = o.args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = AbsolutePath::try_from( fs::canonicalize( path )? )?;
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
    } = o.props.try_into()?;

    let mut channels = HashSet::new();
    if with_stable { channels.insert( Channel::Stable ); }
    if with_nightly { channels.insert( Channel::Nightly ); }

    let mut optimizations = HashSet::new();
    if with_release { optimizations.insert( Optimization::Release ); }
    if with_debug { optimizations.insert( Optimization::Debug ); }

    if optimizations.is_empty()
    {
      bail!( "Cannot run tests if with_debug and with_release are both false. \
Set at least one of them to true." );
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
        if dry
        {
          let args = if args_line.is_empty() { String::new() } else { format!(" {}", args_line) };
          let prop = if prop_line.is_empty() { String::new() } else { format!(" {}", prop_line) };
          let line = format!("will .publish{}{} dry:0", args, prop);
          println!("To apply plan, call the command `{}`", line.blue());
        }
        else
        {
          println!( "{report} ");
        }

        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" );
        Err( e.context( "package test command" ) )
      }
    }
  }

  impl TryFrom< wca::Props > for TestsProperties
  {
    type Error = error::untyped::Error;
    fn try_from( value : wca::Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value
      .get_owned( "dry" ) { this.dry::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "temp" ) { this.temp::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_stable" ) { this.with_stable::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_nightly" ) { this.with_nightly::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "concurrent" ) { this.concurrent::< u32 >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "power" ) { this.power::< u32 >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "include" ) { this.include::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "exclude" ) { this.exclude::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_debug" ) { this.with_debug::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_release" ) { this.with_release::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_all_features" ) { this.with_all_features::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_none_features" ) { this.with_none_features::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "always" ) { this.enabled_features::< Vec< String > >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_progress" ) { this.with_progress::< bool >( v ) } else { this };

      Ok( this.form() )
    }
  }
}

crate::mod_interface!
{
  /// run tests in specified crate
  exposed use test;
}