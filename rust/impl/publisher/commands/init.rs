
/// Internal namespace.
pub( crate ) mod private
{
  ///
  /// Form CA commands.
  ///

  pub fn commands_form() -> std::collections::HashMap< String, wca::command::Command >
  {
    let publish_command = wca::CommandOptions::default()
    .hint( "Publish package on `crates.io`." )
    .long_hint( "Publish package on `crates.io`." )
    .phrase( "publish" )
    .subject_hint( "A path to package. Should be a directory with file `Cargo.toml`." )
    .property_hint( "dry", "Run command dry. Default is false." )
    .property_hint( "verbosity", "Setup level of verbosity." )
    .property_alias( "verbosity", "v" )
    .routine( &crate::commands::publish::publish )
    .form();

    let workspace_publish_command = wca::CommandOptions::default()
    .hint( "Publish packages from workspace on `crates.io`." )
    .long_hint( "Publish packages from workspace on `crates.io`." )
    .phrase( "workspace.publish" )
    .subject_hint( "A path to manifest path with workspace. Should be a directory with file `Cargo.toml`." )
    .property_hint( "dry", "Run command dry. Default is false." )
    .property_hint( "verbosity", "Setup level of verbosity." )
    .property_alias( "verbosity", "v" )
    .routine( &crate::commands::publish::workspace_publish )
    .form();

    let list_command = wca::CommandOptions::default()
    .hint( "List packages." )
    .long_hint( "List packages" )
    .phrase( "list" )
    .subject_hint( "A path to directory with packages. Should be a glob." )
    .routine( &crate::commands::list::list )
    .form();

    let ca_map = std::collections::HashMap::from
    ([
      ( ".publish".to_string(), publish_command ),
      ( ".workspace.publish".to_string(), workspace_publish_command ),
      ( ".list".to_string(), list_command ),
    ]);

    ca_map
  }
}
//

crate::mod_interface!
{
  prelude use commands_form;
}

