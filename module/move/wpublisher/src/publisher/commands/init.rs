
/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use { commands };
  use wca::{ Type, CommandsAggregator, CommandsAggregatorFormer };

  ///
  /// Form CA commands grammar.
  ///

  pub fn ca() -> CommandsAggregatorFormer
  {
    CommandsAggregator::former()

    .command( "publish" )
      .hint( "Publish package on `crates.io`." )
      .long_hint( "Publish package on `crates.io`." )
      .subject()
        .hint( "A path to package. Should be a directory with file `Cargo.toml`." )
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( true )
        .end()
      .property( "dry" )
        .hint( "Run command dry. Default is false." )
        .kind( Type::String )
        .optional( true )
        .end()
      .property( "verbosity" )
        .hint( "Setup level of verbosity." )
        .kind( Type::String )
        .optional( true )
        .alias( "v" )
        .end()
      .routine( commands::publish::publish )
      .end()

    .command( "workspace.publish" )
      .hint( "Publish packages from workspace on `crates.io`." )
      .long_hint( "Publish packages from workspace on `crates.io`." )
      .subject()
        .hint( "A path to manifest path with workspace. Should be a directory with file `Cargo.toml`." )
        .kind( Type::String )
        .optional( true )
        .end()
      .property( "dry" )
        .hint( "Run command dry. Default is false." )
        .kind( Type::String )
        .optional( true )
        .end()
      .property( "verbosity" )
        .hint( "Setup level of verbosity." )
        .kind( Type::String )
        .optional( true )
        .alias( "v" )
        .end()
      .routine( commands::publish::workspace_publish )
      .end()

    .command( "list" )
      .hint( "List packages." )
      .long_hint( "List packages" )
      .subject()
        .hint( "A path to directory with packages. Should be a glob." )
        .kind( Type::List( Type::String.into(), ',' ) )
        .optional( true )
        .end()
      .routine( commands::list::list )
      .end()
  }
}
//

crate::mod_interface!
{
  prelude use ca;
}

