
/// Internal namespace.
pub( crate ) mod private
{
  use wca::{ Type, Routine };

  ///
  /// Form CA commands grammar.
  ///

  pub fn grammar_form() -> Vec< wca::Command >
  {
    // let publish_no_subj_command = wca::Command::former()
    // .hint( "Publish package on `crates.io`." )
    // .long_hint( "Publish package on `crates.io`." )
    // .phrase( "publish" )
    // .property( "dry", "Run command dry. Default is false.", Type::String, true )
    // .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    // .property_alias( "verbosity", "v" )
    // .form();
    //
    // let publish_command = wca::Command::former()
    // .hint( "Publish package on `crates.io`." )
    // .long_hint( "Publish package on `crates.io`." )
    // .phrase( "publish" )
    // .subject( "A path to package. Should be a directory with file `Cargo.toml`.", Type::List( Type::String.into(), ',' ), true )
    // .property( "dry", "Run command dry. Default is false.", Type::String, true )
    // .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    // .property_alias( "verbosity", "v" )
    // .form();
    //
    // let workspace_publish_no_subj_command = wca::Command::former()
    // .hint( "Publish packages from workspace on `crates.io`." )
    // .long_hint( "Publish packages from workspace on `crates.io`." )
    // .phrase( "workspace.publish" )
    // .property( "dry", "Run command dry. Default is false.", Type::String, true )
    // .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    // .property_alias( "verbosity", "v" )
    // .form();
    //
    // let workspace_publish_command = wca::Command::former()
    // .hint( "Publish packages from workspace on `crates.io`." )
    // .long_hint( "Publish packages from workspace on `crates.io`." )
    // .phrase( "workspace.publish" )
    // .subject( "A path to manifest path with workspace. Should be a directory with file `Cargo.toml`.", Type::String, true )
    // .property( "dry", "Run command dry. Default is false.", Type::String, true )
    // .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    // .property_alias( "verbosity", "v" )
    // .form();
    //
    // let list_no_subj_command = wca::Command::former()
    // .hint( "List packages." )
    // .long_hint( "List packages" )
    // .phrase( "list" )
    // .form();
    //
    // let list_command = wca::Command::former()
    // .hint( "List packages." )
    // .long_hint( "List packages" )
    // .phrase( "list" )
    // .subject( "A path to directory with packages. Should be a glob.", Type::List( Type::String.into(), ',' ), true )
    // .form();

    let workspace_list_no_subj_command = wca::Command::former()
    .hint( "List workspace packages." )
    .long_hint( "List workspace packages" )
    .phrase( "workspace.list" )
    .form();

    let workspace_list_command = wca::Command::former()
    .hint( "List workspace packages." )
    .long_hint( "List workspace packages" )
    .phrase( "workspace.list" )
    .subject( "A path to directory with workspace config. Should be a glob.", Type::List( Type::String.into(), ',' ), true )
    .form();

    vec!
    [
      // publish_no_subj_command, publish_command,
      // workspace_publish_no_subj_command, workspace_publish_command,
      // list_no_subj_command, list_command,
      workspace_list_no_subj_command, workspace_list_command,
    ]
  }

  ///
  /// Form CA commands executor.
  ///

  pub fn executor_form() -> wtools::HashMap< String, Routine >
  {
    wtools::HashMap::from
    ([
      ( "publish".to_owned(), Routine::new( crate::commands::publish::publish ) ),
      ( "workspace.publish".to_owned(), Routine::new( crate::commands::publish::workspace_publish ) ),
      ( "list".to_owned(), Routine::new( crate::commands::list::list ) ),
      ( "workspace.list".to_owned(), Routine::new( crate::commands::list::workspace_list ) ),
    ])
  }
}
//

crate::mod_interface!
{
  prelude use grammar_form;
  prelude use executor_form;
}

