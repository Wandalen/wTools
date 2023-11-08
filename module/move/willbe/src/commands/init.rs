
/// Internal namespace.
pub( crate ) mod private
{
  use std::collections::HashMap;
  use wca::{ Type, Routine };

  ///
  /// Form CA commands grammar.
  ///

  pub fn grammar_form() -> Vec< wca::Command >
  {
    let publish_no_subj_command = wca::Command::former()
    .hint( "Publish package on `crates.io`." )
    .long_hint( "Publish package on `crates.io`." )
    .phrase( "publish" )
    .property( "dry", "Run command dry. Default is false.", Type::String, true )
    .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    .property_alias( "verbosity", "v" )
    .form();

    let publish_command = wca::Command::former()
    .hint( "Publish package on `crates.io`." )
    .long_hint( "Publish package on `crates.io`." )
    .phrase( "publish" )
    .subject( "A path to package. Should be a directory with file `Cargo.toml`.", Type::List( Type::String.into(), ',' ), true )
    .property( "dry", "Run command dry. Default is false.", Type::String, true )
    .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    .property_alias( "verbosity", "v" )
    .form();

    let workspace_publish_no_subj_command = wca::Command::former()
    .hint( "Publish packages from workspace on `crates.io`." )
    .long_hint( "Publish packages from workspace on `crates.io`." )
    .phrase( "workspace.publish" )
    .property( "dry", "Run command dry. Default is false.", Type::String, true )
    .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    .property_alias( "verbosity", "v" )
    .form();

    let workspace_publish_command = wca::Command::former()
    .hint( "Publish packages from workspace on `crates.io`." )
    .long_hint( "Publish packages from workspace on `crates.io`." )
    .phrase( "workspace.publish" )
    .subject( "A path to manifest path with workspace. Should be a directory with file `Cargo.toml`.", Type::String, true )
    .property( "dry", "Run command dry. Default is false.", Type::String, true )
    .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    .property_alias( "verbosity", "v" )
    .form();

    let list_no_subj_command = wca::Command::former()
    .hint( "List packages." )
    .long_hint( "List packages" )
    .phrase( "list" )
    .form();

    let list_command = wca::Command::former()
    .hint( "List packages." )
    .long_hint( "List packages" )
    .phrase( "list" )
    .subject( "A path to directory with packages. Should be a glob.", Type::List( Type::String.into(), ',' ), true )
    .form();

    let workspace_list_no_subj_command = wca::Command::former()
    .hint( "List workspace packages." )
    .long_hint( "List workspace packages" )
    .phrase( "workspace.list" )
    .property( "type", "Output type. It can be topological sorted list of crates or list + set of independent crates trees.\n               Variants: topsort, tree. Default is \"tree\".", Type::String, true )
    .property( "root_module", "Log dependency tree for selected module. Works in combination with option 'type:tree'", Type::String, true )
    .form();

    let workspace_list_command = wca::Command::former()
    .hint( "List workspace packages." )
    .long_hint( "List workspace packages" )
    .phrase( "workspace.list" )
    .subject( "A path to directory with workspace config. Should be a glob.", Type::List( Type::String.into(), ',' ), true )
    .property( "type", "Output type. It can be topological sorted list of crates or list + set of independent crates trees.\n               Variants: topsort, tree. Default is \"tree\".", Type::String, true )
    .property( "root_module", "Log dependency tree for selected module. Works in combination with option 'type:tree'", Type::String, true )
    .form();

    let create_table_command = wca::Command::former()
    .hint( "Generate table for main Readme.md file" )
    .long_hint( "Generate table for main Readme.md file" )
    .phrase( "readme.health.table.generate" )
    .form();

    vec!
    [
      publish_no_subj_command, publish_command,
      workspace_publish_no_subj_command, workspace_publish_command,
      list_no_subj_command, list_command,
      workspace_list_no_subj_command, workspace_list_command,
      create_table_command,
    ]
  }

  ///
  /// Form CA commands executor.
  ///

  pub fn executor_form() -> HashMap< String, Routine >
  {
    use crate::command::*;

    HashMap::from
    ([
      ( "publish".to_owned(), Routine::new( publish ) ),
      ( "workspace.publish".to_owned(), Routine::new( workspace_publish ) ),
      ( "list".to_owned(), Routine::new( list ) ),
      ( "workspace.list".to_owned(), Routine::new( workspace_list ) ),
      ( "readme.health.table.generate".to_owned(), Routine::new( table_generate ) ),
    ])
  }
}
//

crate::mod_interface!
{
  prelude use grammar_form;
  prelude use executor_form;
}

